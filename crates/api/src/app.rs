use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

// STEP-000/001/003/005 live in this file as a teaching baseline.
// The goal is to keep the first learning steps small, readable, and explicit.
#[derive(Serialize)]
struct HealthResponse {
    service: &'static str,
    status: &'static str,
}

// In-memory state is enough for the first learning steps.
// Later steps can swap this out for a real persistence layer.
#[derive(Clone)]
struct AppState {
    // We store users in memory so the early steps can focus on route logic,
    // validation, and learning the flow without database noise.
    users_by_email: Arc<RwLock<HashMap<String, UserRecord>>>,
    // Refresh tokens are tracked separately so the session rotation story
    // stays visible without adding database complexity too early.
    refresh_tokens: Arc<RwLock<HashMap<String, u64>>>,
    next_user_id: Arc<RwLock<u64>>,
    next_refresh_id: Arc<RwLock<u64>>,
}

// Minimal user record for the teaching branch.
// We only keep the fields needed to demonstrate signup/login flow.
#[derive(Clone)]
#[allow(dead_code)]
struct UserRecord {
    id: u64,
    email: String,
    // This is a placeholder transformation, not production hashing.
    // The point here is to teach the flow while avoiding raw password storage.
    password_hash: String,
}

// Signup request used by the learning route.
// Keep the payload small so the validation rules are easy to read.
#[derive(Debug, Deserialize)]
struct SignupRequest {
    email: String,
    password: String,
}

// Login request mirrors the signup shape for the first learning loop.
#[derive(Debug, Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

// Refresh requests stay explicit in the learning branch so the token flow is
// easy to read and easy to test.
#[derive(Debug, Deserialize)]
struct RefreshRequest {
    refresh_token: String,
}

// Successful signup response keeps only the essential teaching fields.
#[derive(Debug, Serialize)]
struct SignupResponse {
    user_id: u64,
    email: String,
}

// Login response is intentionally lightweight for the early branch.
#[derive(Debug, Serialize)]
struct LoginResponse {
    user_id: u64,
    email: String,
    refresh_token: String,
}

// Refresh returns the rotated token so the tests can prove one-time usage.
#[derive(Debug, Serialize)]
struct RefreshResponse {
    user_id: u64,
    email: String,
    refresh_token: String,
}

// Stable error envelope so the tests can assert machine-readable outcomes.
#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: &'static str,
}

// STEP-000 starts with a health endpoint only.
// The state is attached here so later route steps can share the same baseline.
pub async fn build_router() -> anyhow::Result<Router> {
    let state = AppState {
        users_by_email: Arc::new(RwLock::new(HashMap::new())),
        refresh_tokens: Arc::new(RwLock::new(HashMap::new())),
        next_user_id: Arc::new(RwLock::new(1)),
        next_refresh_id: Arc::new(RwLock::new(1)),
    };

    Ok(
        Router::new()
            .route("/health", get(health))
            .route("/auth/signup", axum::routing::post(signup))
            // STEP-003 keeps login on the same in-memory state so we can
            // validate the authentication flow before introducing tokens/DB.
            .route("/auth/login", axum::routing::post(login))
            // STEP-005 adds refresh so the branch can show a complete session loop.
            .route("/auth/refresh", axum::routing::post(refresh))
            .with_state(state),
    )
}

// Health exists first because every later learning step depends on a quick
// signal that the binary boots and the router is wired correctly.
async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        service: "rev0auth-api",
        status: "ok",
    })
}

// Signup is the first real behavior step.
// It demonstrates validation, duplicate detection, and response shaping.
async fn signup(
    State(state): State<AppState>,
    Json(payload): Json<SignupRequest>,
) -> Result<Json<SignupResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Normalize early so all checks use the same canonical email.
    let normalized_email = normalize_email(&payload.email);

    // Bad email input is rejected immediately so the rest of the route stays clean.
    if !is_valid_email(&normalized_email) {
        return Err(err(StatusCode::BAD_REQUEST, "invalid_email"));
    }

    // A small password rule keeps the learning branch honest without pulling
    // in the full production password pipeline too early.
    if payload.password.len() < 12 {
        return Err(err(StatusCode::BAD_REQUEST, "weak_password"));
    }

    // Duplicate detection happens before the write lock so we explain the flow
    // clearly: read, check, then write only if the email is new.
    {
        let users = state.users_by_email.read().await;
        if users.contains_key(&normalized_email) {
            return Err(err(StatusCode::CONFLICT, "email_already_exists"));
        }
    }

    // User IDs are generated locally in the in-memory branch to keep the step
    // deterministic and easy to test.
    let user_id = {
        let mut next = state.next_user_id.write().await;
        let id = *next;
        *next += 1;
        id
    };

    // We keep a simple transformed password value so the code does not store
    // the raw password, while still staying lightweight for the learning phase.
    let record = UserRecord {
        id: user_id,
        email: normalized_email.clone(),
        password_hash: pseudo_hash(&payload.password),
    };

    // The write lock is held only for the insertion itself.
    let mut users = state.users_by_email.write().await;
    users.insert(normalized_email.clone(), record);

    Ok(Json(SignupResponse {
        user_id,
        email: normalized_email,
    }))
}

// Login reuses the same in-memory state so the first auth loop is visible and
// the step-by-step history stays easy to follow.
async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Canonical email lookup keeps signup/login behavior consistent.
    let normalized_email = normalize_email(&payload.email);

    // We use a shared read lock because login is read-only.
    let users = state.users_by_email.read().await;
    let record = match users.get(&normalized_email) {
        Some(record) => record,
        None => {
            // Missing user and wrong password intentionally share the same
            // response to avoid leaking whether an email exists.
            return Err(err(StatusCode::UNAUTHORIZED, "invalid_credentials"));
        }
    };

    // Password comparison stays simple in the learning branch so the route
    // behavior is obvious during TDD.
    if record.password_hash != pseudo_hash(&payload.password) {
        return Err(err(StatusCode::UNAUTHORIZED, "invalid_credentials"));
    }

    // Login now issues a refresh token so the next step can demonstrate token
    // rotation and one-time usage.
    let refresh_token = issue_refresh_token(&state, record.id).await;

    // The response mirrors signup: just enough data to prove the flow works.
    Ok(Json(LoginResponse {
        user_id: record.id,
        email: record.email.clone(),
        refresh_token,
    }))
}

// Refresh rotates the session token and returns a new one.
// This is the first visible token lifecycle in the learning branch.
async fn refresh(
    State(state): State<AppState>,
    Json(payload): Json<RefreshRequest>,
) -> Result<Json<RefreshResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Consuming the token first makes the rotation rule explicit.
    let user_id = {
        let mut tokens = state.refresh_tokens.write().await;
        match tokens.remove(&payload.refresh_token) {
            Some(user_id) => user_id,
            None => return Err(err(StatusCode::UNAUTHORIZED, "invalid_refresh_token")),
        }
    };

    let users = state.users_by_email.read().await;
    let user = match users.values().find(|candidate| candidate.id == user_id) {
        Some(user) => user,
        None => return Err(err(StatusCode::UNAUTHORIZED, "invalid_refresh_token")),
    };

    // A new token is issued only after the previous one has been consumed.
    let next_refresh_token = issue_refresh_token(&state, user_id).await;

    Ok(Json(RefreshResponse {
        user_id: user.id,
        email: user.email.clone(),
        refresh_token: next_refresh_token,
    }))
}

// Shared helpers stay tiny and readable in the early steps.
fn normalize_email(email: &str) -> String {
    email.trim().to_ascii_lowercase()
}

// Basic validation is enough for the learning branch.
// Production validation comes later when the codebase has more structure.
fn is_valid_email(email: &str) -> bool {
    email.contains('@') && email.len() >= 5
}

// Placeholder transformation used only for the step-by-step learning branch.
// This will be replaced by the real password hashing pipeline later.
fn pseudo_hash(password: &str) -> String {
    format!("step001::{}", password)
}

// The refresh token generator stays tiny and deterministic enough for tests.
// It exists only to teach the lifecycle of issued and consumed tokens.
async fn issue_refresh_token(state: &AppState, user_id: u64) -> String {
    let token_id = {
        let mut next = state.next_refresh_id.write().await;
        let id = *next;
        *next += 1;
        id
    };

    let token = format!("refresh-{}-{}", user_id, token_id);
    let mut tokens = state.refresh_tokens.write().await;
    tokens.insert(token.clone(), user_id);
    token
}

// Stable error helper keeps route code focused on the happy path.
fn err(status: StatusCode, code: &'static str) -> (StatusCode, Json<ErrorResponse>) {
    (status, Json(ErrorResponse { error: code }))
}

// Tests are written next to the minimal implementation so the learning branch
// reads like a self-contained story.
#[cfg(test)]
mod tests {
    use super::build_router;
    use axum::{body::Body, http::Request};
    use serde_json::Value;
    use tower::ServiceExt;

    // Shared helper for signup route tests.
    async fn post_signup(app: axum::Router, body: serde_json::Value) -> (u16, Value) {
        let response = app
            .oneshot(
                Request::post("/auth/signup")
                    .header("content-type", "application/json")
                    .body(Body::from(body.to_string()))
                    .expect("request"),
            )
            .await
            .expect("response");

        let status = response.status().as_u16();
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("bytes");
        let body = serde_json::from_slice::<Value>(&bytes).expect("json");
        (status, body)
    }

    // Shared helper for login route tests keeps the assertions short.
    async fn post_login(app: axum::Router, body: serde_json::Value) -> (u16, Value) {
        let response = app
            .oneshot(
                Request::post("/auth/login")
                    .header("content-type", "application/json")
                    .body(Body::from(body.to_string()))
                    .expect("request"),
            )
            .await
            .expect("response");

        let status = response.status().as_u16();
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("bytes");
        let body = serde_json::from_slice::<Value>(&bytes).expect("json");
        (status, body)
    }

    // Shared helper for refresh tests keeps token rotation checks focused.
    async fn post_refresh(app: axum::Router, body: serde_json::Value) -> (u16, Value) {
        let response = app
            .oneshot(
                Request::post("/auth/refresh")
                    .header("content-type", "application/json")
                    .body(Body::from(body.to_string()))
                    .expect("request"),
            )
            .await
            .expect("response");

        let status = response.status().as_u16();
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("bytes");
        let body = serde_json::from_slice::<Value>(&bytes).expect("json");
        (status, body)
    }

    // The health test is the first proof that the router boots correctly.
    #[tokio::test]
    async fn test_health_endpoint() {
        let app = build_router().await.expect("build router");

        let response = app
            .oneshot(
                Request::get("/health")
                    .body(Body::empty())
                    .expect("request"),
            )
            .await
            .expect("response");

        assert_eq!(response.status().as_u16(), 200);

        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("bytes");
        let body = serde_json::from_slice::<Value>(&bytes).expect("json");

        assert_eq!(body["service"], "rev0auth-api");
        assert_eq!(body["status"], "ok");
    }

    // Signup success is the main happy-path proof for the route.
    #[tokio::test]
    async fn test_signup_success_returns_user_payload() {
        let app = build_router().await.expect("build router");

        let (status, body) = post_signup(
            app,
            serde_json::json!({
                "email": "member@example.com",
                "password": "my-strong-password-123"
            }),
        )
        .await;

        assert_eq!(status, 200);
        assert_eq!(body["email"], "member@example.com");
        assert_eq!(body["user_id"], 1);
    }

    // Invalid emails should fail fast before any state mutation happens.
    #[tokio::test]
    async fn test_signup_rejects_invalid_email() {
        let app = build_router().await.expect("build router");

        let (status, body) = post_signup(
            app,
            serde_json::json!({
                "email": "invalid-email",
                "password": "my-strong-password-123"
            }),
        )
        .await;

        assert_eq!(status, 400);
        assert_eq!(body["error"], "invalid_email");
    }

    // Weak passwords are rejected so the step already encodes a real rule.
    #[tokio::test]
    async fn test_signup_rejects_weak_password() {
        let app = build_router().await.expect("build router");

        let (status, body) = post_signup(
            app,
            serde_json::json!({
                "email": "member@example.com",
                "password": "short"
            }),
        )
        .await;

        assert_eq!(status, 400);
        assert_eq!(body["error"], "weak_password");
    }

    // Duplicate email protection is one of the most important early auth rules.
    #[tokio::test]
    async fn test_signup_duplicate_email_returns_conflict() {
        let app = build_router().await.expect("build router");

        let payload = serde_json::json!({
            "email": "dup@example.com",
            "password": "my-strong-password-123"
        });

        let (first_status, _) = post_signup(app.clone(), payload.clone()).await;
        assert_eq!(first_status, 200);

        let (second_status, body) = post_signup(app, payload).await;

        assert_eq!(second_status, 409);
        assert_eq!(body["error"], "email_already_exists");
    }

    // Login success now also proves that the refresh token has been issued.
    #[tokio::test]
    async fn test_login_success_returns_user_payload() {
        let app = build_router().await.expect("build router");

        let _ = post_signup(
            app.clone(),
            serde_json::json!({
                "email": "login@example.com",
                "password": "my-strong-password-123"
            }),
        )
        .await;

        let (status, body) = post_login(
            app,
            serde_json::json!({
                "email": "login@example.com",
                "password": "my-strong-password-123"
            }),
        )
        .await;

        assert_eq!(status, 200);
        assert_eq!(body["email"], "login@example.com");
        assert_eq!(body["user_id"], 1);
        assert!(body["refresh_token"].is_string());
    }

    // Refresh rotates the token and returns a new one.
    #[tokio::test]
    async fn test_refresh_rotates_token() {
        let app = build_router().await.expect("build router");

        let _ = post_signup(
            app.clone(),
            serde_json::json!({
                "email": "rotate@example.com",
                "password": "my-strong-password-123"
            }),
        )
        .await;

        let (_, login_body) = post_login(
            app.clone(),
            serde_json::json!({
                "email": "rotate@example.com",
                "password": "my-strong-password-123"
            }),
        )
        .await;

        let original_token = login_body["refresh_token"].as_str().unwrap_or_default().to_string();

        let (status, body) = post_refresh(
            app.clone(),
            serde_json::json!({
                "refresh_token": original_token
            }),
        )
        .await;

        assert_eq!(status, 200);
        assert_eq!(body["email"], "rotate@example.com");
        assert!(body["refresh_token"].is_string());
        assert_ne!(body["refresh_token"].as_str().unwrap_or_default(), original_token);

        // A rotated token cannot be reused; this proves one-time consumption.
        let (reuse_status, reuse_body) = post_refresh(
            app,
            serde_json::json!({
                "refresh_token": original_token
            }),
        )
        .await;

        assert_eq!(reuse_status, 401);
        assert_eq!(reuse_body["error"], "invalid_refresh_token");
    }

    // Unknown refresh tokens are rejected without touching state.
    #[tokio::test]
    async fn test_refresh_rejects_unknown_token() {
        let app = build_router().await.expect("build router");

        let (status, body) = post_refresh(
            app,
            serde_json::json!({
                "refresh_token": "missing-token"
            }),
        )
        .await;

        assert_eq!(status, 401);
        assert_eq!(body["error"], "invalid_refresh_token");
    }

    // Unknown users should not leak any extra information.
    #[tokio::test]
    async fn test_login_rejects_unknown_user() {
        let app = build_router().await.expect("build router");

        let (status, body) = post_login(
            app,
            serde_json::json!({
                "email": "missing@example.com",
                "password": "my-strong-password-123"
            }),
        )
        .await;

        assert_eq!(status, 401);
        assert_eq!(body["error"], "invalid_credentials");
    }

    // Wrong passwords use the same error code as unknown users on purpose.
    #[tokio::test]
    async fn test_login_rejects_wrong_password() {
        let app = build_router().await.expect("build router");

        let _ = post_signup(
            app.clone(),
            serde_json::json!({
                "email": "wrongpass@example.com",
                "password": "my-strong-password-123"
            }),
        )
        .await;

        let (status, body) = post_login(
            app,
            serde_json::json!({
                "email": "wrongpass@example.com",
                "password": "wrong-password-123"
            }),
        )
        .await;

        assert_eq!(status, 401);
        assert_eq!(body["error"], "invalid_credentials");
    }
}
