import { redirect, error } from '@sveltejs/kit';
import type { PageServerLoad } from './$types.js';
import { listPasskeys } from '$lib/server/admin-passkey.js';
import { consoleUrl } from '$lib/server/console-host.js';
import { loginUrl, safeReturnTo } from '$lib/server/forward-auth.js';

// Entrée dans la console. Sans session : connexion par la clé seule (AUTH-21),
// ou par le portail public (mot de passe) puis retour ici. Connecté comme
// admin : la clé de sécurité valide la session.
export const load: PageServerLoad = async ({ locals }) => {
    if (locals.adminSession) throw redirect(303, '/japprends/dashboard');
    if (!locals.memberSession) {
        return {
            mode:        'signin' as const,
            pseudo:      null,
            hasKeys:     true,
            portalLogin: loginUrl(safeReturnTo(consoleUrl('/japprends/login'))),
        };
    }
    if (locals.memberSession.role !== 'admin') throw error(403, 'Accès réservé aux administrateurs.');

    const keys = await listPasskeys(locals.memberSession.userId);
    return {
        mode:        'session' as const,
        pseudo:      locals.memberSession.pseudo,
        hasKeys:     keys.length > 0,
        portalLogin: null,
    };
};
