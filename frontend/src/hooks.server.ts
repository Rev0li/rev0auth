import type { Handle } from '@sveltejs/kit';
import { auth } from '$lib/server/auth-v2.js';
import { migrateDb } from '$lib/server/db/index.js';
import { isSessionElevated } from '$lib/server/admin-passkey.js';

await migrateDb();

export const handle: Handle = async ({ event, resolve }) => {
    event.locals.adminSession  = null;
    event.locals.memberSession = null;

    // Membre : session BetterAuth (ba_sessions + ba_users, champs métier
    // compris). Identité = user.id ; pseudo affiché = user.name, fixé côté
    // serveur (aucune route ne permet au client de le modifier). Seul un
    // compte actif ouvre une session membre.
    // Console admin : rôle admin + clé de sécurité validée dans cette même
    // session BetterAuth (cf. admin-passkey.ts).
    // Le test sur le header évite une requête DB quand aucun cookie BetterAuth
    // n'est présent (couvre aussi le préfixe __Secure- en prod).
    const cookieHeader = event.request.headers.get('cookie') ?? '';
    if (cookieHeader.includes('better-auth.session_token')) {
        const baSession = await auth.api.getSession({ headers: event.request.headers });
        if (baSession?.user.active) {
            const { user, session } = baSession;
            event.locals.memberSession = {
                pseudo:    user.name,
                role:      user.role,
                userId:    user.id,
                sessionId: session.id,
            };
            if (user.role === 'admin' && await isSessionElevated(session.id)) {
                event.locals.adminSession = { pseudo: user.name };
            }
        }
    }

    return resolve(event);
};
