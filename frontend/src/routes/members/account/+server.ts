import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { deleteUser } from '$lib/server/users.js';

// Suppression de son propre compte : sessions, mot de passe et avatar
// partent en cascade avec ba_users.
export const DELETE: RequestHandler = async ({ locals }) => {
    if (!locals.memberSession) throw error(401, 'Non autorisé.');
    await deleteUser(locals.memberSession.userId);
    return json({ ok: true, message: 'Compte supprime.' });
};
