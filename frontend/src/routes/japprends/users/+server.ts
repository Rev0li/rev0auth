import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { listUsers, adminView } from '$lib/server/users.js';

// Plus de création directe de compte (AUTH-12) : on ajoute quelqu'un par
// invitation seulement (/japprends/invites).
export const GET: RequestHandler = async ({ locals }) => {
    if (!locals.adminSession) throw error(401, 'Non autorisé.');
    return json((await listUsers()).map(adminView));
};
