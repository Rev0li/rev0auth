import type { Handle } from '@sveltejs/kit';
import { auth } from '$lib/server/auth-v2.js';
import { db, migrateDb } from '$lib/server/db/index.js';
import { isSessionElevated } from '$lib/server/admin-passkey.js';
import { users } from '$lib/server/db/schema.js';
import { sql } from 'drizzle-orm';

await migrateDb();

export const handle: Handle = async ({ event, resolve }) => {
    event.locals.adminSession  = null;
    event.locals.memberSession = null;

    // Membre : session BetterAuth (ba_sessions). L'identité vient de
    // user.username (unique, fixé côté serveur = LOWER(pseudo)), jamais du nom
    // d'affichage. Pseudo exact et rôle sont relus dans web_users, et seul un
    // compte actif ouvre une session membre.
    // Console admin : rôle admin + clé de sécurité validée dans cette même
    // session BetterAuth (cf. admin-passkey.ts).
    // Le test sur le header évite une requête DB quand aucun cookie BetterAuth
    // n'est présent (couvre aussi le préfixe __Secure- en prod).
    const cookieHeader = event.request.headers.get('cookie') ?? '';
    if (cookieHeader.includes('better-auth.session_token')) {
        const baSession = await auth.api.getSession({ headers: event.request.headers });
        const username = baSession?.user.username;
        if (baSession && username) {
            const rows = await db
                .select({ pseudo: users.pseudo, role: users.role, active: users.active })
                .from(users)
                .where(sql`LOWER(${users.pseudo}) = ${username}`)
                .limit(1);
            const user = rows[0];
            if (user?.active) {
                event.locals.memberSession = {
                    pseudo:    user.pseudo,
                    role:      user.role,
                    userId:    baSession.user.id,
                    sessionId: baSession.session.id,
                };
                if (user.role === 'admin' && await isSessionElevated(baSession.session.id)) {
                    event.locals.adminSession = { pseudo: user.pseudo };
                }
            }
        }
    }

    return resolve(event);
};
