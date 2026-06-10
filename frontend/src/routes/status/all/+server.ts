import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { checkApiUp } from '$lib/server/api-health.js';

export const GET: RequestHandler = async ({ locals }) => {
    if (!locals.adminSession) throw error(401, 'Non autorisé.');
    const api_ok = await checkApiUp();
    return json({
        checked_at_epoch: Math.floor(Date.now() / 1000),
        admin_ok: true,
        user_ok: true,
        api_ok,
        web_ok: true,
        sprint: 'AUTH-006',
        tests_api_total: 18,
    });
};
