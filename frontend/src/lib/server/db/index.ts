import postgres from 'postgres';
import { drizzle } from 'drizzle-orm/postgres-js';
import { migrate } from 'drizzle-orm/postgres-js/migrator';
import * as schema from './schema.js';
import * as authSchema from './auth-schema.js';

const url = process.env.DATABASE_URL ?? 'postgres://postgres:postgres@localhost:5432/rev0auth';

// onnotice muet : les « already exists, skipping » des migrations idempotentes
// polluaient les logs à chaque démarrage.
const client = postgres(url, { onnotice: () => {} });

// authSchema (ba_*) : tables BetterAuth — le drizzleAdapter les résout par nom dans ce schema
export const db = drizzle(client, { schema: { ...schema, ...authSchema } });

// Applique les migrations Drizzle (dossier drizzle/) au démarrage. Le journal
// des migrations appliquées vit dans drizzle.__drizzle_migrations.
export async function migrateDb() {
    await migrate(db, { migrationsFolder: process.env.MIGRATIONS_DIR ?? 'drizzle' });
}
