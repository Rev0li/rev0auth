import postgres from 'postgres';
import { drizzle } from 'drizzle-orm/postgres-js';
import * as schema from './schema.js';
import * as authSchema from './auth-schema.js';

const url = process.env.DATABASE_URL ?? 'postgres://postgres:postgres@localhost:5432/rev0auth';

const client = postgres(url);

// authSchema (ba_*) : tables BetterAuth — le drizzleAdapter les résout par nom dans ce schema
export const db = drizzle(client, { schema: { ...schema, ...authSchema } });

export async function initDb() {
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
