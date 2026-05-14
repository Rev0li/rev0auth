import { db } from './db/index.js';
import { sessions } from './db/schema.js';
import { generateToken } from './auth.js';
import { eq, and, gt } from 'drizzle-orm';
import type { Session } from './db/schema.js';

const MEMBER_TTL_MS = 24 * 60 * 60 * 1000; // 24h
const ADMIN_TTL_MS  =  8 * 60 * 60 * 1000; // 8h

export const ADMIN_COOKIE  = 'rev0auth_admin_session';
export const MEMBER_COOKIE = 'rev0auth_member_session';

export async function createSession(pseudo: string, kind: 'admin' | 'member'): Promise<string> {
    const token     = generateToken(32);
    const ttl       = kind === 'admin' ? ADMIN_TTL_MS : MEMBER_TTL_MS;
    const expiresAt = new Date(Date.now() + ttl);

    await db.insert(sessions).values({ token, pseudo, kind, expiresAt });
    return token;
}

export async function getSession(token: string, kind: 'admin' | 'member'): Promise<Session | null> {
    const now = new Date();
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

export function sessionCookieOpts(ttlMs: number) {
    return {
        httpOnly: true,
        sameSite: 'lax' as const,
        path: '/',
        maxAge: ttlMs / 1000,
        secure: process.env.NODE_ENV === 'production',
    };
}

export const ADMIN_COOKIE_OPTS  = sessionCookieOpts(ADMIN_TTL_MS);
export const MEMBER_COOKIE_OPTS = sessionCookieOpts(MEMBER_TTL_MS);
