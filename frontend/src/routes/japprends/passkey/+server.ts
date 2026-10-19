import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { listPasskeys } from '$lib/server/admin-passkey.js';

export const GET: RequestHandler = async ({ locals }) => {
    if (!locals.adminSession || !locals.memberSession) throw error(401, 'Non autorisé.');
    return json(await listPasskeys(locals.memberSession.userId));
};
