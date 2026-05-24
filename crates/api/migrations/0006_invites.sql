-- Migration 0006: replace signup requests with invite-only system

DROP TABLE IF EXISTS web_signup_requests;

CREATE TABLE IF NOT EXISTS web_invites (
    id BIGSERIAL PRIMARY KEY,
    code TEXT NOT NULL UNIQUE,
    note TEXT NOT NULL DEFAULT '',
    created_at_epoch BIGINT NOT NULL,
    expires_at_epoch BIGINT NOT NULL,
    used_by TEXT,
    used_at_epoch BIGINT
);

ALTER TABLE web_users DROP COLUMN IF EXISTS must_change_password;
