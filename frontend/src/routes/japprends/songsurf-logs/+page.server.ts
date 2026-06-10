import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types.js';

export const load: PageServerLoad = async ({ locals }) => {
    if (!locals.adminSession) throw redirect(303, '/japprends/login');
    return {};
};
