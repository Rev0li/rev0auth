use axum::http::{header, HeaderMap, HeaderValue};
use rand::{distributions::Alphanumeric, Rng};

pub const ACCESS_COOKIE_NAME: &str = "access_token";
pub const REFRESH_COOKIE_NAME: &str = "refresh_token";
pub const CSRF_COOKIE_NAME: &str = "csrf_token";

// Dev note: shared parser used by auth handlers/tests to read a named cookie value.
// Attached to: /auth/refresh input path and test helpers.
pub fn extract_cookie(cookie_header: &str, cookie_name: &str) -> Option<String> {
    cookie_header
        .split(';')
        .map(|part| part.trim())
        .find_map(|pair| {
            let (name, value) = pair.split_once('=')?;
            if name == cookie_name {
                Some(value.to_string())
            } else {
                None
            }
        })
}

// Dev note: convenience wrapper for request headers.
// Attached to: refresh handler so route code stays focused on auth logic.
pub fn extract_cookie_from_headers(headers: &HeaderMap, cookie_name: &str) -> Option<String> {
    let raw = headers.get(header::COOKIE)?.to_str().ok()?;
    extract_cookie(raw, cookie_name)
}

// Dev note: access cookie policy for browser auth sessions.
// Attached to: /auth/login and /auth/refresh responses.
pub fn build_access_cookie(access_token: &str) -> String {
    format!(
        "{}={}; HttpOnly; Secure; SameSite=Lax; Path=/; Max-Age=900",
        ACCESS_COOKIE_NAME, access_token
    )
}

// Dev note: refresh cookie policy with dynamic lifetime from token service.
// Attached to: /auth/login and /auth/refresh responses.
pub fn build_refresh_cookie(refresh_token: &str, max_age_secs: u64) -> String {
    format!(
        "{}={}; HttpOnly; Secure; SameSite=Lax; Path=/; Max-Age={}",
        REFRESH_COOKIE_NAME, refresh_token, max_age_secs
    )
}

// Dev note: csrf cookie is readable by JS (no HttpOnly) so client can mirror it in header later.
// Attached to: frontend mutation calls protected by CSRF checks (next ticket).
pub fn build_csrf_cookie(csrf_token: &str) -> String {
    format!(
        "{}={}; Secure; SameSite=Lax; Path=/; Max-Age=900",
        CSRF_COOKIE_NAME, csrf_token
    )
}

// Dev note: central builder so all auth responses use consistent cookie flags.
// Attached to: login/refresh route outputs.
pub fn build_auth_set_cookie_headers(
    access_token: &str,
    refresh_token: &str,
    refresh_max_age_secs: u64,
    csrf_token: &str,
) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.append(
        header::SET_COOKIE,
        HeaderValue::from_str(&build_access_cookie(access_token)).expect("valid access cookie"),
    );
    headers.append(
        header::SET_COOKIE,
        HeaderValue::from_str(&build_refresh_cookie(refresh_token, refresh_max_age_secs))
            .expect("valid refresh cookie"),
    );
    headers.append(
        header::SET_COOKIE,
        HeaderValue::from_str(&build_csrf_cookie(csrf_token)).expect("valid csrf cookie"),
    );
    headers
}

// Dev note: stateless CSRF nonce for browser session boundary.
// Attached to: login and refresh response body/cookie.
pub fn generate_csrf_token() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{extract_cookie, ACCESS_COOKIE_NAME};

    #[test]
    fn extract_cookie_returns_value_when_present() {
        let raw = "a=1; access_token=abc123; x=y";
        assert_eq!(extract_cookie(raw, ACCESS_COOKIE_NAME).as_deref(), Some("abc123"));
    }
}