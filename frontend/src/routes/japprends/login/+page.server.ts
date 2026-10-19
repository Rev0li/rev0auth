import { redirect, error } from '@sveltejs/kit';
import type { PageServerLoad } from './$types.js';
import { listPasskeys } from '$lib/server/admin-passkey.js';

// Entrée dans la console : connexion membre d'abord, puis clé de sécurité.
export const load: PageServerLoad = async ({ locals }) => {
    if (locals.adminSession) throw redirect(303, '/japprends/dashboard');
    if (!locals.memberSession) throw redirect(303, '/');
    if (locals.memberSession.role !== 'admin') throw error(403, 'Accès réservé aux administrateurs.');

    const keys = await listPasskeys(locals.memberSession.userId);
    return { pseudo: locals.memberSession.pseudo, hasKeys: keys.length > 0 };
};
