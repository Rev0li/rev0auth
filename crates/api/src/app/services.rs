use crate::app::domain::{AppError, AppState, Role, UserRecord};
use axum::http::HeaderMap;

pub(crate) fn normalize_email(email: &str) -> String {
    email.trim().to_ascii_lowercase()
}

pub(crate) fn is_valid_email(email: &str) -> bool {
    email.contains('@') && email.len() >= 5
}

pub(crate) fn pseudo_hash(password: &str) -> String {
    format!("step001::{}", password)
}

pub(crate) fn role_for_email(email: &str) -> Role {
    if email == "admin@example.com" {
        Role::Admin
    } else {
        Role::Member
    }
}

pub(crate) async fn issue_refresh_token(state: &AppState, user_id: u64) -> String {
    let token_id = state.next_refresh_token_id().await;
    let token = format!("refresh-{}-{}", user_id, token_id);
    state.store_refresh_token(token.clone(), user_id).await;
    token
}

pub(crate) async fn issue_access_token(state: &AppState, user_id: u64) -> String {
    let token_id = state.next_access_token_id().await;
    let token = format!("access-{}-{}", user_id, token_id);
    state.store_access_token(token.clone(), user_id).await;
    token
}

pub(crate) fn extract_bearer_token(headers: &HeaderMap) -> Option<String> {
    let raw = headers.get("authorization")?.to_str().ok()?;
    raw.strip_prefix("Bearer ").map(|value| value.to_string())
}

pub(crate) async fn authenticated_user(
    state: &AppState,
    headers: &HeaderMap,
) -> Result<UserRecord, AppError> {
    let access_token = extract_bearer_token(headers)
        .ok_or_else(|| AppError::unauthorized("missing_bearer_token"))?;

    let user_id = state
        .access_token_user_id(&access_token)
        .await
        .ok_or_else(|| AppError::unauthorized("invalid_access_token"))?;

    state
        .find_user_by_id(user_id)
        .await
        .ok_or_else(|| AppError::unauthorized("invalid_access_token"))
}
