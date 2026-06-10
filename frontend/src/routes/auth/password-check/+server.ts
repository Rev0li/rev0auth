import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { db } from '$lib/server/db/index.js';
import { users } from '$lib/server/db/schema.js';
import { sql } from 'drizzle-orm';
import { verifyPassword } from '$lib/server/auth.js';
import { createSession, MEMBER_COOKIE, MEMBER_COOKIE_OPTS } from '$lib/server/session.js';
import { checkRateLimit, recordFailure, clearAttempts, getIp } from '$lib/server/ratelimit.js';
import { songsurfConfigured, songsurfBaseUrl, signSongsurfJwt } from '$lib/server/songsurf.js';

async function buildSongsurfUrl(pseudo: string, role: string): Promise<string | null> {
    if (!songsurfConfigured()) return null;
    const token = await signSongsurfJwt(pseudo, role, 8 * 3600);
    return `${songsurfBaseUrl()}?token=${token}`;
}

export const POST: RequestHandler = async ({ request, cookies }) => {
    const ip = getIp(request);
    if (checkRateLimit(ip).blocked) {
        return json({ ok: false, state: 'invalid', message: 'Trop de tentatives, réessaie dans 15 minutes.' }, { status: 429 });
    }

    const { pseudo, password } = await request.json();
    const key = pseudo?.trim()?.toLowerCase();
    if (!key || !password?.trim()) {
        return json({ ok: false, state: 'invalid', message: 'Champs requis.' });
    }

    const rows = await db
        .select()
        .from(users)
        .where(sql`LOWER(${users.pseudo}) = ${key}`)
        .limit(1);

    const user = rows[0];
    if (!user || !user.active) {
        recordFailure(ip);
        return json({ ok: false, state: 'invalid', message: 'Mot de passe incorrect.' });
    }

    const valid = await verifyPassword(password, user.passwordHash);
    if (!valid) {
        recordFailure(ip);
        return json({ ok: false, state: 'invalid', message: 'Mot de passe incorrect.' });
    }

    clearAttempts(ip);
    const token = await createSession(user.pseudo, 'member');
    cookies.set(MEMBER_COOKIE, token, MEMBER_COOKIE_OPTS);

    const songsurfUrl = user.accessSongsurf
        ? await buildSongsurfUrl(user.pseudo, user.role)
        : null;

    return json({
        ok: true,
        state: 'ok',
        message: 'Connexion autorisée.',
        pseudo: user.pseudo,
        songsurf_url: songsurfUrl,
    });
};
