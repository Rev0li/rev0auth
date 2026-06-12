import { db } from './db/index.js';
import { sessions } from './db/schema.js';
import { generateToken } from './auth.js';
import { eq, and, gt } from 'drizzle-orm';
import type { Session } from './db/schema.js';

const MEMBER_TTL_S = 24 * 60 * 60;
const ADMIN_TTL_S  =  8 * 60 * 60;

export const ADMIN_COOKIE  = 'rev0auth_admin_session';
// Legacy : les sessions membres sont passées sur BetterAuth (ba_sessions).
// Ce cookie n'est plus émis — il ne sert qu'au nettoyage des sessions
// d'avant la bascule (logout, suppression de compte).
export const MEMBER_COOKIE = 'rev0auth_member_session';

export async function createSession(pseudo: string, kind: 'admin' | 'member'): Promise<string> {
    const token     = generateToken(32);
    const ttl       = kind === 'admin' ? ADMIN_TTL_S : MEMBER_TTL_S;
    const expiresAt = Math.floor(Date.now() / 1000) + ttl;

    await db.insert(sessions).values({ token, pseudo, kind, expiresAt });
    return token;
}

export async function getSession(token: string, kind: 'admin' | 'member'): Promise<Session | null> {
    const now = Math.floor(Date.now() / 1000);
    const rows = await db
        .select()
        .from(sessions)
        .where(and(eq(sessions.token, token), eq(sessions.kind, kind), gt(sessions.expiresAt, now)))
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
