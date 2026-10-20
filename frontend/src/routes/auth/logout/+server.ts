import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { auth, cookieDomain } from '$lib/server/auth-v2.js';

const SESSION_COOKIES = ['__Secure-better-auth.session_token', 'better-auth.session_token'];

export const POST: RequestHandler = async ({ request }) => {
    // Révoque la session BetterAuth (throw si déjà déconnecté → ignoré)
    let baHeaders: Headers | null = null;
    try {
        const result = await auth.api.signOut({ headers: request.headers, returnHeaders: true });
        baHeaders = result.headers;
    } catch {
        // pas de session BetterAuth active
    }

    const response = json({ ok: true });
    for (const cookie of baHeaders?.getSetCookie() ?? []) {
        response.headers.append('set-cookie', cookie);
    }

    // Cookie partagé (AUTH-07) : BetterAuth n'efface que celui du domaine
    // parent. Un cookie lié au seul hôte, posé avant AUTH-07, resterait
    // envoyé au portail : on l'efface aussi.
    if (cookieDomain) {
        const sent = (request.headers.get('cookie') ?? '').split(';').map((c) => c.split('=')[0].trim());
        for (const name of SESSION_COOKIES.filter((n) => sent.includes(n))) {
            const secure = name.startsWith('__Secure-') ? '; Secure' : '';
            response.headers.append('set-cookie', `${name}=; Path=/; Max-Age=0; HttpOnly; SameSite=Lax${secure}`);
        }
    }
    return response;
};
