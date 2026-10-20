import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types.js';
import { safeReturnTo } from '$lib/server/forward-auth.js';

export const load: PageServerLoad = async ({ locals, url }) => {
    // Arrivée depuis un service (/auth/verify) : sa requête n'a pas porté de
    // session valable (absente, ou ancien cookie lié au seul portail). On
    // affiche le formulaire même si le portail voit une session.
    const returnTo = safeReturnTo(url.searchParams.get('redirect'));
    if (returnTo) return { returnTo };

    if (locals.adminSession)  throw redirect(302, '/japprends/dashboard');
    if (locals.memberSession) throw redirect(302, '/home/friend');
    return { returnTo: null };
};
