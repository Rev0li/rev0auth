import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { auth } from '$lib/server/auth-v2.js';
import { verifySignin, elevateSession, SIGNIN_COOKIE } from '$lib/server/admin-passkey.js';
import { checkRateLimit, recordFailure, clearAttempts, getIp } from '$lib/server/ratelimit.js';
import { writeAudit } from '$lib/server/audit.js';

// Connexion par la clé seule (AUTH-21), étape 2 : la clé (avec son PIN) a
// signé le défi → session BetterAuth ouverte pour cet admin, déjà validée.
export const POST: RequestHandler = async ({ request, cookies }) => {
    const ip = getIp(request);
    if (checkRateLimit(ip).blocked) {
        return json({ ok: false, message: 'Trop de tentatives, réessaie dans 15 minutes.' }, { status: 429 });
    }

    const challengeId = cookies.get(SIGNIN_COOKIE);
    cookies.delete(SIGNIN_COOKIE, { path: '/japprends/passkey' });
    const { response } = await request.json().catch(() => ({}));
    const admin = challengeId ? await verifySignin(challengeId, response) : null;
    if (!admin) {
        recordFailure(ip);
        await writeAudit('admin_login_failed', '', '', 'passkey-signin');
        return json({ ok: false, message: 'Clé non reconnue, ou PIN non saisi.' }, { status: 401 });
    }

    clearAttempts(ip);
    const { headers, response: created } = await auth.api.createAdminKeySession({
        body: { userId: admin.userId },
        returnHeaders: true,
    });
    await elevateSession(created.sessionId);
    await writeAudit('admin_login', admin.pseudo, '', 'passkey-signin');

    const res = json({ ok: true });
    for (const cookie of headers.getSetCookie()) res.headers.append('set-cookie', cookie);
    return res;
};
