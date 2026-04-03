use crate::auth::jwt::now_epoch;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

// Dev note: in-memory login throttle state keyed by normalized email.
// Attached to: /auth/login brute-force mitigation path.
#[derive(Debug, Clone)]
pub struct LoginRateLimiter {
    entries: Arc<RwLock<HashMap<String, LoginAttemptState>>>,
}

#[derive(Debug, Clone)]
struct LoginAttemptState {
    failed_attempts: u8,
    lock_level: u8,
    lock_until_epoch: u64,
}

impl LoginRateLimiter {
    pub fn new() -> Self {
        Self {
            entries: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    // Dev note: checks active lock window before any password verification work.
    // Returns remaining lock duration in seconds when user is blocked.
    pub async fn check_allowed(&self, email: &str) -> Result<(), u64> {
        let now = now_epoch();
        let mut entries = self.entries.write().await;

        if let Some(state) = entries.get_mut(email) {
            if state.lock_until_epoch > now {
                return Err(state.lock_until_epoch - now);
            }

            // Lock expired: keep escalation level for future cycles, clear active lock.
            state.lock_until_epoch = 0;
        }

        Ok(())
    }

    // Dev note: records a failed attempt and applies progressive lockout at threshold.
    // Lock tiers: 15m -> 1h -> 24h.
    pub async fn record_failure(&self, email: &str) -> Option<u64> {
        let now = now_epoch();
        let mut entries = self.entries.write().await;
        let state = entries
            .entry(email.to_string())
            .or_insert(LoginAttemptState {
                failed_attempts: 0,
                lock_level: 0,
                lock_until_epoch: 0,
            });

        // Ignore extra bookkeeping while lock is already active.
        if state.lock_until_epoch > now {
            return Some(state.lock_until_epoch - now);
        }

        state.failed_attempts = state.failed_attempts.saturating_add(1);

        if state.failed_attempts < 5 {
            return None;
        }

        state.failed_attempts = 0;
        state.lock_level = (state.lock_level + 1).min(3);

        let lock_seconds = match state.lock_level {
            1 => 15 * 60,
            2 => 60 * 60,
            _ => 24 * 60 * 60,
        };
        state.lock_until_epoch = now + lock_seconds;

        Some(lock_seconds)
    }

    // Dev note: successful login clears failures and lock progression for that account.
    pub async fn record_success(&self, email: &str) {
        let mut entries = self.entries.write().await;
        entries.remove(email);
    }
}
