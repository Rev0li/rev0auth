import { describe, it, expect, beforeAll, afterAll } from 'vitest';
import Database from 'better-sqlite3';
import { drizzle } from 'drizzle-orm/better-sqlite3';
import { migrate } from 'drizzle-orm/better-sqlite3/migrator';
import * as schema from '../db/schema.js';
import { users, sessions } from '../db/schema.js';
import { eq } from 'drizzle-orm';
import { generateToken } from '../auth.js';

// In-memory SQLite for tests — no file I/O
const sqlite = new Database(':memory:');
sqlite.pragma('foreign_keys = ON');
const db = drizzle(sqlite, { schema });
migrate(db, { migrationsFolder: './drizzle' });

const TEST_PSEUDO = 'testuser_session';
const TEST_TOKEN  = generateToken(32);

beforeAll(async () => {
    await db.insert(users).values({
        pseudo:       TEST_PSEUDO,
        role:         'member',
        active:       true,
        passwordHash: 'hash-placeholder',
        mustChangePassword: false,
    });
});

afterAll(() => {
    sqlite.close();
});

describe('sessions table', () => {
    it('inserts a session and retrieves it', async () => {
        const expiresAt = new Date(Date.now() + 3600 * 1000);
        await db.insert(sessions).values({
            token:  TEST_TOKEN,
            pseudo: TEST_PSEUDO,
            kind:   'member',
            expiresAt,
        });

        const rows = await db.select().from(sessions).where(eq(sessions.token, TEST_TOKEN));
        expect(rows).toHaveLength(1);
        expect(rows[0].pseudo).toBe(TEST_PSEUDO);
        expect(rows[0].kind).toBe('member');
    });

    it('cascade-deletes sessions when user is deleted', async () => {
        await db.delete(users).where(eq(users.pseudo, TEST_PSEUDO));
        const rows = await db.select().from(sessions).where(eq(sessions.token, TEST_TOKEN));
        expect(rows).toHaveLength(0);
    });
});

describe('users table', () => {
    it('enforces unique pseudo constraint', async () => {
        await db.insert(users).values({ pseudo: 'unique_test', passwordHash: 'x', active: true });
        await expect(
            db.insert(users).values({ pseudo: 'unique_test', passwordHash: 'y', active: true })
        ).rejects.toThrow();
    });

    it('defaults role to guest', async () => {
        await db.insert(users).values({ pseudo: 'default_role_test', passwordHash: 'x', active: true });
        const rows = await db.select().from(users).where(eq(users.pseudo, 'default_role_test'));
        expect(rows[0].role).toBe('guest');
    });

    it('stores and retrieves status correctly', async () => {
        await db.insert(users).values({ pseudo: 'status_test', passwordHash: 'x', active: true, status: 'occupe' });
        const rows = await db.select().from(users).where(eq(users.pseudo, 'status_test'));
        expect(rows[0].status).toBe('occupe');
    });
});
