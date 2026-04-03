use crate::auth::{
    audit::{new_event, AuditEvent, AuditEventType},
    jwt::{now_epoch, TokenService},
    migrations::run_pending_migrations,
    models::{RefreshSession, Role, User},
    rate_limit::LoginRateLimiter,
};
use chrono::{DateTime, Utc};
use rand::{distributions::Alphanumeric, Rng};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{collections::HashMap, sync::Arc};
use std::time::Instant;
use tokio::sync::RwLock;
use tracing::info;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct DbHealth {
    pub status: &'static str,
    pub backend: &'static str,
    pub latency_ms: u64,
}

#[derive(Clone)]
pub struct AppState {
    backend: AuthBackend,
    pub token_service: TokenService,
    pub login_rate_limiter: LoginRateLimiter,
}

#[derive(Clone)]
enum AuthBackend {
    Memory(MemoryStore),
    Postgres(PgPool),
}

#[derive(Clone)]
struct MemoryStore {
    users_by_email: Arc<RwLock<HashMap<String, User>>>,
    refresh_tokens: Arc<RwLock<HashMap<String, RefreshSession>>>,
    audit_events: Arc<RwLock<Vec<AuditEvent>>>,
}

impl AppState {
    pub fn new_in_memory(token_service: TokenService) -> Self {
        Self {
            backend: AuthBackend::Memory(MemoryStore {
                users_by_email: Arc::new(RwLock::new(HashMap::new())),
                refresh_tokens: Arc::new(RwLock::new(HashMap::new())),
                audit_events: Arc::new(RwLock::new(Vec::new())),
            }),
            token_service,
            login_rate_limiter: LoginRateLimiter::new(),
        }
    }

    pub async fn from_env(token_service: TokenService) -> anyhow::Result<Self> {
        match std::env::var("DATABASE_URL") {
            Ok(database_url) if !database_url.trim().is_empty() => {
                let pool = PgPoolOptions::new()
                    .max_connections(10)
                    .connect(&database_url)
                    .await?;
                run_pending_migrations(&pool).await?;
                info!("Auth backend: PostgreSQL");
                Ok(Self {
                    backend: AuthBackend::Postgres(pool),
                    token_service,
                    login_rate_limiter: LoginRateLimiter::new(),
                })
            }
            _ => {
                info!("Auth backend: in-memory (DATABASE_URL not set)");
                Ok(Self::new_in_memory(token_service))
            }
        }
    }

    pub async fn create_user(
        &self,
        email: String,
        password_hash: String,
        role: Role,
    ) -> Result<User, &'static str> {
        match &self.backend {
            AuthBackend::Memory(memory) => {
                let mut users = memory.users_by_email.write().await;
                let normalized = normalize_email(&email);
                if users.contains_key(&normalized) {
                    return Err("email_already_exists");
                }

                let user = User {
                    id: Uuid::new_v4(),
                    email: normalized.clone(),
                    password_hash,
                    role,
                };
                users.insert(normalized, user.clone());
                Ok(user)
            }
            AuthBackend::Postgres(pool) => {
                let normalized = normalize_email(&email);
                let role_str = role_to_str(&role);
                let record = sqlx::query_as::<_, (Uuid, String, String, String)>(
                    r#"
                    INSERT INTO auth_users (id, email, password_hash, role)
                    VALUES ($1, $2, $3, $4)
                    RETURNING id, email, password_hash, role
                    "#,
                )
                .bind(Uuid::new_v4())
                .bind(&normalized)
                .bind(password_hash)
                .bind(role_str)
                .fetch_one(pool)
                .await;

                match record {
                    Ok((id, db_email, db_hash, db_role)) => Ok(User {
                        id,
                        email: db_email,
                        password_hash: db_hash,
                        role: str_to_role(&db_role),
                    }),
                    Err(err) => {
                        if is_unique_violation(&err) {
                            Err("email_already_exists")
                        } else {
                            Err("user_creation_failed")
                        }
                    }
                }
            }
        }
    }

    pub async fn find_user_by_email(&self, email: &str) -> Option<User> {
        match &self.backend {
            AuthBackend::Memory(memory) => {
                let users = memory.users_by_email.read().await;
                users.get(&normalize_email(email)).cloned()
            }
            AuthBackend::Postgres(pool) => {
                let normalized = normalize_email(email);
                let record = sqlx::query_as::<_, (Uuid, String, String, String)>(
                    r#"
                    SELECT id, email, password_hash, role
                    FROM auth_users
                    WHERE email = $1
                    "#,
                )
                .bind(normalized)
                .fetch_optional(pool)
                .await
                .ok()??;

                Some(User {
                    id: record.0,
                    email: record.1,
                    password_hash: record.2,
                    role: str_to_role(&record.3),
                })
            }
        }
    }

    pub async fn issue_refresh_for_user(&self, user: &User, csrf_token: &str) -> String {
        match &self.backend {
            AuthBackend::Memory(memory) => {
                let token = generate_refresh_token();
                let expires_at_epoch = now_epoch() + self.token_service.refresh_ttl_secs();
                let session = RefreshSession {
                    user_id: user.id,
                    email: user.email.clone(),
                    role: user.role.clone(),
                    csrf_token: csrf_token.to_string(),
                    expires_at_epoch,
                };

                let mut refresh_tokens = memory.refresh_tokens.write().await;
                refresh_tokens.insert(token.clone(), session);
                token
            }
            AuthBackend::Postgres(pool) => {
                let token = generate_refresh_token();
                let expires_at = epoch_to_utc(now_epoch() + self.token_service.refresh_ttl_secs());

                let insert_result = sqlx::query(
                    r#"
                    INSERT INTO auth_refresh_tokens (token, user_id, csrf_token, expires_at)
                    VALUES ($1, $2, $3, $4)
                    "#,
                )
                .bind(&token)
                .bind(user.id)
                .bind(csrf_token)
                .bind(expires_at)
                .execute(pool)
                .await;

                if insert_result.is_err() {
                    return generate_refresh_token();
                }
                token
            }
        }
    }

    pub async fn rotate_refresh_token(
        &self,
        current_token: &str,
        csrf_token: &str,
    ) -> Result<(User, String), &'static str> {
        match &self.backend {
            AuthBackend::Memory(memory) => {
                let mut refresh_tokens = memory.refresh_tokens.write().await;
                let session = match refresh_tokens.remove(current_token) {
                    Some(session) => session,
                    None => return Err("invalid_refresh_token"),
                };

                if session.csrf_token != csrf_token {
                    return Err("invalid_csrf_token");
                }

                if session.expires_at_epoch < now_epoch() {
                    return Err("expired_refresh_token");
                }

                let user = User {
                    id: session.user_id,
                    email: session.email,
                    password_hash: String::new(),
                    role: session.role,
                };

                let next_token = generate_refresh_token();
                let next_session = RefreshSession {
                    user_id: user.id,
                    email: user.email.clone(),
                    role: user.role.clone(),
                    csrf_token: csrf_token.to_string(),
                    expires_at_epoch: now_epoch() + self.token_service.refresh_ttl_secs(),
                };

                refresh_tokens.insert(next_token.clone(), next_session);
                Ok((user, next_token))
            }
            AuthBackend::Postgres(pool) => {
                // Dev note: CSRF binding for postgres refresh sessions is deferred until schema upgrade.
                let mut tx = pool
                    .begin()
                    .await
                    .map_err(|_| "refresh_rotation_failed")?;

                let removed = sqlx::query_as::<_, (Uuid, String, DateTime<Utc>)>(
                    r#"
                    DELETE FROM auth_refresh_tokens
                    WHERE token = $1
                    RETURNING user_id, csrf_token, expires_at
                    "#,
                )
                .bind(current_token)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|_| "refresh_rotation_failed")?;

                let (user_id, stored_csrf_token, expires_at) = match removed {
                    Some(row) => row,
                    None => return Err("invalid_refresh_token"),
                };

                if stored_csrf_token != csrf_token {
                    return Err("invalid_csrf_token");
                }

                if expires_at.timestamp() as u64 <= now_epoch() {
                    return Err("expired_refresh_token");
                }

                let user_record = sqlx::query_as::<_, (Uuid, String, String, String)>(
                    r#"
                    SELECT id, email, password_hash, role
                    FROM auth_users
                    WHERE id = $1
                    "#,
                )
                .bind(user_id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|_| "refresh_rotation_failed")?;

                let (id, email, password_hash, role) =
                    user_record.ok_or("invalid_refresh_token")?;
                let user = User {
                    id,
                    email,
                    password_hash,
                    role: str_to_role(&role),
                };

                let next_token = generate_refresh_token();
                let next_expiry = epoch_to_utc(now_epoch() + self.token_service.refresh_ttl_secs());
                sqlx::query(
                    r#"
                    INSERT INTO auth_refresh_tokens (token, user_id, csrf_token, expires_at)
                    VALUES ($1, $2, $3, $4)
                    "#,
                )
                .bind(&next_token)
                .bind(user.id)
                .bind(csrf_token)
                .bind(next_expiry)
                .execute(&mut *tx)
                .await
                .map_err(|_| "refresh_rotation_failed")?;

                tx.commit().await.map_err(|_| "refresh_rotation_failed")?;
                Ok((user, next_token))
            }
        }
    }

    // Dev note: central audit write path for auth events.
    // Attached to: auth handlers and admin audit review endpoint.
    pub async fn record_audit_event(
        &self,
        user_id: Option<Uuid>,
        event_type: AuditEventType,
        ip: String,
    ) {
        let event = new_event(user_id, event_type, ip);

        match &self.backend {
            AuthBackend::Memory(memory) => {
                let mut events = memory.audit_events.write().await;
                events.push(event.clone());
            }
            AuthBackend::Postgres(_) => {}
        }

        info!(
            target: "rev0auth.audit",
            event_type = ?event.event_type,
            user_id = ?event.user_id,
            ip = %event.ip,
            timestamp = event.timestamp,
            "Auth audit event"
        );
    }

    pub async fn list_audit_events(&self) -> Vec<AuditEvent> {
        match &self.backend {
            AuthBackend::Memory(memory) => {
                let events = memory.audit_events.read().await;
                events.iter().cloned().rev().collect()
            }
            AuthBackend::Postgres(_) => Vec::new(),
        }
    }

    // Dev note: database readiness probe used by /health/db for deploy checks.
    // Attached to: container orchestration health probes and runtime diagnostics.
    pub async fn db_health(&self) -> DbHealth {
        match &self.backend {
            AuthBackend::Memory(_) => DbHealth {
                status: "ok",
                backend: "memory",
                latency_ms: 0,
            },
            AuthBackend::Postgres(pool) => {
                let started = Instant::now();
                let probe = sqlx::query_scalar::<_, i64>("SELECT 1")
                    .fetch_one(pool)
                    .await;

                match probe {
                    Ok(_) => DbHealth {
                        status: "ok",
                        backend: "postgres",
                        latency_ms: started.elapsed().as_millis() as u64,
                    },
                    Err(_) => DbHealth {
                        status: "down",
                        backend: "postgres",
                        latency_ms: started.elapsed().as_millis() as u64,
                    },
                }
            }
        }
    }
}

pub fn normalize_email(email: &str) -> String {
    email.trim().to_ascii_lowercase()
}

fn generate_refresh_token() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(64)
        .map(char::from)
        .collect()
}

fn role_to_str(role: &Role) -> &'static str {
    match role {
        Role::Member => "member",
        Role::Admin => "admin",
    }
}

fn str_to_role(role: &str) -> Role {
    match role {
        "admin" => Role::Admin,
        _ => Role::Member,
    }
}

fn epoch_to_utc(epoch: u64) -> DateTime<Utc> {
    DateTime::<Utc>::from_timestamp(epoch as i64, 0).unwrap_or_else(Utc::now)
}

fn is_unique_violation(err: &sqlx::Error) -> bool {
    match err {
        sqlx::Error::Database(db_err) => db_err.code().is_some_and(|code| code == "23505"),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::{normalize_email, AppState};
    use crate::auth::{jwt::TokenService, models::Role};

    fn make_state() -> AppState {
        AppState::new_in_memory(TokenService::from_env())
    }

    #[tokio::test]
    async fn test_create_user_in_memory() {
        let state = make_state();

        let created = state
            .create_user(
                "Member@Example.com".to_string(),
                "hashed-password".to_string(),
                Role::Member,
            )
            .await
            .expect("user should be created");

        let found = state.find_user_by_email("member@example.com").await;
        assert!(found.is_some());

        let found = found.expect("user should exist");
        assert_eq!(created.id, found.id);
        assert_eq!(found.email, "member@example.com");
    }

    #[tokio::test]
    async fn test_create_user_duplicate_email_rejected() {
        let state = make_state();

        let _ = state
            .create_user(
                "member@example.com".to_string(),
                "hash-1".to_string(),
                Role::Member,
            )
            .await
            .expect("first user should be created");

        let second = state
            .create_user(
                "MEMBER@example.com".to_string(),
                "hash-2".to_string(),
                Role::Member,
            )
            .await;

        assert!(matches!(second, Err("email_already_exists")));
    }

    #[tokio::test]
    async fn test_find_user_returns_none_if_missing() {
        let state = make_state();
        let found = state.find_user_by_email("missing@example.com").await;
        assert!(found.is_none());
    }

    #[test]
    fn test_normalize_email_trims_and_lowercases() {
        let normalized = normalize_email("  User@Example.COM  ");
        assert_eq!(normalized, "user@example.com");
    }

    #[tokio::test]
    async fn test_issue_refresh_token_generates_unique() {
        let state = make_state();
        let user = state
            .create_user(
                "member@example.com".to_string(),
                "hashed-password".to_string(),
                Role::Member,
            )
            .await
            .expect("user should be created");

        let first = state.issue_refresh_for_user(&user, "csrf-a").await;
        let second = state.issue_refresh_for_user(&user, "csrf-b").await;

        assert_ne!(first, second);
        assert_eq!(first.len(), 64);
        assert_eq!(second.len(), 64);
        assert!(first.chars().all(|c| c.is_ascii_alphanumeric()));
        assert!(second.chars().all(|c| c.is_ascii_alphanumeric()));
    }

    #[tokio::test]
    async fn test_rotate_refresh_deletes_old_token() {
        let state = make_state();
        let user = state
            .create_user(
                "member@example.com".to_string(),
                "hashed-password".to_string(),
                Role::Member,
            )
            .await
            .expect("user should be created");

        let original = state.issue_refresh_for_user(&user, "csrf-test").await;
        let _ = state
            .rotate_refresh_token(&original, "csrf-test")
            .await
            .expect("rotation should succeed");

        let reused = state.rotate_refresh_token(&original, "csrf-test").await;
        assert!(matches!(reused, Err("invalid_refresh_token")));
    }

    #[tokio::test]
    async fn test_rotate_refresh_issued_new_token() {
        let state = make_state();
        let user = state
            .create_user(
                "member@example.com".to_string(),
                "hashed-password".to_string(),
                Role::Member,
            )
            .await
            .expect("user should be created");

        let original = state.issue_refresh_for_user(&user, "csrf-test").await;
        let (rotated_user, new_token) = state
            .rotate_refresh_token(&original, "csrf-test")
            .await
            .expect("rotation should succeed");

        assert_eq!(rotated_user.id, user.id);
        assert_eq!(rotated_user.email, user.email);
        assert_ne!(new_token, original);

        let second_rotation = state.rotate_refresh_token(&new_token, "csrf-test").await;
        assert!(second_rotation.is_ok());
    }
}
