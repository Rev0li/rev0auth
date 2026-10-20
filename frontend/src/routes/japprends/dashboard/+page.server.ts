import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types.js';
import { listUsers, adminView } from '$lib/server/users.js';

export const load: PageServerLoad = async ({ locals }) => {
    if (!locals.adminSession) throw redirect(302, '/japprends/login');

    return {
        users: (await listUsers()).map(adminView),
        admin: locals.adminSession.pseudo,
        songsurfEnabled: !!(process.env.SONGSURF_URL && process.env.AUTH_JWT_SECRET),
    };
};
