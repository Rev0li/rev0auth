-- AUTH-03 : un seul modèle utilisateur. Les comptes web_users rejoignent
-- ba_users (identité BetterAuth) avec leurs champs métier ; les mots de passe
-- restent dans ba_accounts, les avatars passent dans user_avatars.
-- Pseudo exact → name / display_username ; LOWER(pseudo) reste le username.

-- 1. Comptes encore absents côté BetterAuth (jamais connectés depuis la bascule)
INSERT INTO "ba_users" ("id", "name", "email", "email_verified", "username", "display_username", "role", "created_at", "updated_at")
SELECT gen_random_uuid()::text, w."pseudo", lower(w."pseudo") || '@local.invalid', false,
       lower(w."pseudo"), w."pseudo", w."role", to_timestamp(w."created_at_epoch")::timestamp, now()
FROM "web_users" w
WHERE NOT EXISTS (SELECT 1 FROM "ba_users" b WHERE b."username" = lower(w."pseudo"));
--> statement-breakpoint

-- 2. Champs métier, rôle, pseudo affiché et date de création d'origine
UPDATE "ba_users" b SET
    "name"             = w."pseudo",
    "display_username" = w."pseudo",
    "role"             = w."role",
    "active"           = w."active",
    "approved"         = w."approved",
    "bio"              = w."bio",
    "access_jellyfin"  = w."access_jellyfin",
    "access_songsurf"  = w."access_songsurf",
    "request_jellyfin" = w."request_jellyfin",
    "request_songsurf" = w."request_songsurf",
    "github_username"  = w."github_username",
    "linkedin_name"    = w."linkedin_name",
    "created_at"       = to_timestamp(w."created_at_epoch")::timestamp
FROM "web_users" w
WHERE b."username" = lower(w."pseudo");
--> statement-breakpoint

-- 3. Mot de passe : créé depuis web_users pour les comptes sans credential
INSERT INTO "ba_accounts" ("id", "account_id", "provider_id", "user_id", "password", "created_at", "updated_at")
SELECT gen_random_uuid()::text, b."id", 'credential', b."id", w."password_hash", now(), now()
FROM "web_users" w
JOIN "ba_users" b ON b."username" = lower(w."pseudo")
WHERE w."password_hash" <> ''
  AND NOT EXISTS (SELECT 1 FROM "ba_accounts" a WHERE a."user_id" = b."id" AND a."provider_id" = 'credential');
--> statement-breakpoint

-- 4. Avatars
INSERT INTO "user_avatars" ("user_id", "bytes", "mime_type", "filename", "size_bytes")
SELECT b."id", w."avatar_bytes", coalesce(w."avatar_mime_type", 'image/svg+xml'), w."avatar_filename", w."avatar_size_bytes"
FROM "web_users" w
JOIN "ba_users" b ON b."username" = lower(w."pseudo")
WHERE w."avatar_bytes" IS NOT NULL
ON CONFLICT ("user_id") DO NOTHING;
--> statement-breakpoint

-- 5. Garde-fou : tout compte web_users doit avoir son compte ba_users, sinon
--    la transaction des migrations est annulée et rien n'est supprimé.
DO $$
DECLARE manquants integer;
BEGIN
    SELECT count(*) INTO manquants FROM "web_users" w
    WHERE NOT EXISTS (SELECT 1 FROM "ba_users" b WHERE b."username" = lower(w."pseudo"));
    IF manquants > 0 THEN
        RAISE EXCEPTION 'AUTH-03 : % compte(s) web_users sans compte ba_users', manquants;
    END IF;
END $$;
