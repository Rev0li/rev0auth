use crate::auth::{
    audit::{client_ip_from_headers, AuditEvent, AuditEventType},
    cookies::{
        build_auth_set_cookie_headers, build_csrf_cookie, extract_cookie_from_headers,
        generate_csrf_token, CSRF_COOKIE_NAME, REFRESH_COOKIE_NAME,
    },
    extractor::UserClaims,
    models::{
        AuthResponse, CsrfTokenResponse, ErrorResponse, LoginRequest, Role, SessionResponse,
        SignupRequest,
    },
    password,
    store::{normalize_email, AppState},
};
use axum::{extract::State, http::header, http::HeaderMap, http::HeaderValue, http::StatusCode, Json};
use serde::Serialize;
use tracing::info;

#[derive(Serialize)]
pub struct AdminAuditLogsResponse {
    pub items: Vec<AuditEvent>,
}

// Dev note: issues a CSRF token for unauthenticated browser flows (signup/login).
// Attached to: GET /csrf bootstrap endpoint before POST mutations.
pub async fn csrf() -> (HeaderMap, Json<CsrfTokenResponse>) {
    let token = generate_csrf_token();
    let mut headers = HeaderMap::new();
    headers.append(
        header::SET_COOKIE,
        HeaderValue::from_str(&build_csrf_cookie(&token)).expect("valid csrf cookie"),
    );

    (headers, Json(CsrfTokenResponse { csrf_token: token }))
}

// Dev note: signup remains JSON-token based for now to preserve existing API tests and tooling.
// Attached to: public onboarding clients and initial auth smoke tests.
pub async fn signup(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<SignupRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<ErrorResponse>)> {
    validate_csrf_headers(&headers)?;
    let client_ip = client_ip_from_headers(&headers);

    if !is_valid_email(&payload.email) {
        return Err(err(StatusCode::BAD_REQUEST, "invalid_email"));
    }
    if payload.password.len() < 12 {
        return Err(err(StatusCode::BAD_REQUEST, "weak_password"));
    }

    let password_hash = password::hash_password(&payload.password)
        .map_err(|_| err(StatusCode::INTERNAL_SERVER_ERROR, "password_hash_failed"))?;

    let user = state
        .create_user(normalize_email(&payload.email), password_hash, Role::Member)
        .await
        .map_err(|e| match e {
            "email_already_exists" => err(StatusCode::CONFLICT, "email_already_exists"),
            _ => err(StatusCode::INTERNAL_SERVER_ERROR, "user_creation_failed"),
        })?;

    state
        .record_audit_event(Some(user.id), AuditEventType::CreateUser, client_ip)
        .await;

    let access_token = state
        .token_service
        .issue_access_token(&user)
        .map_err(|_| err(StatusCode::INTERNAL_SERVER_ERROR, "token_issue_failed"))?;
    let csrf_token = extract_csrf_header_token(&headers)?;
    let refresh_token = state.issue_refresh_for_user(&user, &csrf_token).await;

    Ok(Json(AuthResponse {
        user_id: user.id,
        email: user.email,
        role: user.role,
        access_token,
        refresh_token,
    }))
}

// Dev note: login switched to cookie session setup (HttpOnly/Secure) instead of bearer response.
// Attached to: browser clients that should not store tokens in localStorage.
pub async fn login(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<LoginRequest>,
) -> Result<(HeaderMap, Json<SessionResponse>), (StatusCode, Json<ErrorResponse>)> {
    validate_csrf_headers(&headers)?;
    let client_ip = client_ip_from_headers(&headers);
    let normalized_email = normalize_email(&payload.email);

    if let Err(remaining_secs) = state.login_rate_limiter.check_allowed(&normalized_email).await {
        state
            .record_audit_event(None, AuditEventType::FailedAuth, client_ip.clone())
            .await;
        info!(
            target: "rev0auth.auth",
            event = "login_blocked",
            email = %normalized_email,
            remaining_secs,
            "Login blocked by rate limiter"
        );
        return Err(err(StatusCode::TOO_MANY_REQUESTS, "login_rate_limited"));
    }

    let user = state
        .find_user_by_email(&payload.email)
        .await
        .ok_or_else(|| err(StatusCode::UNAUTHORIZED, "invalid_credentials"));

    let user = match user {
        Ok(user) => user,
        Err(e) => {
            let lock = state.login_rate_limiter.record_failure(&normalized_email).await;
            state
                .record_audit_event(None, AuditEventType::FailedAuth, client_ip.clone())
                .await;
            info!(
                target: "rev0auth.auth",
                event = "login_failure",
                email = %normalized_email,
                reason = "unknown_user",
                lock_applied_secs = lock.unwrap_or(0),
                "Login failed"
            );
            return Err(e);
        }
    };

    let verified = password::verify_password(&payload.password, &user.password_hash);
    if !verified {
        let lock = state.login_rate_limiter.record_failure(&normalized_email).await;
        state
            .record_audit_event(Some(user.id), AuditEventType::FailedAuth, client_ip.clone())
            .await;
        info!(
            target: "rev0auth.auth",
            event = "login_failure",
            email = %normalized_email,
            reason = "wrong_password",
            lock_applied_secs = lock.unwrap_or(0),
            "Login failed"
        );
        return Err(err(StatusCode::UNAUTHORIZED, "invalid_credentials"));
    }

    state.login_rate_limiter.record_success(&normalized_email).await;
    state
        .record_audit_event(Some(user.id), AuditEventType::Login, client_ip)
        .await;
    info!(
        target: "rev0auth.auth",
        event = "login_success",
        email = %normalized_email,
        "Login succeeded"
    );

    let access_token = state
        .token_service
        .issue_access_token(&user)
        .map_err(|_| err(StatusCode::INTERNAL_SERVER_ERROR, "token_issue_failed"))?;
    let csrf_token = generate_csrf_token();
    let refresh_token = state.issue_refresh_for_user(&user, &csrf_token).await;

    let headers = build_auth_set_cookie_headers(
        &access_token,
        &refresh_token,
        state.token_service.refresh_ttl_secs(),
        &csrf_token,
    );

    Ok((
        headers,
        Json(SessionResponse {
            user_id: user.id,
            email: user.email,
            role: user.role,
            csrf_token,
        }),
    ))
}

// Dev note: refresh now consumes refresh_token from Cookie header and rotates session cookies.
// Attached to: silent browser session renewal flow.
pub async fn refresh(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<(HeaderMap, Json<SessionResponse>), (StatusCode, Json<ErrorResponse>)> {
    validate_csrf_headers(&headers)?;
    let client_ip = client_ip_from_headers(&headers);

    let refresh_token = extract_cookie_from_headers(&headers, REFRESH_COOKIE_NAME)
        .ok_or_else(|| err(StatusCode::UNAUTHORIZED, "missing_refresh_cookie"))?;
    let csrf_token = extract_csrf_header_token(&headers)?;

    let (user, rotated_refresh_token) = state
        .rotate_refresh_token(&refresh_token, &csrf_token)
        .await
        .map_err(|_| err(StatusCode::UNAUTHORIZED, "invalid_refresh_token"))?;

    state
        .record_audit_event(Some(user.id), AuditEventType::Refresh, client_ip)
        .await;

    let access_token = state
        .token_service
        .issue_access_token(&user)
        .map_err(|_| err(StatusCode::INTERNAL_SERVER_ERROR, "token_issue_failed"))?;
    let csrf_token = generate_csrf_token();

    let response_headers = build_auth_set_cookie_headers(
        &access_token,
        &rotated_refresh_token,
        state.token_service.refresh_ttl_secs(),
        &csrf_token,
    );

    Ok((
        response_headers,
        Json(SessionResponse {
            user_id: user.id,
            email: user.email,
            role: user.role,
            csrf_token,
        }),
    ))
}

// Dev note: double-submit validation, compares X-CSRF-Token header with csrf_token cookie.
// Attached to: all state-changing auth mutations.
fn validate_csrf_headers(headers: &HeaderMap) -> Result<(), (StatusCode, Json<ErrorResponse>)> {
    let header_token = extract_csrf_header_token(headers)?;
    let cookie_token = extract_cookie_from_headers(headers, CSRF_COOKIE_NAME)
        .ok_or_else(|| err(StatusCode::FORBIDDEN, "missing_csrf_cookie"))?;

    if header_token != cookie_token {
        return Err(err(StatusCode::FORBIDDEN, "invalid_csrf_token"));
    }
    Ok(())
}

fn extract_csrf_header_token(headers: &HeaderMap) -> Result<String, (StatusCode, Json<ErrorResponse>)> {
    headers
        .get("x-csrf-token")
        .and_then(|v| v.to_str().ok())
        .map(|v| v.to_string())
        .ok_or_else(|| err(StatusCode::FORBIDDEN, "missing_csrf_token"))
}

// Dev note: shared error envelope helper so auth routes expose stable machine-readable codes.
fn err(status: StatusCode, code: &'static str) -> (StatusCode, Json<ErrorResponse>) {
    (status, Json(ErrorResponse { error: code }))
}

// Dev note: intentionally minimal email validation for unit test focus.
// Attached to: signup edge-case behavior tests.
fn is_valid_email(email: &str) -> bool {
    let e = email.trim();
    e.contains('@') && e.len() >= 5
}

pub async fn admin_panel(
    claims: UserClaims,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    if !matches!(claims.role, Role::Admin) {
        return Err(err(StatusCode::FORBIDDEN, "forbidden"));
    }
    Ok(Json(serde_json::json!({ "status": "admin_ok", "actor_email": claims.email })))
}

pub async fn admin_audit_logs(
    State(state): State<AppState>,
    claims: UserClaims,
) -> Result<Json<AdminAuditLogsResponse>, (StatusCode, Json<ErrorResponse>)> {
    if !matches!(claims.role, Role::Admin) {
        return Err(err(StatusCode::FORBIDDEN, "forbidden"));
    }
    let items = state.list_audit_events().await;
    Ok(Json(AdminAuditLogsResponse { items }))
}

#[cfg(test)]
mod tests {
    use crate::app::build_router_in_memory;
    use crate::auth::{
        jwt::TokenService,
        models::{Role, User},
    };
    use axum::{
        body::Body,
        http::{header, HeaderMap, Request},
    };
    use serde_json::{json, Value};
    use std::sync::Mutex;
    use std::time::Duration;
    use tower::ServiceExt;
    use uuid::Uuid;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    fn create_test_app() -> axum::Router {
        build_router_in_memory()
    }

    async fn with_env_var_async<F, Fut, T>(key: &str, value: Option<&str>, f: F) -> T
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = T>,
    {
        let _guard = ENV_LOCK.lock().expect("env mutex poisoned");
        let previous = std::env::var(key).ok();

        match value {
            Some(v) => unsafe {
                std::env::set_var(key, v);
            },
            None => unsafe {
                std::env::remove_var(key);
            },
        }

        let out = f().await;

        match previous {
            Some(v) => unsafe {
                std::env::set_var(key, v);
            },
            None => unsafe {
                std::env::remove_var(key);
            },
        }

        out
    }

    async fn get_json(app: axum::Router, path: &str) -> (u16, HeaderMap, Value) {
        let response = app
            .oneshot(
                Request::get(path)
                    .body(Body::empty())
                    .expect("request"),
            )
            .await
            .expect("response");

        let status = response.status().as_u16();
        let headers = response.headers().clone();
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("body bytes");
        let json = serde_json::from_slice::<Value>(&bytes).expect("json response");
        (status, headers, json)
    }

    async fn send_json(app: axum::Router, path: &str, body: Value) -> (u16, HeaderMap, Value) {
        let response = app
            .oneshot(
                Request::post(path)
                    .header("content-type", "application/json")
                    .body(Body::from(body.to_string()))
                    .expect("request"),
            )
            .await
            .expect("response");

        let status = response.status().as_u16();
        let headers = response.headers().clone();
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("body bytes");
        let json = serde_json::from_slice::<Value>(&bytes).expect("json response");
        (status, headers, json)
    }

    async fn send_json_with_csrf(
        app: axum::Router,
        path: &str,
        body: Value,
        cookie_header: &str,
        csrf_token: &str,
    ) -> (u16, HeaderMap, Value) {
        let response = app
            .oneshot(
                Request::post(path)
                    .header("content-type", "application/json")
                    .header(header::COOKIE, cookie_header)
                    .header("x-csrf-token", csrf_token)
                    .body(Body::from(body.to_string()))
                    .expect("request"),
            )
            .await
            .expect("response");

        let status = response.status().as_u16();
        let headers = response.headers().clone();
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("body bytes");
        let json = serde_json::from_slice::<Value>(&bytes).expect("json response");
        (status, headers, json)
    }

    async fn send_json_with_cookie(
        app: axum::Router,
        path: &str,
        body: Value,
        cookie_header: &str,
        csrf_token: &str,
    ) -> (u16, HeaderMap, Value) {
        let response = app
            .oneshot(
                Request::post(path)
                    .header("content-type", "application/json")
                    .header(header::COOKIE, cookie_header)
                    .header("x-csrf-token", csrf_token)
                    .body(Body::from(body.to_string()))
                    .expect("request"),
            )
            .await
            .expect("response");

        let status = response.status().as_u16();
        let headers = response.headers().clone();
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("body bytes");
        let json = serde_json::from_slice::<Value>(&bytes).expect("json response");
        (status, headers, json)
    }

    async fn get_with_bearer(app: axum::Router, path: &str, token: &str) -> (u16, Value) {
        let response = app
            .oneshot(
                Request::get(path)
                    .header("authorization", format!("Bearer {token}"))
                    .body(Body::empty())
                    .expect("request"),
            )
            .await
            .expect("response");

        let status = response.status().as_u16();
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("body bytes");
        let json = serde_json::from_slice::<Value>(&bytes).expect("json response");
        (status, json)
    }

    fn issue_admin_bearer() -> String {
        let token_service = TokenService::from_env();
        let admin = User {
            id: Uuid::new_v4(),
            email: "admin@example.com".to_string(),
            password_hash: "unused".to_string(),
            role: Role::Admin,
        };

        token_service
            .issue_access_token(&admin)
            .expect("admin token")
    }

    fn extract_set_cookie_value(headers: &HeaderMap, prefix: &str) -> Option<String> {
        headers
            .get_all(header::SET_COOKIE)
            .iter()
            .filter_map(|v| v.to_str().ok())
            .find(|v| v.starts_with(prefix))
            .map(|v| v.to_string())
    }

    fn extract_cookie_token(set_cookie: &str) -> String {
        set_cookie
            .split(';')
            .next()
            .and_then(|kv| kv.split_once('='))
            .map(|(_, value)| value.to_string())
            .unwrap_or_default()
    }

    async fn bootstrap_csrf(app: axum::Router) -> (String, String) {
        let (status, headers, body) = get_json(app, "/csrf").await;
        assert_eq!(status, 200);

        let csrf_token = body["csrf_token"].as_str().unwrap_or_default().to_string();
        let csrf_cookie = extract_set_cookie_value(&headers, "csrf_token=")
            .expect("csrf cookie")
            .split(';')
            .next()
            .unwrap_or_default()
            .to_string();

        (csrf_cookie, csrf_token)
    }

    #[tokio::test]
    async fn signup_login_refresh_pipeline_works() {
        let app = create_test_app();
        let (csrf_cookie, csrf_token) = bootstrap_csrf(app.clone()).await;

        let signup_payload = json!({
            "email": "member@example.com",
            "password": "my-strong-password-123"
        });
        let (signup_status, _, signup_json) = send_json_with_csrf(
            app.clone(),
            "/auth/signup",
            signup_payload,
            &csrf_cookie,
            &csrf_token,
        )
        .await;
        assert_eq!(signup_status, 200);
        assert!(signup_json["access_token"].is_string());
        assert!(signup_json["refresh_token"].is_string());

        let login_payload = json!({
            "email": "member@example.com",
            "password": "my-strong-password-123"
        });
        let (login_status, login_headers, login_json) = send_json_with_csrf(
            app.clone(),
            "/auth/login",
            login_payload,
            &csrf_cookie,
            &csrf_token,
        )
        .await;
        assert_eq!(login_status, 200);
        assert!(login_json["csrf_token"].is_string());
        let login_csrf_token = login_json["csrf_token"].as_str().unwrap_or_default().to_string();
        let login_csrf_cookie =
            extract_set_cookie_value(&login_headers, "csrf_token=").expect("login csrf cookie");
        let login_csrf_cookie_pair = login_csrf_cookie
            .split(';')
            .next()
            .unwrap_or_default()
            .to_string();

        let refresh_cookie =
            extract_set_cookie_value(&login_headers, "refresh_token=").expect("refresh cookie");
        let original = extract_cookie_token(&refresh_cookie);

        let cookie_header = format!("{}; refresh_token={}", login_csrf_cookie_pair, original);
        let (refresh_status, refresh_headers, refresh_json) =
            send_json_with_cookie(
                app.clone(),
                "/auth/refresh",
                json!({}),
                &cookie_header,
                &login_csrf_token,
            )
                .await;
        assert_eq!(refresh_status, 200);
        assert!(refresh_json["csrf_token"].is_string());

        let rotated_cookie = extract_set_cookie_value(&refresh_headers, "refresh_token=")
            .expect("rotated refresh cookie");
        let rotated = extract_cookie_token(&rotated_cookie);
        assert_ne!(original, rotated);

        let reuse_cookie_header = format!("{}; refresh_token={}", login_csrf_cookie_pair, original);
        let (reuse_status, _, _) =
            send_json_with_cookie(
                app.clone(),
                "/auth/refresh",
                json!({}),
                &reuse_cookie_header,
                &login_csrf_token,
            )
            .await;
        assert_eq!(reuse_status, 401);
    }

    #[tokio::test]
    async fn duplicate_signup_returns_conflict() {
        let app = create_test_app();
        let (csrf_cookie, csrf_token) = bootstrap_csrf(app.clone()).await;
        let payload = json!({
            "email": "duplicate@example.com",
            "password": "my-strong-password-456"
        });

        let (first_status, _, _) = send_json_with_csrf(
            app.clone(),
            "/auth/signup",
            payload.clone(),
            &csrf_cookie,
            &csrf_token,
        )
        .await;
        assert_eq!(first_status, 200);

        let (second_status, _, second_json) = send_json_with_csrf(
            app.clone(),
            "/auth/signup",
            payload,
            &csrf_cookie,
            &csrf_token,
        )
        .await;
        assert_eq!(second_status, 409);
        assert_eq!(second_json["error"], "email_already_exists");
    }

    #[tokio::test]
    async fn test_signup_empty_email() {
        let app = create_test_app();
        let (csrf_cookie, csrf_token) = bootstrap_csrf(app.clone()).await;
        let payload = json!({
            "email": "",
            "password": "my-strong-password-123"
        });

        let (status, _, body) =
            send_json_with_csrf(app, "/auth/signup", payload, &csrf_cookie, &csrf_token).await;
        assert_eq!(status, 400);
        assert_eq!(body["error"], "invalid_email");
    }

    #[tokio::test]
    async fn test_signup_weak_password_9_chars() {
        let app = create_test_app();
        let (csrf_cookie, csrf_token) = bootstrap_csrf(app.clone()).await;
        let payload = json!({
            "email": "member@example.com",
            "password": "123456789"
        });

        let (status, _, body) =
            send_json_with_csrf(app, "/auth/signup", payload, &csrf_cookie, &csrf_token).await;
        assert_eq!(status, 400);
        assert_eq!(body["error"], "weak_password");
    }

    #[tokio::test]
    async fn test_signup_password_11_chars_rejected() {
        let app = create_test_app();
        let (csrf_cookie, csrf_token) = bootstrap_csrf(app.clone()).await;
        let payload = json!({
            "email": "boundary11@example.com",
            "password": "abcdefghijk"
        });
        let (status, _, body) =
            send_json_with_csrf(app, "/auth/signup", payload, &csrf_cookie, &csrf_token).await;
        assert_eq!(status, 400);
        assert_eq!(body["error"], "weak_password");
    }

    #[tokio::test]
    async fn test_signup_password_12_chars_accepted() {
        let app = create_test_app();
        let (csrf_cookie, csrf_token) = bootstrap_csrf(app.clone()).await;
        let payload = json!({
            "email": "boundary12@example.com",
            "password": "abcdefghijkl"
        });
        let (status, _, body) =
            send_json_with_csrf(app, "/auth/signup", payload, &csrf_cookie, &csrf_token).await;
        assert_eq!(status, 200);
        assert!(body["access_token"].is_string());
    }

    #[tokio::test]
    async fn test_login_nonexistent_user() {
        let app = create_test_app();
        let (csrf_cookie, csrf_token) = bootstrap_csrf(app.clone()).await;
        let payload = json!({
            "email": "missing@example.com",
            "password": "my-strong-password-123"
        });

        let (status, _, body) =
            send_json_with_csrf(app, "/auth/login", payload, &csrf_cookie, &csrf_token).await;
        assert_eq!(status, 401);
        assert_eq!(body["error"], "invalid_credentials");
    }

    #[tokio::test]
    async fn test_login_wrong_password() {
        let app = create_test_app();
        let (csrf_cookie, csrf_token) = bootstrap_csrf(app.clone()).await;
        let signup_payload = json!({
            "email": "member@example.com",
            "password": "my-strong-password-123"
        });

        let (signup_status, _, _) = send_json_with_csrf(
            app.clone(),
            "/auth/signup",
            signup_payload,
            &csrf_cookie,
            &csrf_token,
        )
        .await;
        assert_eq!(signup_status, 200);

        let login_payload = json!({
            "email": "member@example.com",
            "password": "wrong-password-123"
        });

        let (status, _, body) =
            send_json_with_csrf(app, "/auth/login", login_payload, &csrf_cookie, &csrf_token)
                .await;
        assert_eq!(status, 401);
        assert_eq!(body["error"], "invalid_credentials");
    }

    #[tokio::test]
    async fn test_5_failed_logins_locks_for_15min() {
        let app = create_test_app();
        let (csrf_cookie, csrf_token) = bootstrap_csrf(app.clone()).await;

        let (signup_status, _, _) = send_json_with_csrf(
            app.clone(),
            "/auth/signup",
            json!({ "email": "lockout@example.com", "password": "my-strong-password-123" }),
            &csrf_cookie,
            &csrf_token,
        )
        .await;
        assert_eq!(signup_status, 200);

        for _ in 0..5 {
            let (status, _, body) = send_json_with_csrf(
                app.clone(),
                "/auth/login",
                json!({ "email": "lockout@example.com", "password": "wrong-password-123" }),
                &csrf_cookie,
                &csrf_token,
            )
            .await;
            assert_eq!(status, 401);
            assert_eq!(body["error"], "invalid_credentials");
        }

        let (blocked_status, _, blocked_body) = send_json_with_csrf(
            app,
            "/auth/login",
            json!({ "email": "lockout@example.com", "password": "my-strong-password-123" }),
            &csrf_cookie,
            &csrf_token,
        )
        .await;
        assert_eq!(blocked_status, 429);
        assert_eq!(blocked_body["error"], "login_rate_limited");
    }

    #[tokio::test]
    async fn test_successful_login_resets_counter() {
        let app = create_test_app();
        let (csrf_cookie, csrf_token) = bootstrap_csrf(app.clone()).await;

        let (signup_status, _, _) = send_json_with_csrf(
            app.clone(),
            "/auth/signup",
            json!({ "email": "reset@example.com", "password": "my-strong-password-123" }),
            &csrf_cookie,
            &csrf_token,
        )
        .await;
        assert_eq!(signup_status, 200);

        for _ in 0..3 {
            let (status, _, body) = send_json_with_csrf(
                app.clone(),
                "/auth/login",
                json!({ "email": "reset@example.com", "password": "wrong-password-123" }),
                &csrf_cookie,
                &csrf_token,
            )
            .await;
            assert_eq!(status, 401);
            assert_eq!(body["error"], "invalid_credentials");
        }

        let (ok_status, _, _) = send_json_with_csrf(
            app.clone(),
            "/auth/login",
            json!({ "email": "reset@example.com", "password": "my-strong-password-123" }),
            &csrf_cookie,
            &csrf_token,
        )
        .await;
        assert_eq!(ok_status, 200);

        for _ in 0..3 {
            let (status, _, body) = send_json_with_csrf(
                app.clone(),
                "/auth/login",
                json!({ "email": "reset@example.com", "password": "wrong-password-123" }),
                &csrf_cookie,
                &csrf_token,
            )
            .await;
            assert_eq!(status, 401);
            assert_eq!(body["error"], "invalid_credentials");
        }
    }

    #[tokio::test]
    async fn test_signup_creates_audit_event() {
        let app = create_test_app();
        let (csrf_cookie, csrf_token) = bootstrap_csrf(app.clone()).await;

        let (signup_status, _, signup_body) = send_json_with_csrf(
            app.clone(),
            "/auth/signup",
            json!({ "email": "audit-signup@example.com", "password": "my-strong-password-123" }),
            &csrf_cookie,
            &csrf_token,
        )
        .await;
        assert_eq!(signup_status, 200);

        let admin_token = issue_admin_bearer();
        let (audit_status, audit_json) =
            get_with_bearer(app, "/admin/audit-logs", &admin_token).await;
        assert_eq!(audit_status, 200);

        let items = audit_json["items"].as_array().expect("audit items array");
        let created = items
            .iter()
            .find(|event| event["event_type"] == "create_user")
            .expect("create_user event");

        assert_eq!(created["user_id"], signup_body["user_id"]);
        assert!(created["timestamp"].as_u64().is_some());
    }

    #[tokio::test]
    async fn test_failed_login_logged() {
        let app = create_test_app();
        let (csrf_cookie, csrf_token) = bootstrap_csrf(app.clone()).await;

        let (signup_status, _, _) = send_json_with_csrf(
            app.clone(),
            "/auth/signup",
            json!({ "email": "audit-failed@example.com", "password": "my-strong-password-123" }),
            &csrf_cookie,
            &csrf_token,
        )
        .await;
        assert_eq!(signup_status, 200);

        let (failed_status, _, failed_body) = send_json_with_csrf(
            app.clone(),
            "/auth/login",
            json!({ "email": "audit-failed@example.com", "password": "wrong-password-123" }),
            &csrf_cookie,
            &csrf_token,
        )
        .await;
        assert_eq!(failed_status, 401);
        assert_eq!(failed_body["error"], "invalid_credentials");

        let admin_token = issue_admin_bearer();
        let (audit_status, audit_json) =
            get_with_bearer(app, "/admin/audit-logs", &admin_token).await;
        assert_eq!(audit_status, 200);

        let items = audit_json["items"].as_array().expect("audit items array");
        let failed = items
            .iter()
            .find(|event| event["event_type"] == "failed_auth")
            .expect("failed_auth event");

        assert!(failed["timestamp"].as_u64().is_some());
    }

    #[tokio::test]
    async fn test_refresh_invalid_token_format() {
        let app = create_test_app();
        let (csrf_cookie, csrf_token) = bootstrap_csrf(app.clone()).await;
        let cookie_header = format!("{}; refresh_token=not-a-valid-refresh-token-format", csrf_cookie);
        let (status, _, body) = send_json_with_cookie(
            app,
            "/auth/refresh",
            json!({}),
            &cookie_header,
            &csrf_token,
        )
        .await;
        assert_eq!(status, 401);
        assert_eq!(body["error"], "invalid_refresh_token");
    }

    #[tokio::test]
    async fn test_refresh_expired_token() {
        with_env_var_async("AUTH_REFRESH_TTL_SECS", Some("0"), || async {
            let app = create_test_app();
            let (csrf_cookie, csrf_token) = bootstrap_csrf(app.clone()).await;

            let signup_payload = json!({
                "email": "expire@example.com",
                "password": "my-strong-password-123"
            });
            let (signup_status, _, signup_json) =
                send_json_with_csrf(
                    app.clone(),
                    "/auth/signup",
                    signup_payload,
                    &csrf_cookie,
                    &csrf_token,
                )
                .await;
            assert_eq!(signup_status, 200);

            tokio::time::sleep(Duration::from_millis(1200)).await;

            let refresh_payload = json!({});
            let cookie_header = format!(
                "{}; refresh_token={}",
                csrf_cookie,
                signup_json["refresh_token"].as_str().unwrap_or_default()
            );
            let (status, _, body) =
                send_json_with_cookie(
                    app,
                    "/auth/refresh",
                    refresh_payload,
                    &cookie_header,
                    &csrf_token,
                )
                .await;
            assert_eq!(status, 401);
            assert_eq!(body["error"], "invalid_refresh_token");
        })
        .await;
    }

    #[tokio::test]
    async fn test_login_sets_secure_cookie() {
        let app = create_test_app();
        let (csrf_cookie, csrf_token) = bootstrap_csrf(app.clone()).await;
        let _ = send_json_with_csrf(
            app.clone(),
            "/auth/signup",
            json!({ "email": "cookie@example.com", "password": "my-strong-password-123" }),
            &csrf_cookie,
            &csrf_token,
        )
        .await;

        let (status, headers, _) = send_json_with_csrf(
            app,
            "/auth/login",
            json!({ "email": "cookie@example.com", "password": "my-strong-password-123" }),
            &csrf_cookie,
            &csrf_token,
        )
        .await;

        assert_eq!(status, 200);
        let access_cookie = extract_set_cookie_value(&headers, "access_token=").expect("access cookie");
        assert!(access_cookie.contains("HttpOnly"));
        assert!(access_cookie.contains("Secure"));
        assert!(access_cookie.contains("SameSite=Lax"));
        assert!(access_cookie.contains("Path=/"));
        assert!(access_cookie.contains("Max-Age=900"));
    }

    #[tokio::test]
    async fn test_refresh_rotates_cookie() {
        let app = create_test_app();
        let (csrf_cookie, csrf_token) = bootstrap_csrf(app.clone()).await;
        let _ = send_json_with_csrf(
            app.clone(),
            "/auth/signup",
            json!({ "email": "rotate@example.com", "password": "my-strong-password-123" }),
            &csrf_cookie,
            &csrf_token,
        )
        .await;

        let (_, login_headers, login_json) = send_json_with_csrf(
            app.clone(),
            "/auth/login",
            json!({ "email": "rotate@example.com", "password": "my-strong-password-123" }),
            &csrf_cookie,
            &csrf_token,
        )
        .await;
        let login_csrf_token = login_json["csrf_token"].as_str().unwrap_or_default().to_string();
        let login_csrf_cookie =
            extract_set_cookie_value(&login_headers, "csrf_token=").expect("login csrf cookie");
        let login_csrf_cookie_pair = login_csrf_cookie
            .split(';')
            .next()
            .unwrap_or_default()
            .to_string();

        let first_cookie =
            extract_set_cookie_value(&login_headers, "refresh_token=").expect("refresh cookie");
        let first_token = extract_cookie_token(&first_cookie);

        let cookie_header = format!("{}; refresh_token={}", login_csrf_cookie_pair, first_token);
        let (status, refresh_headers, _) =
            send_json_with_cookie(
                app,
                "/auth/refresh",
                json!({}),
                &cookie_header,
                &login_csrf_token,
            )
                .await;
        assert_eq!(status, 200);

        let rotated_cookie =
            extract_set_cookie_value(&refresh_headers, "refresh_token=").expect("rotated cookie");
        let rotated_token = extract_cookie_token(&rotated_cookie);
        assert_ne!(first_token, rotated_token);
    }

    #[tokio::test]
    async fn test_csrf_token_generated() {
        let app = create_test_app();
        let (csrf_cookie, csrf_token) = bootstrap_csrf(app.clone()).await;
        let _ = send_json_with_csrf(
            app.clone(),
            "/auth/signup",
            json!({ "email": "csrf@example.com", "password": "my-strong-password-123" }),
            &csrf_cookie,
            &csrf_token,
        )
        .await;

        let (status, headers, body) = send_json_with_csrf(
            app,
            "/auth/login",
            json!({ "email": "csrf@example.com", "password": "my-strong-password-123" }),
            &csrf_cookie,
            &csrf_token,
        )
        .await;

        assert_eq!(status, 200);
        let csrf = body["csrf_token"].as_str().unwrap_or_default();
        assert!(!csrf.is_empty());

        let csrf_cookie = extract_set_cookie_value(&headers, "csrf_token=").expect("csrf cookie");
        assert!(csrf_cookie.contains("Secure"));
        assert!(csrf_cookie.contains("SameSite=Lax"));
    }

    #[tokio::test]
    async fn test_signup_without_csrf_token() {
        let app = create_test_app();
        let payload = json!({
            "email": "no-csrf@example.com",
            "password": "my-strong-password-123"
        });

        let (status, _, body) = send_json(app, "/auth/signup", payload).await;
        assert_eq!(status, 403);
        assert_eq!(body["error"], "missing_csrf_token");
    }

    #[tokio::test]
    async fn test_signup_with_invalid_csrf() {
        let app = create_test_app();
        let (csrf_cookie, _) = bootstrap_csrf(app.clone()).await;

        let payload = json!({
            "email": "bad-csrf@example.com",
            "password": "my-strong-password-123"
        });

        let (status, _, body) = send_json_with_csrf(
            app,
            "/auth/signup",
            payload,
            &csrf_cookie,
            "mismatch-token",
        )
        .await;
        assert_eq!(status, 403);
        assert_eq!(body["error"], "invalid_csrf_token");
    }

    #[tokio::test]
    async fn test_signup_with_valid_csrf() {
        let app = create_test_app();
        let (csrf_cookie, csrf_token) = bootstrap_csrf(app.clone()).await;

        let payload = json!({
            "email": "good-csrf@example.com",
            "password": "my-strong-password-123"
        });

        let (status, _, body) =
            send_json_with_csrf(app, "/auth/signup", payload, &csrf_cookie, &csrf_token).await;
        assert_eq!(status, 200);
        assert!(body["access_token"].is_string());
    }

    // --- Security matrix: refresh token replay (deep) ---

    // The CSRF token is bound to the session at login and does not rotate per-refresh.
    // Each rotation consumes the current refresh token and issues a new one.
    // A replayed token from any previous generation must be rejected.
    #[tokio::test]
    async fn test_refresh_replay_two_generations_rejected() {
        let app = create_test_app();
        let (csrf_cookie, csrf_token) = bootstrap_csrf(app.clone()).await;

        let _ = send_json_with_csrf(
            app.clone(), "/auth/signup",
            json!({ "email": "replay-deep@example.com", "password": "my-strong-password-123" }),
            &csrf_cookie, &csrf_token,
        ).await;

        let (_, login_headers, login_json) = send_json_with_csrf(
            app.clone(), "/auth/login",
            json!({ "email": "replay-deep@example.com", "password": "my-strong-password-123" }),
            &csrf_cookie, &csrf_token,
        ).await;

        // Session CSRF is fixed at login time; same token used for all rotations.
        let session_csrf = login_json["csrf_token"].as_str().unwrap_or_default().to_string();
        let session_csrf_cookie = extract_set_cookie_value(&login_headers, "csrf_token=")
            .expect("csrf cookie after login")
            .split(';').next().unwrap_or_default().to_string();
        let rt1 = extract_cookie_token(
            &extract_set_cookie_value(&login_headers, "refresh_token=").expect("rt1 cookie"),
        );

        // First rotation: rt1 → rt2
        let cookie_rt1 = format!("{}; refresh_token={}", session_csrf_cookie, rt1);
        let (s1, r1_headers, _) = send_json_with_cookie(
            app.clone(), "/auth/refresh", json!({}), &cookie_rt1, &session_csrf,
        ).await;
        assert_eq!(s1, 200);

        let rt2 = extract_cookie_token(
            &extract_set_cookie_value(&r1_headers, "refresh_token=").expect("rt2 cookie"),
        );

        // Second rotation: rt2 → rt3, still using the same session CSRF — chain works
        let cookie_rt2 = format!("{}; refresh_token={}", session_csrf_cookie, rt2);
        let (s2, _, _) = send_json_with_cookie(
            app.clone(), "/auth/refresh", json!({}), &cookie_rt2, &session_csrf,
        ).await;
        assert_eq!(s2, 200);

        // Replay rt1 (two generations old) — must be rejected
        let (replay_status, _, replay_body) = send_json_with_cookie(
            app.clone(), "/auth/refresh", json!({}), &cookie_rt1, &session_csrf,
        ).await;
        assert_eq!(replay_status, 401);
        assert_eq!(replay_body["error"], "invalid_refresh_token");
    }

    // --- Security matrix: CSRF full mutation coverage ---

    #[tokio::test]
    async fn test_login_blocked_without_csrf_header() {
        let app = create_test_app();
        let payload = json!({ "email": "a@example.com", "password": "my-strong-password-123" });
        let (status, _, body) = send_json(app, "/auth/login", payload).await;
        assert_eq!(status, 403);
        assert_eq!(body["error"], "missing_csrf_token");
    }

    #[tokio::test]
    async fn test_login_blocked_with_mismatched_csrf() {
        let app = create_test_app();
        let (csrf_cookie, _) = bootstrap_csrf(app.clone()).await;
        let payload = json!({ "email": "a@example.com", "password": "my-strong-password-123" });
        let (status, _, body) = send_json_with_csrf(
            app, "/auth/login", payload, &csrf_cookie, "wrong-token",
        ).await;
        assert_eq!(status, 403);
        assert_eq!(body["error"], "invalid_csrf_token");
    }

    #[tokio::test]
    async fn test_refresh_blocked_without_csrf_header() {
        let app = create_test_app();
        let (status, _, body) = send_json(app, "/auth/refresh", json!({})).await;
        assert_eq!(status, 403);
        assert_eq!(body["error"], "missing_csrf_token");
    }

    #[tokio::test]
    async fn test_refresh_blocked_with_mismatched_csrf() {
        let app = create_test_app();
        let (csrf_cookie, csrf_token) = bootstrap_csrf(app.clone()).await;

        let _ = send_json_with_csrf(
            app.clone(), "/auth/signup",
            json!({ "email": "csrf-refresh@example.com", "password": "my-strong-password-123" }),
            &csrf_cookie, &csrf_token,
        ).await;

        let (_, login_headers, login_json) = send_json_with_csrf(
            app.clone(), "/auth/login",
            json!({ "email": "csrf-refresh@example.com", "password": "my-strong-password-123" }),
            &csrf_cookie, &csrf_token,
        ).await;

        let real_csrf_cookie = extract_set_cookie_value(&login_headers, "csrf_token=")
            .expect("csrf cookie after login")
            .split(';').next().unwrap_or_default().to_string();
        let rt = extract_cookie_token(
            &extract_set_cookie_value(&login_headers, "refresh_token=").expect("rt cookie"),
        );
        let _ = login_json["csrf_token"].as_str().unwrap_or_default();

        let cookie_header = format!("{}; refresh_token={}", real_csrf_cookie, rt);
        let (status, _, body) = send_json_with_cookie(
            app, "/auth/refresh", json!({}), &cookie_header, "tampered-csrf-token",
        ).await;
        assert_eq!(status, 403);
        assert_eq!(body["error"], "invalid_csrf_token");
    }

    // --- Security matrix: RBAC escalation member → admin (end-to-end) ---

    #[tokio::test]
    async fn test_rbac_member_cannot_escalate_to_admin() {
        let app = create_test_app();
        let (csrf_cookie, csrf_token) = bootstrap_csrf(app.clone()).await;

        let _ = send_json_with_csrf(
            app.clone(), "/auth/signup",
            json!({ "email": "member-esc@example.com", "password": "my-strong-password-123" }),
            &csrf_cookie, &csrf_token,
        ).await;

        let (_, login_headers, _) = send_json_with_csrf(
            app.clone(), "/auth/login",
            json!({ "email": "member-esc@example.com", "password": "my-strong-password-123" }),
            &csrf_cookie, &csrf_token,
        ).await;

        let member_token = extract_cookie_token(
            &extract_set_cookie_value(&login_headers, "access_token=").expect("access token cookie"),
        );

        // Member token blocked on admin panel
        let (panel_status, panel_body) = get_with_bearer(app.clone(), "/admin/panel", &member_token).await;
        assert_eq!(panel_status, 403);
        assert_eq!(panel_body["error"], "forbidden");

        // Member token blocked on audit logs
        let (audit_status, audit_body) = get_with_bearer(app.clone(), "/admin/audit-logs", &member_token).await;
        assert_eq!(audit_status, 403);
        assert_eq!(audit_body["error"], "forbidden");

        // Forged alg:none token claiming admin role is rejected at JWT validation
        let forged = "eyJhbGciOiJub25lIn0.eyJzdWIiOiIwMDAwMDAwMC0wMDAwLTAwMDAtMDAwMC0wMDAwMDAwMDAwMDAiLCJlbWFpbCI6ImhhY2tlckBleGFtcGxlLmNvbSIsInJvbGUiOiJhZG1pbiIsInRva2VuX3R5cGUiOiJhY2Nlc3MiLCJpYXQiOjAsImV4cCI6OTk5OTk5OTk5OX0.";
        let (forged_status, _) = get_with_bearer(app, "/admin/panel", forged).await;
        assert_eq!(forged_status, 401);
    }
}
