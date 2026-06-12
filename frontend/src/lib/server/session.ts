import { db } from './db/index.js';
import { sessions } from './db/schema.js';
import { generateToken } from './auth.js';
import { eq, and, gt } from 'drizzle-orm';
import type { Session } from './db/schema.js';

// Sessions custom admin uniquement — les sessions membres sont passées sur
// BetterAuth (ba_sessions). La colonne kind reste en DB pour les lignes legacy.
const ADMIN_TTL_S = 8 * 60 * 60;

export const ADMIN_COOKIE  = 'rev0auth_admin_session';
// Legacy : cookie membre d'avant la bascule BetterAuth. Plus émis — ne sert
// qu'au nettoyage des vieilles sessions (logout, suppression de compte).
export const MEMBER_COOKIE = 'rev0auth_member_session';

export async function createSession(pseudo: string): Promise<string> {
    const token     = generateToken(32);
    const expiresAt = Math.floor(Date.now() / 1000) + ADMIN_TTL_S;

    await db.insert(sessions).values({ token, pseudo, kind: 'admin', expiresAt });
    return token;
}

export async function getSession(token: string): Promise<Session | null> {
    const now = Math.floor(Date.now() / 1000);
    const rows = await db
        .select()
        .from(sessions)
        .where(and(eq(sessions.token, token), eq(sessions.kind, 'admin'), gt(sessions.expiresAt, now)))
        .limit(1);
    return rows[0] ?? null;
}

export async function deleteSession(token: string): Promise<void> {
    await db.delete(sessions).where(eq(sessions.token, token));
}

export const ADMIN_COOKIE_OPTS = {
    httpOnly: true,
    sameSite: 'lax' as const,
    path: '/',
    maxAge: ADMIN_TTL_S,
    secure: process.env.NODE_ENV === 'production',
};
