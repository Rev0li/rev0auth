use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Role {
    Member,
    Admin,
}

#[derive(Clone)]
pub(crate) struct AppState {
    users_by_email: Arc<RwLock<HashMap<String, UserRecord>>>,
    refresh_tokens: Arc<RwLock<HashMap<String, u64>>>,
    access_tokens: Arc<RwLock<HashMap<String, u64>>>,
    next_user_id: Arc<RwLock<u64>>,
    next_refresh_id: Arc<RwLock<u64>>,
    next_access_id: Arc<RwLock<u64>>,
}

#[derive(Debug, Clone)]
pub(crate) struct UserRecord {
    pub(crate) id: u64,
    pub(crate) email: String,
    pub(crate) role: Role,
    pub(crate) password_hash: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct SignupRequest {
    pub(crate) email: String,
    pub(crate) password: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct LoginRequest {
    pub(crate) email: String,
    pub(crate) password: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct RefreshRequest {
    pub(crate) refresh_token: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct SignupResponse {
    pub(crate) user_id: u64,
    pub(crate) email: String,
    pub(crate) role: Role,
}

#[derive(Debug, Serialize)]
pub(crate) struct LoginResponse {
    pub(crate) user_id: u64,
    pub(crate) email: String,
    pub(crate) role: Role,
    pub(crate) refresh_token: String,
    pub(crate) access_token: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct RefreshResponse {
    pub(crate) user_id: u64,
    pub(crate) email: String,
    pub(crate) refresh_token: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct MeResponse {
    pub(crate) user_id: u64,
    pub(crate) email: String,
    pub(crate) role: Role,
}

#[derive(Debug, Serialize)]
pub(crate) struct AdminPanelResponse {
    pub(crate) status: &'static str,
    pub(crate) actor_email: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct HealthResponse {
    pub(crate) service: &'static str,
    pub(crate) status: &'static str,
}

#[derive(Debug, Serialize)]
pub(crate) struct ErrorResponse {
    pub(crate) error: &'static str,
}

#[derive(Debug)]
pub(crate) struct AppError {
    status: StatusCode,
    code: &'static str,
}

impl AppState {
    pub(crate) fn new() -> Self {
        Self {
            users_by_email: Arc::new(RwLock::new(HashMap::new())),
            refresh_tokens: Arc::new(RwLock::new(HashMap::new())),
            access_tokens: Arc::new(RwLock::new(HashMap::new())),
            next_user_id: Arc::new(RwLock::new(1)),
            next_refresh_id: Arc::new(RwLock::new(1)),
            next_access_id: Arc::new(RwLock::new(1)),
        }
    }

    pub(crate) async fn next_user_id(&self) -> u64 {
        let mut next = self.next_user_id.write().await;
        let id = *next;
        *next += 1;
        id
    }

    pub(crate) async fn next_refresh_token_id(&self) -> u64 {
        let mut next = self.next_refresh_id.write().await;
        let id = *next;
        *next += 1;
        id
    }

    pub(crate) async fn next_access_token_id(&self) -> u64 {
        let mut next = self.next_access_id.write().await;
        let id = *next;
        *next += 1;
        id
    }

    pub(crate) async fn user_exists(&self, email: &str) -> bool {
        let users = self.users_by_email.read().await;
        users.contains_key(email)
    }

    pub(crate) async fn insert_user(&self, user: UserRecord) {
        let mut users = self.users_by_email.write().await;
        users.insert(user.email.clone(), user);
    }

    pub(crate) async fn find_user_by_email(&self, email: &str) -> Option<UserRecord> {
        let users = self.users_by_email.read().await;
        users.get(email).cloned()
    }

    pub(crate) async fn find_user_by_id(&self, user_id: u64) -> Option<UserRecord> {
        let users = self.users_by_email.read().await;
        users.values().find(|user| user.id == user_id).cloned()
    }

    pub(crate) async fn store_refresh_token(&self, token: String, user_id: u64) {
        let mut tokens = self.refresh_tokens.write().await;
        tokens.insert(token, user_id);
    }

    pub(crate) async fn consume_refresh_token(&self, token: &str) -> Option<u64> {
        let mut tokens = self.refresh_tokens.write().await;
        tokens.remove(token)
    }

    pub(crate) async fn store_access_token(&self, token: String, user_id: u64) {
        let mut tokens = self.access_tokens.write().await;
        tokens.insert(token, user_id);
    }

    pub(crate) async fn access_token_user_id(&self, token: &str) -> Option<u64> {
        let tokens = self.access_tokens.read().await;
        tokens.get(token).copied()
    }
}

impl AppError {
    pub(crate) fn new(status: StatusCode, code: &'static str) -> Self {
        Self { status, code }
    }

    pub(crate) fn bad_request(code: &'static str) -> Self {
        Self::new(StatusCode::BAD_REQUEST, code)
    }

    pub(crate) fn unauthorized(code: &'static str) -> Self {
        Self::new(StatusCode::UNAUTHORIZED, code)
    }

    pub(crate) fn conflict(code: &'static str) -> Self {
        Self::new(StatusCode::CONFLICT, code)
    }

    pub(crate) fn forbidden(code: &'static str) -> Self {
        Self::new(StatusCode::FORBIDDEN, code)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (self.status, Json(ErrorResponse { error: self.code })).into_response()
    }
}
