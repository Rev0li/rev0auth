import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types.js';
import { listUsers, adminView } from '$lib/server/users.js';
import { songsurfBaseUrl } from '$lib/server/songsurf.js';

export const load: PageServerLoad = async ({ locals }) => {
    if (!locals.adminSession) throw redirect(302, '/japprends/login');

    return {
        users: (await listUsers()).map(adminView),
        admin: locals.adminSession.pseudo,
        songsurfUrl: songsurfBaseUrl() || null,
    };
};
