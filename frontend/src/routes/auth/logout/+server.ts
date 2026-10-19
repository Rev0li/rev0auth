import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { auth } from '$lib/server/auth-v2.js';

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
    return response;
};
