import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { checkRateLimit, getIp } from '$lib/server/ratelimit.js';

const WEB_UPSTREAM = process.env.REV0AUTH_WEB_UPSTREAM ?? 'http://127.0.0.1:3000';

export const GET: RequestHandler = async ({ request }) => {
    const ip = getIp(request);
    if (checkRateLimit(ip).blocked) {
        return json({ locked: true, message: 'Trop de tentatives, réessaie dans 15 minutes.' }, { status: 429 });
    }

    try {
        const upstream = await fetch(`${WEB_UPSTREAM}/japprends/webauthn/auth/start`, {
            headers: { 'x-forwarded-for': ip },
        });
        const data = await upstream.json();
        return json(data, { status: upstream.status });
    } catch {
        return json({ locked: false, webauthn_required: false, error: 'Service indisponible.' }, { status: 503 });
    }
};
