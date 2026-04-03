ALTER TABLE auth_refresh_tokens
ADD COLUMN IF NOT EXISTS csrf_token TEXT NOT NULL DEFAULT '';

CREATE INDEX IF NOT EXISTS idx_auth_refresh_tokens_expires_at
ON auth_refresh_tokens(expires_at);

CREATE INDEX IF NOT EXISTS idx_auth_audit_logs_event_type_created_at
ON auth_audit_logs(event_type, created_at DESC);
