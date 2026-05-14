import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { deleteSession, ADMIN_COOKIE } from '$lib/server/session.js';

export const POST: RequestHandler = async ({ cookies }) => {
    const token = cookies.get(ADMIN_COOKIE);
    if (token) await deleteSession(token);
    cookies.delete(ADMIN_COOKIE, { path: '/' });
    return json({ ok: true });
};
