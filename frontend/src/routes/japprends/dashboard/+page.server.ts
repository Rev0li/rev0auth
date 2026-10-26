import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types.js';
import { listUsers, adminView } from '$lib/server/users.js';
import { songsurfBaseUrl } from '$lib/server/songsurf.js';
import { portalUrl } from '$lib/server/console-host.js';

export const load: PageServerLoad = async ({ locals }) => {
    if (!locals.adminSession) throw redirect(302, '/japprends/login');

    return {
        users: (await listUsers()).map(adminView),
        admin: locals.adminSession.pseudo,
        songsurfUrl: songsurfBaseUrl() || null,
        // Liens d'invitation sur le portail public, pas sur le nom de la console
        signupUrl:   portalUrl('/signup'),
    };
};
