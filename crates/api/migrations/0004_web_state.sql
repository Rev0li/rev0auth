CREATE TABLE IF NOT EXISTS web_users (
    pseudo TEXT PRIMARY KEY,
    role TEXT NOT NULL DEFAULT 'member',
    active BOOLEAN NOT NULL DEFAULT true,
    status TEXT NOT NULL DEFAULT 'offline',
    bio TEXT NOT NULL DEFAULT '',
    commentary TEXT NOT NULL DEFAULT '',
    access_github BOOLEAN NOT NULL DEFAULT false,
    access_jellyfin BOOLEAN NOT NULL DEFAULT false,
    access_songsurf BOOLEAN NOT NULL DEFAULT false,
    request_github BOOLEAN NOT NULL DEFAULT false,
    request_jellyfin BOOLEAN NOT NULL DEFAULT false,
    request_songsurf BOOLEAN NOT NULL DEFAULT false,
    github_star_claimed BOOLEAN NOT NULL DEFAULT false,
    github_username TEXT,
    linkedin_name TEXT,
    avatar_filename TEXT,
    avatar_size_bytes BIGINT,
    avatar_mime_type TEXT,
    avatar_bytes BYTEA,
    must_change_password BOOLEAN NOT NULL DEFAULT false,
    password_hash TEXT NOT NULL DEFAULT '',
    created_at_epoch BIGINT NOT NULL DEFAULT EXTRACT(EPOCH FROM NOW())::BIGINT
);

CREATE TABLE IF NOT EXISTS web_signup_requests (
    id BIGSERIAL PRIMARY KEY,
    pseudo TEXT NOT NULL,
    referral TEXT NOT NULL DEFAULT '',
    temp_password TEXT NOT NULL DEFAULT '',
    status TEXT NOT NULL DEFAULT 'pending',
    created_at_epoch BIGINT NOT NULL DEFAULT EXTRACT(EPOCH FROM NOW())::BIGINT
);

CREATE TABLE IF NOT EXISTS web_messages (
    id BIGSERIAL PRIMARY KEY,
    from_pseudo TEXT NOT NULL,
    to_pseudo TEXT NOT NULL,
    body TEXT NOT NULL,
    is_read BOOLEAN NOT NULL DEFAULT false,
    created_at_epoch BIGINT NOT NULL DEFAULT EXTRACT(EPOCH FROM NOW())::BIGINT
);

CREATE TABLE IF NOT EXISTS web_donations (
    id BIGSERIAL PRIMARY KEY,
    pseudo TEXT NOT NULL,
    method TEXT NOT NULL,
    code TEXT NOT NULL DEFAULT '',
    reviewed BOOLEAN NOT NULL DEFAULT false,
    approved BOOLEAN NOT NULL DEFAULT false,
    created_at_epoch BIGINT NOT NULL DEFAULT EXTRACT(EPOCH FROM NOW())::BIGINT
);

CREATE TABLE IF NOT EXISTS web_wall_posts (
    id BIGSERIAL PRIMARY KEY,
    pseudo TEXT NOT NULL,
    body TEXT NOT NULL,
    created_at_epoch BIGINT NOT NULL DEFAULT EXTRACT(EPOCH FROM NOW())::BIGINT
);
