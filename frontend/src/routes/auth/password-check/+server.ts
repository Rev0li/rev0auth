import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { auth } from '$lib/server/auth-v2.js';
import { findUserByPseudo, usernameOf } from '$lib/server/users.js';
import { checkRateLimit, recordFailure, clearAttempts, getIp } from '$lib/server/ratelimit.js';
import { songsurfConfigured, songsurfBaseUrl, signSongsurfJwt } from '$lib/server/songsurf.js';

// Login membre : BetterAuth vérifie pseudo + mot de passe et pose son cookie
// de session. Même contrat JSON qu'avant (ok/message/pseudo/songsurf_url).

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
    if (typeof pseudo !== 'string' || !pseudo.trim() || typeof password !== 'string' || !password.trim()) {
        return json({ ok: false, state: 'invalid', message: 'Champs requis.' });
    }

    // Compte désactivé : refus avant d'ouvrir une session
    const user = await findUserByPseudo(pseudo);
    const result = user?.active ? await baSignIn(usernameOf(pseudo), password) : null;
    if (!user || !result) {
        recordFailure(ip);
        return json({ ok: false, state: 'invalid', message: 'Mot de passe incorrect.' });
    }

    clearAttempts(ip);

    const songsurfUrl = user.accessSongsurf ? await buildSongsurfUrl(user.name, user.role) : null;

    const response = json({
        ok: true,
        state: 'ok',
        message: 'Connexion autorisée.',
        pseudo: user.name,
        songsurf_url: songsurfUrl,
    });
    // Forward du cookie de session BetterAuth vers le navigateur
    for (const cookie of result.headers.getSetCookie()) {
        response.headers.append('set-cookie', cookie);
    }
    return response;
};
