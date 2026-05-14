import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { verifyTotp } from '$lib/server/auth.js';
import { createSession, ADMIN_COOKIE, ADMIN_COOKIE_OPTS } from '$lib/server/session.js';
import { checkRateLimit, recordFailure, clearAttempts, getIp } from '$lib/server/ratelimit.js';

export const POST: RequestHandler = async ({ request, cookies }) => {
    const ip = getIp(request);

    if (checkRateLimit(ip).blocked) {
        return json({ ok: false, message: 'Trop de tentatives, réessaie dans 15 minutes.' }, { status: 429 });
    }

    const body = await request.json();
    const { pseudo, seed, password, challenge_choice, trap_value, otp } = body;

    // Honeypot
    if (trap_value?.trim()) {
        recordFailure(ip);
        return json({ ok: false, message: 'Tentative invalide.' }, { status: 401 });
    }

    // Challenge
    if (challenge_choice !== 'secure-lock') {
        recordFailure(ip);
        return json({ ok: false, message: 'Challenge invalide.' }, { status: 401 });
    }

    // Credentials from env
    const expectedPassword = process.env.ADMIN_DASH_PASSWORD;
    const expectedPseudo   = process.env.ADMIN_DASH_PSEUDO   ?? 'admin';
    const expectedSeed     = process.env.ADMIN_DASH_SEED     ?? 'rev0auth-seed';
    const totpSecret       = process.env.ADMIN_DASH_TOTP_SECRET;

    if (!expectedPassword) {
        return json({ ok: false, message: 'Admin non configuré.' }, { status: 503 });
    }

    if (pseudo !== expectedPseudo || seed !== expectedSeed || password !== expectedPassword) {
        recordFailure(ip);
        return json({ ok: false, message: 'Identifiants admin invalides.' }, { status: 401 });
    }

    // TOTP (if configured)
    if (totpSecret) {
        if (!verifyTotp(totpSecret, otp ?? '')) {
            recordFailure(ip);
            return json({ ok: false, message: 'Code 2FA invalide.' }, { status: 401 });
        }
    }

    clearAttempts(ip);

    const token = await createSession(expectedPseudo, 'admin');
    cookies.set(ADMIN_COOKIE, token, ADMIN_COOKIE_OPTS);

    return json({ ok: true, message: 'Connexion admin validée.' });
};
