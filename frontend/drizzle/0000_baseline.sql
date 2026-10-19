-- Baseline : schéma au 2026-10-19 (ex-initDb()).
-- Idempotent : crée tout sur une base vide, et aligne une base existante
-- (créée par l'ancien initDb) sans toucher aux données.

-- Nettoyage hérité de initDb : colonnes mortes et tables sociales (AUTH-01)
ALTER TABLE IF EXISTS "web_users" DROP COLUMN IF EXISTS "commentary";--> statement-breakpoint
ALTER TABLE IF EXISTS "web_users" DROP COLUMN IF EXISTS "access_github";--> statement-breakpoint
ALTER TABLE IF EXISTS "web_users" DROP COLUMN IF EXISTS "request_github";--> statement-breakpoint
ALTER TABLE IF EXISTS "web_users" DROP COLUMN IF EXISTS "github_star_claimed";--> statement-breakpoint
DROP TABLE IF EXISTS "web_messages";--> statement-breakpoint
DROP TABLE IF EXISTS "web_donations";--> statement-breakpoint
DROP TABLE IF EXISTS "web_wall_posts";--> statement-breakpoint
CREATE TABLE IF NOT EXISTS "web_audit_log" (
	"id" bigserial PRIMARY KEY NOT NULL,
	"action" text NOT NULL,
	"actor_pseudo" text NOT NULL,
	"target" text DEFAULT '' NOT NULL,
	"detail" text DEFAULT '' NOT NULL,
	"created_at_epoch" bigint DEFAULT EXTRACT(EPOCH FROM NOW())::BIGINT NOT NULL
);
--> statement-breakpoint
CREATE TABLE IF NOT EXISTS "web_invites" (
	"id" bigserial PRIMARY KEY NOT NULL,
	"code" text NOT NULL,
	"note" text DEFAULT '' NOT NULL,
	"created_at_epoch" bigint NOT NULL,
	"expires_at_epoch" bigint NOT NULL,
	"used_by" text,
	"used_at_epoch" bigint,
	CONSTRAINT "web_invites_code_unique" UNIQUE("code")
);
--> statement-breakpoint
CREATE TABLE IF NOT EXISTS "web_sessions" (
	"token" text PRIMARY KEY NOT NULL,
	"pseudo" text NOT NULL,
	"kind" text NOT NULL,
	"expires_at" bigint NOT NULL,
	"created_at" bigint DEFAULT EXTRACT(EPOCH FROM NOW())::BIGINT NOT NULL
);
--> statement-breakpoint
CREATE TABLE IF NOT EXISTS "songsurf_events" (
	"id" bigserial PRIMARY KEY NOT NULL,
	"source" text DEFAULT '' NOT NULL,
	"event_type" text NOT NULL,
	"event_ts_epoch" bigint NOT NULL,
	"pseudo" text DEFAULT '' NOT NULL,
	"role" text DEFAULT '' NOT NULL,
	"artist" text DEFAULT '' NOT NULL,
	"album" text DEFAULT '' NOT NULL,
	"title" text DEFAULT '' NOT NULL,
	"detail" text DEFAULT '{}' NOT NULL,
	"ip" text DEFAULT '' NOT NULL,
	"received_at_epoch" bigint DEFAULT EXTRACT(EPOCH FROM NOW())::BIGINT NOT NULL
);
--> statement-breakpoint
CREATE TABLE IF NOT EXISTS "web_users" (
	"pseudo" text PRIMARY KEY NOT NULL,
	"role" text DEFAULT 'member' NOT NULL,
	"active" boolean DEFAULT true NOT NULL,
	"status" text DEFAULT 'offline' NOT NULL,
	"bio" text DEFAULT '' NOT NULL,
	"access_jellyfin" boolean DEFAULT false NOT NULL,
	"access_songsurf" boolean DEFAULT false NOT NULL,
	"request_jellyfin" boolean DEFAULT false NOT NULL,
	"request_songsurf" boolean DEFAULT false NOT NULL,
	"github_username" text,
	"linkedin_name" text,
	"avatar_filename" text,
	"avatar_size_bytes" bigint,
	"avatar_mime_type" text,
	"avatar_bytes" "bytea",
	"password_hash" text DEFAULT '' NOT NULL,
	"created_at_epoch" bigint DEFAULT EXTRACT(EPOCH FROM NOW())::BIGINT NOT NULL,
	"approved" boolean DEFAULT false NOT NULL
);
--> statement-breakpoint
CREATE TABLE IF NOT EXISTS "ba_accounts" (
	"id" text PRIMARY KEY NOT NULL,
	"account_id" text NOT NULL,
	"provider_id" text NOT NULL,
	"user_id" text NOT NULL,
	"access_token" text,
	"refresh_token" text,
	"id_token" text,
	"access_token_expires_at" timestamp,
	"refresh_token_expires_at" timestamp,
	"scope" text,
	"password" text,
	"created_at" timestamp DEFAULT now() NOT NULL,
	"updated_at" timestamp DEFAULT now() NOT NULL
);
--> statement-breakpoint
CREATE TABLE IF NOT EXISTS "ba_sessions" (
	"id" text PRIMARY KEY NOT NULL,
	"expires_at" timestamp NOT NULL,
	"token" text NOT NULL,
	"created_at" timestamp DEFAULT now() NOT NULL,
	"updated_at" timestamp DEFAULT now() NOT NULL,
	"ip_address" text,
	"user_agent" text,
	"user_id" text NOT NULL,
	CONSTRAINT "ba_sessions_token_unique" UNIQUE("token")
);
--> statement-breakpoint
CREATE TABLE IF NOT EXISTS "ba_users" (
	"id" text PRIMARY KEY NOT NULL,
	"name" text NOT NULL,
	"email" text NOT NULL,
	"email_verified" boolean DEFAULT false NOT NULL,
	"image" text,
	"created_at" timestamp DEFAULT now() NOT NULL,
	"updated_at" timestamp DEFAULT now() NOT NULL,
	"username" text,
	"display_username" text,
	"role" text DEFAULT 'member',
	CONSTRAINT "ba_users_email_unique" UNIQUE("email"),
	CONSTRAINT "ba_users_username_unique" UNIQUE("username")
);
--> statement-breakpoint
CREATE TABLE IF NOT EXISTS "ba_verifications" (
	"id" text PRIMARY KEY NOT NULL,
	"identifier" text NOT NULL,
	"value" text NOT NULL,
	"expires_at" timestamp NOT NULL,
	"created_at" timestamp DEFAULT now() NOT NULL,
	"updated_at" timestamp DEFAULT now() NOT NULL
);
--> statement-breakpoint
-- Contraintes : renomme les noms Postgres par défaut (ancien initDb) en noms Drizzle,
-- ou ajoute la contrainte si elle manque.
DO $$
DECLARE r record;
BEGIN
  FOR r IN SELECT * FROM (VALUES
    ('web_invites', 'web_invites_code_key',      'web_invites_code_unique'),
    ('ba_users',    'ba_users_email_key',        'ba_users_email_unique'),
    ('ba_users',    'ba_users_username_key',     'ba_users_username_unique'),
    ('ba_sessions', 'ba_sessions_token_key',     'ba_sessions_token_unique'),
    ('ba_sessions', 'ba_sessions_user_id_fkey',  'ba_sessions_user_id_ba_users_id_fk'),
    ('ba_accounts', 'ba_accounts_user_id_fkey',  'ba_accounts_user_id_ba_users_id_fk')
  ) AS t(tbl, old_name, new_name)
  LOOP
    IF EXISTS (SELECT 1 FROM pg_constraint WHERE conname = r.old_name) THEN
      EXECUTE format('ALTER TABLE %I RENAME CONSTRAINT %I TO %I', r.tbl, r.old_name, r.new_name);
    END IF;
  END LOOP;

  IF NOT EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'ba_accounts_user_id_ba_users_id_fk') THEN
    ALTER TABLE "ba_accounts" ADD CONSTRAINT "ba_accounts_user_id_ba_users_id_fk" FOREIGN KEY ("user_id") REFERENCES "public"."ba_users"("id") ON DELETE cascade ON UPDATE no action;
  END IF;
  IF NOT EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'ba_sessions_user_id_ba_users_id_fk') THEN
    ALTER TABLE "ba_sessions" ADD CONSTRAINT "ba_sessions_user_id_ba_users_id_fk" FOREIGN KEY ("user_id") REFERENCES "public"."ba_users"("id") ON DELETE cascade ON UPDATE no action;
  END IF;
END $$;--> statement-breakpoint
CREATE INDEX IF NOT EXISTS "web_audit_log_created_idx" ON "web_audit_log" USING btree ("created_at_epoch" DESC NULLS LAST);--> statement-breakpoint
CREATE INDEX IF NOT EXISTS "songsurf_events_received_idx" ON "songsurf_events" USING btree ("received_at_epoch" DESC NULLS LAST);--> statement-breakpoint
CREATE INDEX IF NOT EXISTS "songsurf_events_pseudo_idx" ON "songsurf_events" USING btree ("pseudo","received_at_epoch" DESC NULLS LAST);--> statement-breakpoint
CREATE INDEX IF NOT EXISTS "songsurf_events_type_idx" ON "songsurf_events" USING btree ("event_type","received_at_epoch" DESC NULLS LAST);--> statement-breakpoint
CREATE INDEX IF NOT EXISTS "ba_accounts_userId_idx" ON "ba_accounts" USING btree ("user_id");--> statement-breakpoint
CREATE INDEX IF NOT EXISTS "ba_sessions_userId_idx" ON "ba_sessions" USING btree ("user_id");--> statement-breakpoint
CREATE INDEX IF NOT EXISTS "ba_verifications_identifier_idx" ON "ba_verifications" USING btree ("identifier");