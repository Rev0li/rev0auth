import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';

export const GET: RequestHandler = async ({ locals }) => {
    if (!locals.adminSession) throw error(401, 'Non autorisé.');
    return json({
        admin_ok: true,
        user_ok: true,
        checked_at_epoch: Math.floor(Date.now() / 1000),
    });
};
