import postgres from 'postgres';
import { drizzle } from 'drizzle-orm/postgres-js';
import * as schema from './schema.js';

const url = process.env.DATABASE_URL ?? 'postgres://postgres:postgres@localhost:5432/rev0auth';

const client = postgres(url);

export const db = drizzle(client, { schema });

// Create SvelteKit-specific tables that don't exist in Rust web crate
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
    CREATE TABLE IF NOT EXISTS web_test_runs (
        run_id      TEXT PRIMARY KEY,
        executed_at BIGINT NOT NULL,
        passed      BIGINT NOT NULL,
        total       BIGINT NOT NULL,
        cases       TEXT NOT NULL DEFAULT '[]'
    )
`;
