use axum::{extract::State, http::HeaderMap, http::StatusCode, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

// STEP-000/001/003/005/007 live in this file as a teaching baseline.
// The goal is to keep early steps explicit and easy to audit line by line.
#[derive(Serialize)]
struct HealthResponse {
    service: &'static str,
    status: &'static str,
}

// In-memory state is enough for the first learning stages.
// We can add database-backed stores later without changing endpoint contracts.
#[derive(Clone)]
struct AppState {
    users_by_email: Arc<RwLock<HashMap<String, UserRecord>>>,
    refresh_tokens: Arc<RwLock<HashMap<String, u64>>>,
    // Access tokens back protected read routes in this stage.
    access_tokens: Arc<RwLock<HashMap<String, u64>>>,
    next_user_id: Arc<RwLock<u64>>,
    next_refresh_id: Arc<RwLock<u64>>,
    next_access_id: Arc<RwLock<u64>>,
}

// Minimal user record for the learning branch.
#[derive(Clone)]
#[allow(dead_code)]
struct UserRecord {
    id: u64,
    email: String,
    // Placeholder transformation, not production hashing.
    password_hash: String,
}

#[derive(Debug, Deserialize)]
struct SignupRequest {
    email: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct RefreshRequest {
    refresh_token: String,
}

#[derive(Debug, Serialize)]
struct SignupResponse {
    user_id: u64,
    email: String,
}

#[derive(Debug, Serialize)]
struct LoginResponse {
    user_id: u64,
    email: String,
    refresh_token: String,
    access_token: String,
}

#[derive(Debug, Serialize)]
struct RefreshResponse {
    user_id: u64,
    email: String,
    refresh_token: String,
}

#[derive(Debug, Serialize)]
struct MeResponse {
    user_id: u64,
    email: String,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: &'static str,
}

pub async fn build_router() -> anyhow::Result<Router> {
    let state = AppState {
        users_by_email: Arc::new(RwLock::new(HashMap::new())),
        refresh_tokens: Arc::new(RwLock::new(HashMap::new())),
        access_tokens: Arc::new(RwLock::new(HashMap::new())),
        next_user_id: Arc::new(RwLock::new(1)),
        next_refresh_id: Arc::new(RwLock::new(1)),
        next_access_id: Arc::new(RwLock::new(1)),
    };

    Ok(
        Router::new()
            .route("/health", get(health))
            .route("/auth/signup", axum::routing::post(signup))
            .route("/auth/login", axum::routing::post(login))
            .route("/auth/refresh", axum::routing::post(refresh))
            // STEP-007: first bearer-protected route.
            .route("/auth/me", get(me))
            .with_state(state),
    )
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        service: "rev0auth-api",
        status: "ok",
    })
}

async fn signup(
    State(state): State<AppState>,
    Json(payload): Json<SignupRequest>,
) -> Result<Json<SignupResponse>, (StatusCode, Json<ErrorResponse>)> {
    let normalized_email = normalize_email(&payload.email);

    if !is_valid_email(&normalized_email) {
        return Err(err(StatusCode::BAD_REQUEST, "invalid_email"));
    }

    if payload.password.len() < 12 {
        return Err(err(StatusCode::BAD_REQUEST, "weak_password"));
    }

    {
        let users = state.users_by_email.read().await;
        if users.contains_key(&normalized_email) {
            return Err(err(StatusCode::CONFLICT, "email_already_exists"));
        }
    }

    let user_id = {
        let mut next = state.next_user_id.write().await;
        let id = *next;
        *next += 1;
        id
    };

    let record = UserRecord {
        id: user_id,
        email: normalized_email.clone(),
        password_hash: pseudo_hash(&payload.password),
    };

    let mut users = state.users_by_email.write().await;
    users.insert(normalized_email.clone(), record);

    Ok(Json(SignupResponse {
        user_id,
        email: normalized_email,
    }))
}

async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, Json<ErrorResponse>)> {
    let normalized_email = normalize_email(&payload.email);

    let users = state.users_by_email.read().await;
    let record = match users.get(&normalized_email) {
        Some(record) => record,
        None => return Err(err(StatusCode::UNAUTHORIZED, "invalid_credentials")),
    };

    if record.password_hash != pseudo_hash(&payload.password) {
        return Err(err(StatusCode::UNAUTHORIZED, "invalid_credentials"));
    }

    let refresh_token = issue_refresh_token(&state, record.id).await;
    let access_token = issue_access_token(&state, record.id).await;

    Ok(Json(LoginResponse {
        user_id: record.id,
        email: record.email.clone(),
        refresh_token,
        access_token,
    }))
}

async fn refresh(
    State(state): State<AppState>,
    Json(payload): Json<RefreshRequest>,
) -> Result<Json<RefreshResponse>, (StatusCode, Json<ErrorResponse>)> {
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

    let next_refresh_token = issue_refresh_token(&state, user_id).await;

    Ok(Json(RefreshResponse {
        user_id: user.id,
        email: user.email.clone(),
        refresh_token: next_refresh_token,
    }))
}

async fn me(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<MeResponse>, (StatusCode, Json<ErrorResponse>)> {
    let access_token = extract_bearer_token(&headers)
        .ok_or_else(|| err(StatusCode::UNAUTHORIZED, "missing_bearer_token"))?;

    let user_id = {
        let tokens = state.access_tokens.read().await;
        match tokens.get(&access_token) {
            Some(user_id) => *user_id,
            None => return Err(err(StatusCode::UNAUTHORIZED, "invalid_access_token")),
        }
    };

    let users = state.users_by_email.read().await;
    let user = match users.values().find(|candidate| candidate.id == user_id) {
        Some(user) => user,
        None => return Err(err(StatusCode::UNAUTHORIZED, "invalid_access_token")),
    };

    Ok(Json(MeResponse {
        user_id: user.id,
        email: user.email.clone(),
    }))
}

fn normalize_email(email: &str) -> String {
    email.trim().to_ascii_lowercase()
}

fn is_valid_email(email: &str) -> bool {
    email.contains('@') && email.len() >= 5
}

fn pseudo_hash(password: &str) -> String {
    format!("step001::{}", password)
}

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

async fn issue_access_token(state: &AppState, user_id: u64) -> String {
    let token_id = {
        let mut next = state.next_access_id.write().await;
        let id = *next;
        *next += 1;
        id
    };

    let token = format!("access-{}-{}", user_id, token_id);
    let mut tokens = state.access_tokens.write().await;
    tokens.insert(token.clone(), user_id);
    token
}

fn extract_bearer_token(headers: &HeaderMap) -> Option<String> {
    let raw = headers.get("authorization")?.to_str().ok()?;
    raw.strip_prefix("Bearer ").map(|v| v.to_string())
}

fn err(status: StatusCode, code: &'static str) -> (StatusCode, Json<ErrorResponse>) {
    (status, Json(ErrorResponse { error: code }))
}

#[cfg(test)]
mod tests {
    use super::build_router;
    use axum::{body::Body, http::Request};
    use serde_json::Value;
    use tower::ServiceExt;

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

    async fn get_me(app: axum::Router, bearer: Option<&str>) -> (u16, Value) {
        let mut request = Request::get("/auth/me")
            .body(Body::empty())
            .expect("request");

        if let Some(token) = bearer {
            request.headers_mut().insert(
                "authorization",
                format!("Bearer {token}")
                    .parse()
                    .expect("authorization header"),
            );
        }

        let response = app.oneshot(request).await.expect("response");
        let status = response.status().as_u16();
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("bytes");
        let body = serde_json::from_slice::<Value>(&bytes).expect("json");
        (status, body)
    }

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
        assert!(body["access_token"].is_string());
    }

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

    #[tokio::test]
    async fn test_me_returns_profile_for_valid_bearer() {
        let app = build_router().await.expect("build router");

        let _ = post_signup(
            app.clone(),
            serde_json::json!({
                "email": "me@example.com",
                "password": "my-strong-password-123"
            }),
        )
        .await;

        let (_, login_body) = post_login(
            app.clone(),
            serde_json::json!({
                "email": "me@example.com",
                "password": "my-strong-password-123"
            }),
        )
        .await;

        let access_token = login_body["access_token"].as_str().unwrap_or_default().to_string();
        let (status, body) = get_me(app, Some(&access_token)).await;

        assert_eq!(status, 200);
        assert_eq!(body["email"], "me@example.com");
        assert_eq!(body["user_id"], 1);
    }

    #[tokio::test]
    async fn test_me_rejects_missing_bearer() {
        let app = build_router().await.expect("build router");

        let (status, body) = get_me(app, None).await;

        assert_eq!(status, 401);
        assert_eq!(body["error"], "missing_bearer_token");
    }

    #[tokio::test]
    async fn test_me_rejects_invalid_bearer() {
        let app = build_router().await.expect("build router");

        let (status, body) = get_me(app, Some("invalid-access-token")).await;

        assert_eq!(status, 401);
        assert_eq!(body["error"], "invalid_access_token");
    }
}
