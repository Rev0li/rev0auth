import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { db } from '$lib/server/db/index.js';
import { users } from '$lib/server/db/schema.js';
import { sql } from 'drizzle-orm';
import { auth } from '$lib/server/auth-v2.js';
import { setBaPassword } from '$lib/server/ba-sync.js';
import { verifyPassword } from '$lib/server/auth.js';
import { checkRateLimit, recordFailure, clearAttempts, getIp } from '$lib/server/ratelimit.js';
import { songsurfConfigured, songsurfBaseUrl, signSongsurfJwt } from '$lib/server/songsurf.js';

// Login membre — même contrat JSON qu'avant (ok/message/pseudo/songsurf_url),
// mais la session est désormais BetterAuth (cookie better-auth.session_token,
// table ba_sessions) au lieu du cookie custom rev0auth_member_session.

async function buildSongsurfUrl(pseudo: string, role: string): Promise<string | null> {
    if (!songsurfConfigured()) return null;
    const token = await signSongsurfJwt(pseudo, role, 8 * 3600);
    return `${songsurfBaseUrl()}?token=${token}`;
}

async function baSignIn(username: string, password: string) {
    try {
        return await auth.api.signInUsername({
            body: { username, password },
            returnHeaders: true,
        });
    } catch {
        return null;
    }
}

export const POST: RequestHandler = async ({ request }) => {
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

    let result = await baSignIn(key, password);
    if (!result && (await verifyPassword(password, user.passwordHash))) {
        // Compte valide côté web_users mais inconnu/désynchronisé côté ba_*
        // (créé ou mot de passe changé hors SvelteKit) : provisionnement
        // paresseux, puis nouvelle tentative.
        await setBaPassword(user.pseudo, user.passwordHash, user.role);
        result = await baSignIn(key, password);
    }
    if (!result) {
        recordFailure(ip);
        return json({ ok: false, state: 'invalid', message: 'Mot de passe incorrect.' });
    }

    clearAttempts(ip);

    const songsurfUrl = user.accessSongsurf
        ? await buildSongsurfUrl(user.pseudo, user.role)
        : null;

    const response = json({
        ok: true,
        state: 'ok',
        message: 'Connexion autorisée.',
        pseudo: user.pseudo,
        songsurf_url: songsurfUrl,
    });
    // Forward du cookie de session BetterAuth vers le navigateur
    for (const cookie of result.headers.getSetCookie()) {
        response.headers.append('set-cookie', cookie);
    }
    return response;
};
