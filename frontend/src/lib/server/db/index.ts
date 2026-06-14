import postgres from 'postgres';
import { drizzle } from 'drizzle-orm/postgres-js';
import * as schema from './schema.js';
import * as authSchema from './auth-schema.js';

const url = process.env.DATABASE_URL ?? 'postgres://postgres:postgres@localhost:5432/rev0auth';

const client = postgres(url);

// authSchema (ba_*) : tables BetterAuth — le drizzleAdapter les résout par nom dans ce schema
export const db = drizzle(client, { schema: { ...schema, ...authSchema } });

export async function initDb() {
    // ── Tables métier (web_*) ────────────────────────────────────────────────
    // Historiquement créées par les migrations Rust sqlx (0004/0005/0006), désormais
    // retirées. Portées ici pour que SvelteKit soit autonome (DB fraîche / DR / dev).
    // Idempotent : no-op sur une DB existante. Schéma = miroir exact de schema.ts.
    await client`
        CREATE TABLE IF NOT EXISTS web_users (
            pseudo              TEXT PRIMARY KEY,
            role                TEXT NOT NULL DEFAULT 'member',
            active              BOOLEAN NOT NULL DEFAULT true,
            status              TEXT NOT NULL DEFAULT 'offline',
            bio                 TEXT NOT NULL DEFAULT '',
            access_jellyfin     BOOLEAN NOT NULL DEFAULT false,
            access_songsurf     BOOLEAN NOT NULL DEFAULT false,
            request_jellyfin    BOOLEAN NOT NULL DEFAULT false,
            request_songsurf    BOOLEAN NOT NULL DEFAULT false,
            github_username     TEXT,
            linkedin_name       TEXT,
            avatar_filename     TEXT,
            avatar_size_bytes   BIGINT,
            avatar_mime_type    TEXT,
            avatar_bytes        BYTEA,
            password_hash       TEXT NOT NULL DEFAULT '',
            created_at_epoch    BIGINT NOT NULL DEFAULT EXTRACT(EPOCH FROM NOW())::BIGINT,
            approved            BOOLEAN NOT NULL DEFAULT false
        )
    `;
    // Colonnes mortes retirées (github jamais request/access ; commentary inutilisé).
    // DROP IF EXISTS : nettoie les DB de prod existantes, no-op sur une DB fraîche.
    await client`ALTER TABLE web_users DROP COLUMN IF EXISTS commentary`;
    await client`ALTER TABLE web_users DROP COLUMN IF EXISTS access_github`;
    await client`ALTER TABLE web_users DROP COLUMN IF EXISTS request_github`;
    await client`ALTER TABLE web_users DROP COLUMN IF EXISTS github_star_claimed`;

    await client`
        CREATE TABLE IF NOT EXISTS web_messages (
            id               BIGSERIAL PRIMARY KEY,
            from_pseudo      TEXT NOT NULL,
            to_pseudo        TEXT NOT NULL,
            body             TEXT NOT NULL,
            is_read          BOOLEAN NOT NULL DEFAULT false,
            created_at_epoch BIGINT NOT NULL DEFAULT EXTRACT(EPOCH FROM NOW())::BIGINT
        )
    `;
    await client`
        CREATE TABLE IF NOT EXISTS web_donations (
            id               BIGSERIAL PRIMARY KEY,
            pseudo           TEXT NOT NULL,
            method           TEXT NOT NULL,
            code             TEXT NOT NULL DEFAULT '',
            reviewed         BOOLEAN NOT NULL DEFAULT false,
            approved         BOOLEAN NOT NULL DEFAULT false,
            created_at_epoch BIGINT NOT NULL DEFAULT EXTRACT(EPOCH FROM NOW())::BIGINT
        )
    `;
    await client`
        CREATE TABLE IF NOT EXISTS web_wall_posts (
            id               BIGSERIAL PRIMARY KEY,
            pseudo           TEXT NOT NULL,
            body             TEXT NOT NULL,
            created_at_epoch BIGINT NOT NULL DEFAULT EXTRACT(EPOCH FROM NOW())::BIGINT
        )
    `;
    await client`
        CREATE TABLE IF NOT EXISTS web_invites (
            id               BIGSERIAL PRIMARY KEY,
            code             TEXT NOT NULL UNIQUE,
            note             TEXT NOT NULL DEFAULT '',
            created_at_epoch BIGINT NOT NULL,
            expires_at_epoch BIGINT NOT NULL,
            used_by          TEXT,
            used_at_epoch    BIGINT
        )
    `;

    await client`
        CREATE TABLE IF NOT EXISTS web_sessions (
            token       TEXT PRIMARY KEY,
            pseudo      TEXT NOT NULL,
            kind        TEXT NOT NULL,
            expires_at  BIGINT NOT NULL,
            created_at  BIGINT NOT NULL DEFAULT EXTRACT(EPOCH FROM NOW())::BIGINT
        )
    `;

    await client`
        CREATE TABLE IF NOT EXISTS web_audit_log (
            id               BIGSERIAL PRIMARY KEY,
            action           TEXT NOT NULL,
            actor_pseudo     TEXT NOT NULL,
            target           TEXT NOT NULL DEFAULT '',
            detail           TEXT NOT NULL DEFAULT '',
            created_at_epoch BIGINT NOT NULL DEFAULT EXTRACT(EPOCH FROM NOW())::BIGINT
        )
    `;
    await client`CREATE INDEX IF NOT EXISTS web_audit_log_created_idx ON web_audit_log (created_at_epoch DESC)`;

    await client`
        CREATE TABLE IF NOT EXISTS songsurf_events (
            id                BIGSERIAL PRIMARY KEY,
            source            TEXT NOT NULL DEFAULT '',
            event_type        TEXT NOT NULL,
            event_ts_epoch    BIGINT NOT NULL,
            pseudo            TEXT NOT NULL DEFAULT '',
            role              TEXT NOT NULL DEFAULT '',
            artist            TEXT NOT NULL DEFAULT '',
            album             TEXT NOT NULL DEFAULT '',
            title             TEXT NOT NULL DEFAULT '',
            detail            TEXT NOT NULL DEFAULT '{}',
            ip                TEXT NOT NULL DEFAULT '',
            received_at_epoch BIGINT NOT NULL DEFAULT EXTRACT(EPOCH FROM NOW())::BIGINT
        )
    `;
    await client`CREATE INDEX IF NOT EXISTS songsurf_events_received_idx ON songsurf_events (received_at_epoch DESC)`;
    await client`CREATE INDEX IF NOT EXISTS songsurf_events_pseudo_idx   ON songsurf_events (pseudo, received_at_epoch DESC)`;
    await client`CREATE INDEX IF NOT EXISTS songsurf_events_type_idx     ON songsurf_events (event_type, received_at_epoch DESC)`;

    // ── Tables BetterAuth (Phase 2) — miroir exact de auth-schema.ts ─────────
    await client`
        CREATE TABLE IF NOT EXISTS ba_users (
            id               TEXT PRIMARY KEY,
            name             TEXT NOT NULL,
            email            TEXT NOT NULL UNIQUE,
            email_verified   BOOLEAN NOT NULL DEFAULT FALSE,
            image            TEXT,
            created_at       TIMESTAMP NOT NULL DEFAULT NOW(),
            updated_at       TIMESTAMP NOT NULL DEFAULT NOW(),
            username         TEXT UNIQUE,
            display_username TEXT,
            role             TEXT DEFAULT 'member'
        )
    `;
    await client`
        CREATE TABLE IF NOT EXISTS ba_sessions (
            id         TEXT PRIMARY KEY,
            expires_at TIMESTAMP NOT NULL,
            token      TEXT NOT NULL UNIQUE,
            created_at TIMESTAMP NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
            ip_address TEXT,
            user_agent TEXT,
            user_id    TEXT NOT NULL REFERENCES ba_users(id) ON DELETE CASCADE
        )
    `;
    await client`CREATE INDEX IF NOT EXISTS "ba_sessions_userId_idx" ON ba_sessions (user_id)`;
    await client`
        CREATE TABLE IF NOT EXISTS ba_accounts (
            id                       TEXT PRIMARY KEY,
            account_id               TEXT NOT NULL,
            provider_id              TEXT NOT NULL,
            user_id                  TEXT NOT NULL REFERENCES ba_users(id) ON DELETE CASCADE,
            access_token             TEXT,
            refresh_token            TEXT,
            id_token                 TEXT,
            access_token_expires_at  TIMESTAMP,
            refresh_token_expires_at TIMESTAMP,
            scope                    TEXT,
            password                 TEXT,
            created_at               TIMESTAMP NOT NULL DEFAULT NOW(),
            updated_at               TIMESTAMP NOT NULL DEFAULT NOW()
        )
    `;
    await client`CREATE INDEX IF NOT EXISTS "ba_accounts_userId_idx" ON ba_accounts (user_id)`;
    await client`
        CREATE TABLE IF NOT EXISTS ba_verifications (
            id         TEXT PRIMARY KEY,
            identifier TEXT NOT NULL,
            value      TEXT NOT NULL,
            expires_at TIMESTAMP NOT NULL,
            created_at TIMESTAMP NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMP NOT NULL DEFAULT NOW()
        )
    `;
    await client`CREATE INDEX IF NOT EXISTS "ba_verifications_identifier_idx" ON ba_verifications (identifier)`;
}
