import type { Handle } from '@sveltejs/kit';
import { getSession, ADMIN_COOKIE } from '$lib/server/session.js';
import { auth } from '$lib/server/auth-v2.js';
import { initDb } from '$lib/server/db/index.js';

await initDb();

export const handle: Handle = async ({ event, resolve }) => {
    event.locals.adminSession  = null;
    event.locals.memberSession = null;

    // Admin : session custom (web_sessions) — migration vers BetterAuth reportée
    const adminToken = event.cookies.get(ADMIN_COOKIE);
    if (adminToken) event.locals.adminSession = await getSession(adminToken);

    // Membre : session BetterAuth (ba_sessions). user.name = pseudo exact de
    // web_users (convention du script de migration + ba-sync), donc les
    // endpoints membres continuent de joindre web_users par pseudo.
    // Le test sur le header évite une requête DB quand aucun cookie BetterAuth
    // n'est présent (couvre aussi le préfixe __Secure- en prod).
    const cookieHeader = event.request.headers.get('cookie') ?? '';
    if (cookieHeader.includes('better-auth.session_token')) {
        const baSession = await auth.api.getSession({ headers: event.request.headers });
        if (baSession) {
            event.locals.memberSession = {
                pseudo: baSession.user.name,
                role:   baSession.user.role ?? 'member',
            };
        }
    }

    return resolve(event);
};
