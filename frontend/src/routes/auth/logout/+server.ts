import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { auth } from '$lib/server/auth-v2.js';
import { deleteSession, MEMBER_COOKIE } from '$lib/server/session.js';

export const POST: RequestHandler = async ({ request, cookies }) => {
    // Révoque la session BetterAuth (throw si déjà déconnecté → ignoré)
    let baHeaders: Headers | null = null;
    try {
        const result = await auth.api.signOut({ headers: request.headers, returnHeaders: true });
        baHeaders = result.headers;
    } catch {
        // pas de session BetterAuth active
    }

    // Nettoyage legacy : sessions web_sessions d'avant la bascule BetterAuth
    const legacy = cookies.get(MEMBER_COOKIE);
    if (legacy) await deleteSession(legacy);
    cookies.delete(MEMBER_COOKIE, { path: '/' });

    const response = json({ ok: true });
    for (const cookie of baHeaders?.getSetCookie() ?? []) {
        response.headers.append('set-cookie', cookie);
    }
    return response;
};
