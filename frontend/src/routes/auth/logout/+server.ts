import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { deleteSession, MEMBER_COOKIE } from '$lib/server/session.js';

export const POST: RequestHandler = async ({ cookies, locals }) => {
    const token = cookies.get(MEMBER_COOKIE);
    if (token) await deleteSession(token);
    cookies.delete(MEMBER_COOKIE, { path: '/' });
    return json({ ok: true });
};
