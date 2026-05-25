import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { createSession, ADMIN_COOKIE, ADMIN_COOKIE_OPTS } from '$lib/server/session.js';
import { recordFailure, clearAttempts, getIp } from '$lib/server/ratelimit.js';

const WEB_UPSTREAM = process.env.REV0AUTH_WEB_UPSTREAM ?? 'http://127.0.0.1:3000';
const ADMIN_PSEUDO = process.env.ADMIN_DASH_PSEUDO ?? 'admin';

export const POST: RequestHandler = async ({ request, cookies }) => {
    const ip = getIp(request);
    const body = await request.json();

    try {
        const upstream = await fetch(`${WEB_UPSTREAM}/japprends/webauthn/auth/finish`, {
            method: 'POST',
            headers: {
                'content-type': 'application/json',
                'x-forwarded-for': ip,
            },
            body: JSON.stringify(body),
        });

        const data = await upstream.json();

        if (!data.ok) {
            recordFailure(ip);
            return json({ ok: false, message: data.message ?? 'Authentification échouée.' }, { status: 401 });
        }

        clearAttempts(ip);
        const token = await createSession(ADMIN_PSEUDO, 'admin');
        cookies.set(ADMIN_COOKIE, token, ADMIN_COOKIE_OPTS);

        return json({ ok: true, message: 'Connexion admin validée.' });
    } catch {
        return json({ ok: false, message: 'Service indisponible.' }, { status: 503 });
    }
};
