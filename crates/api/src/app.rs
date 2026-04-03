use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

#[derive(Serialize)]
struct HealthResponse {
    service: &'static str,
    status: &'static str,
}

#[derive(Clone)]
struct AppState {
    // STEP-001 stores users in memory so we can validate the route behavior
    // before introducing DB persistence in later steps.
    users_by_email: Arc<RwLock<HashMap<String, UserRecord>>>,
    next_user_id: Arc<RwLock<u64>>,
}

#[derive(Clone)]
#[allow(dead_code)]
struct UserRecord {
    id: u64,
    email: String,
    // Educational placeholder: in later steps this becomes a real password hash.
    // We keep a non-plaintext transformed value to avoid raw password storage.
    password_hash: String,
}

#[derive(Debug, Deserialize)]
struct SignupRequest {
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct SignupResponse {
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
        next_user_id: Arc::new(RwLock::new(1)),
    };

    Ok(
        Router::new()
            .route("/health", get(health))
            .route("/auth/signup", axum::routing::post(signup))
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

fn normalize_email(email: &str) -> String {
    email.trim().to_ascii_lowercase()
}

fn is_valid_email(email: &str) -> bool {
    email.contains('@') && email.len() >= 5
}

fn pseudo_hash(password: &str) -> String {
    format!("step001::{}", password)
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
}
