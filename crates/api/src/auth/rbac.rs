use crate::auth::{
    extractor::UserClaims,
    models::{ErrorResponse, Role},
};
use axum::{
    extract::FromRequestParts,
    http::{request::Parts, Request, StatusCode},
    middleware::Next,
    response::Response,
    Json,
};

// Dev note: role levels used by middleware to describe minimum access required by a route.
// Attached to: middleware helpers require_member/require_admin.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequiredRole {
    Member,
    Admin,
}

// Dev note: extraction artifact proving role-check was executed for current request.
// Attached to: protected handler signatures.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RoleGuard(pub RequiredRole);

// Dev note: performs authorization (403) after authentication (UserClaims extractor).
// Attached to: all RBAC-protected routes.
#[axum::async_trait]
impl<S> FromRequestParts<S> for RoleGuard
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<ErrorResponse>);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let required = parts
            .extensions
            .get::<RequiredRole>()
            .copied()
            .unwrap_or(RequiredRole::Member);

        let claims = UserClaims::from_request_parts(parts, state).await?;
        if !has_required_role(&claims.role, required) {
            return Err(err(StatusCode::FORBIDDEN, "insufficient_role"));
        }

        Ok(RoleGuard(required))
    }
}

// Dev note: route-layer helper for member-level endpoints.
pub async fn require_member(req: Request<axum::body::Body>, next: Next) -> Response {
    set_required_role(req, next, RequiredRole::Member).await
}

// Dev note: route-layer helper for admin-only endpoints.
pub async fn require_admin(req: Request<axum::body::Body>, next: Next) -> Response {
    set_required_role(req, next, RequiredRole::Admin).await
}

// Dev note: middleware writes the required role into request extensions for RoleGuard.
async fn set_required_role(
    mut req: Request<axum::body::Body>,
    next: Next,
    required: RequiredRole,
) -> Response {
    req.extensions_mut().insert(required);
    next.run(req).await
}

// Dev note: hierarchy rule; admin inherits member permissions.
fn has_required_role(actual: &Role, required: RequiredRole) -> bool {
    match required {
        RequiredRole::Member => matches!(actual, Role::Member | Role::Admin),
        RequiredRole::Admin => matches!(actual, Role::Admin),
    }
}

// Dev note: normalized RBAC rejection envelope.
fn err(status: StatusCode, code: &'static str) -> (StatusCode, Json<ErrorResponse>) {
    (status, Json(ErrorResponse { error: code }))
}

#[cfg(test)]
mod tests {
    use super::{require_admin, require_member, RequiredRole, RoleGuard};
    use crate::auth::{
        extractor::UserClaims,
        jwt::TokenService,
        models::{Role, User},
    };
    use axum::{
        body::{to_bytes, Body},
        http::Request,
        middleware::from_fn,
        routing::get,
        Json, Router,
    };
    use serde_json::{json, Value};
    use std::sync::Mutex;
    use tower::ServiceExt;
    use uuid::Uuid;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    fn make_user(role: Role) -> User {
        User {
            id: Uuid::new_v4(),
            email: match role {
                Role::Member => "member@example.com".to_string(),
                Role::Admin => "admin@example.com".to_string(),
            },
            password_hash: "unused".to_string(),
            role,
        }
    }

    fn create_test_router() -> Router {
        async fn member_route(
            guard: RoleGuard,
            UserClaims { email, role, .. }: UserClaims,
        ) -> Json<Value> {
            assert_eq!(guard.0, RequiredRole::Member);
            Json(json!({ "ok": true, "email": email, "role": role }))
        }

        async fn admin_route(
            guard: RoleGuard,
            UserClaims { email, role, .. }: UserClaims,
        ) -> Json<Value> {
            assert_eq!(guard.0, RequiredRole::Admin);
            Json(json!({ "ok": true, "email": email, "role": role }))
        }

        Router::new()
            .route("/member", get(member_route).route_layer(from_fn(require_member)))
            .route("/admin", get(admin_route).route_layer(from_fn(require_admin)))
    }

    fn issue_token(user: &User) -> String {
        TokenService::from_env()
            .issue_access_token(user)
            .expect("token")
    }

    async fn with_secret<F, Fut, T>(secret: &str, f: F) -> T
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = T>,
    {
        let _guard = ENV_LOCK.lock().expect("env mutex poisoned");
        let prev_secret = std::env::var("AUTH_JWT_SECRET").ok();

        unsafe {
            std::env::set_var("AUTH_JWT_SECRET", secret);
        }

        let out = f().await;

        match prev_secret {
            Some(value) => unsafe {
                std::env::set_var("AUTH_JWT_SECRET", value);
            },
            None => unsafe {
                std::env::remove_var("AUTH_JWT_SECRET");
            },
        }

        out
    }

    async fn get_with_bearer(app: Router, path: &str, token: &str) -> (u16, Value) {
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
        let body = to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("body bytes");
        let json = serde_json::from_slice::<Value>(&body).expect("json response");
        (status, json)
    }

    #[tokio::test]
    async fn test_member_can_access_member_route() {
        with_secret("rbac-guard-secret", || async {
            let app = create_test_router();
            let token = issue_token(&make_user(Role::Member));

            let (status, body) = get_with_bearer(app, "/member", &token).await;
            assert_eq!(status, 200);
            assert_eq!(body["ok"], true);
            assert_eq!(body["role"], "member");
        })
        .await;
    }

    #[tokio::test]
    async fn test_member_cannot_access_admin_route() {
        with_secret("rbac-guard-secret", || async {
            let app = create_test_router();
            let token = issue_token(&make_user(Role::Member));

            let (status, body) = get_with_bearer(app, "/admin", &token).await;
            assert_eq!(status, 403);
            assert_eq!(body["error"], "insufficient_role");
        })
        .await;
    }

    #[tokio::test]
    async fn test_admin_can_access_everything() {
        with_secret("rbac-guard-secret", || async {
            let app = create_test_router();
            let token = issue_token(&make_user(Role::Admin));

            let (member_status, member_body) = get_with_bearer(app.clone(), "/member", &token).await;
            assert_eq!(member_status, 200);
            assert_eq!(member_body["role"], "admin");

            let (admin_status, admin_body) = get_with_bearer(app, "/admin", &token).await;
            assert_eq!(admin_status, 200);
            assert_eq!(admin_body["role"], "admin");
        })
        .await;
    }
}