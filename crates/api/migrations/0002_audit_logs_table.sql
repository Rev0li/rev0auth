CREATE TABLE IF NOT EXISTS auth_audit_logs (
    id BIGSERIAL PRIMARY KEY,
    timestamp_epoch BIGINT NOT NULL,
    user_id UUID REFERENCES auth_users(id) ON DELETE SET NULL,
    event_type TEXT NOT NULL,
    ip TEXT NOT NULL DEFAULT 'unknown',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_auth_audit_logs_created_at
ON auth_audit_logs(created_at DESC);
