use axum::http::HeaderMap;
use serde::Serialize;
use uuid::Uuid;

use crate::auth::jwt::now_epoch;

// Dev note: canonical audit payload for auth security events.
// Attached to: signup/login/refresh/failed-auth telemetry and admin audit endpoint.
#[derive(Debug, Clone, Serialize)]
pub struct AuditEvent {
    pub timestamp: u64,
    pub user_id: Option<Uuid>,
    pub event_type: AuditEventType,
    pub ip: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditEventType {
    CreateUser,
    Login,
    Refresh,
    FailedAuth,
}

pub fn new_event(user_id: Option<Uuid>, event_type: AuditEventType, ip: String) -> AuditEvent {
    AuditEvent {
        timestamp: now_epoch(),
        user_id,
        event_type,
        ip,
    }
}

pub fn client_ip_from_headers(headers: &HeaderMap) -> String {
    headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .and_then(|raw| raw.split(',').next())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .or_else(|| {
            headers
                .get("x-real-ip")
                .and_then(|v| v.to_str().ok())
                .map(str::trim)
                .filter(|s| !s.is_empty())
        })
        .unwrap_or("unknown")
        .to_string()
}
