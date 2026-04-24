pub mod audit;
pub mod cookies;
pub mod extractor;
pub mod handlers;
pub mod jwt;
pub mod migrations;
pub mod models;
pub mod password;
pub mod rate_limit;
pub mod rbac;
pub mod store;

#[cfg(test)]
pub fn build_router_in_memory() -> axum::Router {
    use axum::routing::{get, post};
    let state = store::AppState::new_in_memory(jwt::TokenService::from_env());
    axum::Router::new()
        .route("/csrf", get(handlers::csrf))
        .route("/auth/signup", post(handlers::signup))
        .route("/auth/login", post(handlers::login))
        .route("/auth/refresh", post(handlers::refresh))
        .route("/admin/panel", get(handlers::admin_panel))
        .route("/admin/audit-logs", get(handlers::admin_audit_logs))
        .with_state(state)
}
