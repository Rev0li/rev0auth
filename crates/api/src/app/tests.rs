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

async fn post_login_raw(app: axum::Router, raw_body: &str) -> u16 {
    let response = app
        .oneshot(
            Request::post("/auth/login")
                .header("content-type", "application/json")
                .body(Body::from(raw_body.to_string()))
                .expect("request"),
        )
        .await
        .expect("response");

    response.status().as_u16()
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

async fn get_admin_panel(app: axum::Router, bearer: Option<&str>) -> (u16, Value) {
    let mut request = Request::get("/admin/panel")
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
    assert_eq!(body["role"], "member");
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
    assert_eq!(body["role"], "member");
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
    assert_eq!(body["role"], "member");
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

#[tokio::test]
async fn test_admin_panel_forbidden_for_member() {
    let app = build_router().await.expect("build router");

    let _ = post_signup(
        app.clone(),
        serde_json::json!({
            "email": "member@example.com",
            "password": "my-strong-password-123"
        }),
    )
    .await;

    let (_, login_body) = post_login(
        app.clone(),
        serde_json::json!({
            "email": "member@example.com",
            "password": "my-strong-password-123"
        }),
    )
    .await;

    let access_token = login_body["access_token"].as_str().unwrap_or_default().to_string();
    let (status, body) = get_admin_panel(app, Some(&access_token)).await;

    assert_eq!(status, 403);
    assert_eq!(body["error"], "forbidden");
}

#[tokio::test]
async fn test_admin_panel_allows_admin() {
    let app = build_router().await.expect("build router");

    let _ = post_signup(
        app.clone(),
        serde_json::json!({
            "email": "admin@example.com",
            "password": "my-strong-password-123"
        }),
    )
    .await;

    let (_, login_body) = post_login(
        app.clone(),
        serde_json::json!({
            "email": "admin@example.com",
            "password": "my-strong-password-123"
        }),
    )
    .await;

    let access_token = login_body["access_token"].as_str().unwrap_or_default().to_string();
    let (status, body) = get_admin_panel(app, Some(&access_token)).await;

    assert_eq!(status, 200);
    assert_eq!(body["status"], "admin_ok");
    assert_eq!(body["actor_email"], "admin@example.com");
}

#[tokio::test]
async fn test_login_rejects_malformed_json_payload() {
    let app = build_router().await.expect("build router");

    let status = post_login_raw(app, "{invalid-json").await;
    assert_eq!(status, 400);
}

#[tokio::test]
async fn test_login_rejects_missing_required_fields() {
    let app = build_router().await.expect("build router");

    let status = post_login_raw(app, r#"{"email":"member@example.com"}"#).await;
    assert_eq!(status, 422);
}

#[tokio::test]
async fn test_admin_panel_rejects_forged_bearer() {
    let app = build_router().await.expect("build router");

    let forged = "eyJhbGciOiJub25lIn0.eyJzdWIiOiJhZG1pbkBleGFtcGxlLmNvbSJ9.";
    let (status, body) = get_admin_panel(app, Some(forged)).await;

    assert_eq!(status, 401);
    assert_eq!(body["error"], "invalid_access_token");
}
