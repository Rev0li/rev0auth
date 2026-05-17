ALTER TABLE web_users ADD COLUMN IF NOT EXISTS approved BOOLEAN NOT NULL DEFAULT false;
-- Grandfather existing users: already trusted since they were added manually
UPDATE web_users SET approved = true;
