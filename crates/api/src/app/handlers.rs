use crate::app::domain::{
    AdminPanelResponse, AppError, AppState, HealthResponse, LoginRequest, LoginResponse,
    MeResponse, RefreshRequest, RefreshResponse, Role, SignupRequest, SignupResponse, UserRecord,
};
use crate::app::services::{
    authenticated_user, issue_access_token, issue_refresh_token, is_valid_email, normalize_email,
    pseudo_hash, role_for_email,
};
use axum::{extract::State, http::HeaderMap, routing::get, Json, Router};

pub async fn build_router() -> anyhow::Result<Router> {
    Ok(Router::new()
        .route("/health", get(health))
        .route("/auth/signup", axum::routing::post(signup))
        .route("/auth/login", axum::routing::post(login))
        .route("/auth/refresh", axum::routing::post(refresh))
        .route("/auth/me", get(me))
        .route("/admin/panel", get(admin_panel))
        .with_state(AppState::new()))
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
) -> Result<Json<SignupResponse>, AppError> {
    let normalized_email = normalize_email(&payload.email);

    if !is_valid_email(&normalized_email) {
        return Err(AppError::bad_request("invalid_email"));
    }

    if payload.password.len() < 12 {
        return Err(AppError::bad_request("weak_password"));
    }

    if state.user_exists(&normalized_email).await {
        return Err(AppError::conflict("email_already_exists"));
    }

    let user_id = state.next_user_id().await;
    let role = role_for_email(&normalized_email);
    let record = UserRecord {
        id: user_id,
        email: normalized_email.clone(),
        role: role.clone(),
        password_hash: pseudo_hash(&payload.password),
    };
    state.insert_user(record).await;

    Ok(Json(SignupResponse {
        user_id,
        email: normalized_email,
        role,
    }))
}

async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    let normalized_email = normalize_email(&payload.email);
    let record = state
        .find_user_by_email(&normalized_email)
        .await
        .ok_or_else(|| AppError::unauthorized("invalid_credentials"))?;

    if record.password_hash != pseudo_hash(&payload.password) {
        return Err(AppError::unauthorized("invalid_credentials"));
    }

    let refresh_token = issue_refresh_token(&state, record.id).await;
    let access_token = issue_access_token(&state, record.id).await;

    Ok(Json(LoginResponse {
        user_id: record.id,
        email: record.email,
        role: record.role,
        refresh_token,
        access_token,
    }))
}

async fn refresh(
    State(state): State<AppState>,
    Json(payload): Json<RefreshRequest>,
) -> Result<Json<RefreshResponse>, AppError> {
    let user_id = state
        .consume_refresh_token(&payload.refresh_token)
        .await
        .ok_or_else(|| AppError::unauthorized("invalid_refresh_token"))?;

    let user = state
        .find_user_by_id(user_id)
        .await
        .ok_or_else(|| AppError::unauthorized("invalid_refresh_token"))?;

    let next_refresh_token = issue_refresh_token(&state, user_id).await;

    Ok(Json(RefreshResponse {
        user_id: user.id,
        email: user.email,
        refresh_token: next_refresh_token,
    }))
}

async fn me(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<MeResponse>, AppError> {
    let user = authenticated_user(&state, &headers).await?;

    Ok(Json(MeResponse {
        user_id: user.id,
        email: user.email,
        role: user.role,
    }))
}

async fn admin_panel(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<AdminPanelResponse>, AppError> {
    let user = authenticated_user(&state, &headers).await?;
    if user.role != Role::Admin {
        return Err(AppError::forbidden("forbidden"));
    }

    Ok(Json(AdminPanelResponse {
        status: "admin_ok",
        actor_email: user.email,
    }))
}
