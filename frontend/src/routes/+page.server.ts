import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types.js';

export const load: PageServerLoad = async ({ locals }) => {
    if (locals.adminSession)  throw redirect(302, '/japprends/dashboard');
    if (locals.memberSession) throw redirect(302, '/home/friend');
};
