import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { signinOptions, SIGNIN_CHALLENGE_TTL_S, SIGNIN_COOKIE } from '$lib/server/admin-passkey.js';
import { checkRateLimit, getIp } from '$lib/server/ratelimit.js';

// Connexion par la clé seule (AUTH-21), étape 1 : défi WebAuthn pour les clés
// des admins, sans session. L'identifiant du défi reste dans un cookie httpOnly.
export const POST: RequestHandler = async ({ request, cookies, url }) => {
    if (checkRateLimit(getIp(request)).blocked) {
        return json({ ok: false, message: 'Trop de tentatives, réessaie dans 15 minutes.' }, { status: 429 });
    }
    const result = await signinOptions();
    if (!result) return json({ ok: false, message: 'Aucune clé enregistrée.' }, { status: 409 });

    cookies.set(SIGNIN_COOKIE, result.challengeId, {
        path:     '/japprends/passkey',
        httpOnly: true,
        sameSite: 'strict',
        secure:   url.protocol === 'https:' || (process.env.ORIGIN ?? '').startsWith('https://'),
        maxAge:   SIGNIN_CHALLENGE_TTL_S,
    });
    return json({ ok: true, options: result.options });
};
