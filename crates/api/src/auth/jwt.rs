use crate::auth::models::{Role, User};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct TokenService {
    encoding_key: EncodingKey,
    access_ttl_secs: u64,
    refresh_ttl_secs: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessClaims {
    pub sub: String,
    pub email: String,
    pub role: Role,
    pub token_type: String,
    pub iat: u64,
    pub exp: u64,
}

impl TokenService {
    pub fn from_env_required() -> anyhow::Result<Self> {
        let secret = std::env::var("AUTH_JWT_SECRET")
            .map_err(|_| anyhow::anyhow!("missing AUTH_JWT_SECRET"))?;
        if secret.trim().is_empty() {
            return Err(anyhow::anyhow!("missing AUTH_JWT_SECRET"));
        }

        Ok(Self::from_secret_and_ttls(secret))
    }

    pub fn from_env() -> Self {
        let secret = std::env::var("AUTH_JWT_SECRET")
            .unwrap_or_else(|_| "dev-only-secret-change-me".to_string());

        Self::from_secret_and_ttls(secret)
    }

    fn from_secret_and_ttls(secret: String) -> Self {

        let access_ttl_secs = std::env::var("AUTH_ACCESS_TTL_SECS")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(15 * 60);

        let refresh_ttl_secs = std::env::var("AUTH_REFRESH_TTL_SECS")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(7 * 24 * 60 * 60);

        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            access_ttl_secs,
            refresh_ttl_secs,
        }
    }

    pub fn issue_access_token(&self, user: &User) -> Result<String, String> {
        let now = now_epoch();
        let claims = AccessClaims {
            sub: user.id.to_string(),
            email: user.email.clone(),
            role: user.role.clone(),
            token_type: "access".to_string(),
            iat: now,
            exp: now + self.access_ttl_secs,
        };

        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|_| "token_issue_failed".to_string())
    }

    pub fn refresh_ttl_secs(&self) -> u64 {
        self.refresh_ttl_secs
    }
}

pub fn now_epoch() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::{AccessClaims, TokenService};
    use crate::auth::models::{Role, User};
    use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
    use std::sync::Mutex;
    use uuid::Uuid;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    fn test_user() -> User {
        User {
            id: Uuid::new_v4(),
            email: "member@example.com".to_string(),
            password_hash: "unused-in-jwt-tests".to_string(),
            role: Role::Member,
        }
    }

    fn decode_access_claims(token: &str, secret: &str) -> AccessClaims {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;

        decode::<AccessClaims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &validation,
        )
        .expect("token should decode")
        .claims
    }

    fn with_auth_jwt_secret<F, T>(secret: Option<&str>, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        let _guard = ENV_LOCK.lock().expect("env mutex poisoned");
        let previous = std::env::var("AUTH_JWT_SECRET").ok();

        match secret {
            Some(value) => unsafe {
                std::env::set_var("AUTH_JWT_SECRET", value);
            },
            None => unsafe {
                std::env::remove_var("AUTH_JWT_SECRET");
            },
        }

        let result = f();

        match previous {
            Some(value) => unsafe {
                std::env::set_var("AUTH_JWT_SECRET", value);
            },
            None => unsafe {
                std::env::remove_var("AUTH_JWT_SECRET");
            },
        }

        result
    }

    #[test]
    fn test_token_service_from_env_uses_secret() {
        with_auth_jwt_secret(Some("jwt-test-secret"), || {
            let service = TokenService::from_env();
            let user = test_user();
            let token = service
                .issue_access_token(&user)
                .expect("token issuance should succeed");

            let claims = decode_access_claims(&token, "jwt-test-secret");
            assert_eq!(claims.email, user.email);

            let wrong_decode = decode::<AccessClaims>(
                &token,
                &DecodingKey::from_secret("wrong-secret".as_bytes()),
                &Validation::new(Algorithm::HS256),
            );
            assert!(wrong_decode.is_err());
        });
    }

    #[test]
    fn test_issue_access_token_has_correct_claims() {
        with_auth_jwt_secret(Some("jwt-claims-secret"), || {
            let service = TokenService::from_env();
            let user = test_user();
            let token = service
                .issue_access_token(&user)
                .expect("token issuance should succeed");

            let claims = decode_access_claims(&token, "jwt-claims-secret");
            assert_eq!(claims.sub, user.id.to_string());
            assert_eq!(claims.email, user.email);
            assert_eq!(claims.role, Role::Member);
            assert_eq!(claims.token_type, "access");
        });
    }

    #[test]
    fn test_issue_access_token_expiration() {
        with_auth_jwt_secret(Some("jwt-exp-secret"), || {
            let service = TokenService::from_env();
            let user = test_user();
            let token = service
                .issue_access_token(&user)
                .expect("token issuance should succeed");

            let claims = decode_access_claims(&token, "jwt-exp-secret");
            assert!(claims.exp > claims.iat);
            assert_eq!(claims.exp - claims.iat, 15 * 60);
        });
    }

    #[test]
    fn test_issue_access_token_serializes_to_jwt() {
        with_auth_jwt_secret(Some("jwt-format-secret"), || {
            let service = TokenService::from_env();
            let user = test_user();
            let token = service
                .issue_access_token(&user)
                .expect("token issuance should succeed");

            let parts: Vec<&str> = token.split('.').collect();
            assert_eq!(parts.len(), 3);
            assert!(parts.iter().all(|segment| !segment.is_empty()));
        });
    }
}
