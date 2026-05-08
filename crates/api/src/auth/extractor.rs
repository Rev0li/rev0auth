use crate::auth::{
    jwt::AccessClaims,
    models::{ErrorResponse, Role},
};
use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    Json,
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct UserClaims {
    pub id: Uuid,
    pub email: String,
    pub role: Role,
}

#[axum::async_trait]
impl<S> FromRequestParts<S> for UserClaims
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<ErrorResponse>);

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .ok_or_else(|| err(StatusCode::UNAUTHORIZED, "missing_authorization_header"))?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or_else(|| err(StatusCode::UNAUTHORIZED, "invalid_authorization_scheme"))?;

        let secret = std::env::var("AUTH_JWT_SECRET")
            .unwrap_or_else(|_| "dev-only-secret-change-me".to_string());

        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;
        validation.leeway = 0;

        let decoded = decode::<AccessClaims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &validation,
        )
        .map_err(|_| err(StatusCode::UNAUTHORIZED, "invalid_or_expired_token"))?;

        let id = Uuid::parse_str(&decoded.claims.sub)
            .map_err(|_| err(StatusCode::UNAUTHORIZED, "invalid_token_subject"))?;

        Ok(UserClaims {
            id,
            email: decoded.claims.email,
            role: decoded.claims.role,
        })
    }
}

fn err(status: StatusCode, code: &'static str) -> (StatusCode, Json<ErrorResponse>) {
    (status, Json(ErrorResponse { error: code }))
}

#[cfg(test)]
mod tests {
    use super::UserClaims;
    use crate::auth::{
        jwt::{now_epoch, AccessClaims, TokenService},
        models::{Role, User},
    };
    use axum::{
        body::{to_bytes, Body},
        http::Request,
        routing::get,
        Json, Router,
    };
    use jsonwebtoken::{encode, EncodingKey, Header};
    use serde_json::{json, Value};
    use std::sync::Mutex;
    use std::time::Duration;
    use tower::ServiceExt;
    use uuid::Uuid;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    fn make_test_user() -> User {
        User {
            id: Uuid::new_v4(),
            email: "member@example.com".to_string(),
            password_hash: "unused".to_string(),
            role: Role::Member,
        }
    }

    fn create_test_router() -> Router {
        async fn protected(UserClaims { id, email, role }: UserClaims) -> Json<Value> {
            Json(json!({
                "id": id.to_string(),
                "email": email,
                "role": role,
            }))
        }

        Router::new().route("/protected", get(protected))
    }

    async fn send_get_with_auth(app: Router, token: Option<&str>) -> (u16, Value) {
        let mut builder = Request::get("/protected");
        if let Some(token) = token {
            builder = builder.header("authorization", format!("Bearer {token}"));
        }

        let response = app
            .oneshot(builder.body(Body::empty()).expect("request"))
            .await
            .expect("response");

        let status = response.status().as_u16();
        let body = to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("body bytes");
        let json = serde_json::from_slice::<Value>(&body).expect("json response");

        (status, json)
    }

    async fn with_env<F, Fut, T>(secret: &str, access_ttl: Option<&str>, f: F) -> T
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = T>,
    {
        let _guard = ENV_LOCK.lock().expect("env mutex poisoned");
        let prev_secret = std::env::var("AUTH_JWT_SECRET").ok();
        let prev_ttl = std::env::var("AUTH_ACCESS_TTL_SECS").ok();

        unsafe {
            std::env::set_var("AUTH_JWT_SECRET", secret);
        }
        match access_ttl {
            Some(value) => unsafe {
                std::env::set_var("AUTH_ACCESS_TTL_SECS", value);
            },
            None => unsafe {
                std::env::remove_var("AUTH_ACCESS_TTL_SECS");
            },
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

        match prev_ttl {
            Some(value) => unsafe {
                std::env::set_var("AUTH_ACCESS_TTL_SECS", value);
            },
            None => unsafe {
                std::env::remove_var("AUTH_ACCESS_TTL_SECS");
            },
        }

        out
    }

    fn issue_token_for_user(user: &User) -> String {
        TokenService::from_env()
            .issue_access_token(user)
            .expect("token")
    }

    #[tokio::test]
    async fn test_extract_valid_jwt() {
        with_env("extractor-valid-secret", None, || async {
            let app = create_test_router();
            let user = make_test_user();
            let token = issue_token_for_user(&user);

            let (status, body) = send_get_with_auth(app, Some(&token)).await;
            assert_eq!(status, 200);
            assert_eq!(body["email"], user.email);
            assert_eq!(body["role"], "member");
            assert_eq!(body["id"], user.id.to_string());
        })
        .await;
    }

    #[tokio::test]
    async fn test_extract_missing_header() {
        with_env("extractor-missing-secret", None, || async {
            let app = create_test_router();

            let (status, body) = send_get_with_auth(app, None).await;
            assert_eq!(status, 401);
            assert_eq!(body["error"], "missing_authorization_header");
        })
        .await;
    }

    #[tokio::test]
    async fn test_extract_invalid_signature() {
        with_env("extractor-secret", None, || async {
            let user = make_test_user();
            let claims = AccessClaims {
                sub: user.id.to_string(),
                email: user.email,
                role: user.role,
                token_type: "access".to_string(),
                iat: now_epoch(),
                exp: now_epoch() + 600,
            };
            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret("issuer-secret".as_bytes()),
            )
            .expect("token");

            let app = create_test_router();
            let (status, body) = send_get_with_auth(app, Some(&token)).await;
            assert_eq!(status, 401);
            assert_eq!(body["error"], "invalid_or_expired_token");
        })
        .await;
    }

    #[tokio::test]
    async fn test_extract_expired_token() {
        with_env("extractor-expired-secret", Some("0"), || async {
            let app = create_test_router();
            let user = make_test_user();
            let token = issue_token_for_user(&user);

            tokio::time::sleep(Duration::from_millis(1200)).await;

            let (status, body) = send_get_with_auth(app, Some(&token)).await;
            assert_eq!(status, 401);
            assert_eq!(body["error"], "invalid_or_expired_token");
        })
        .await;
    }
}