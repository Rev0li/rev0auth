import type { Handle } from '@sveltejs/kit';
import { getSession, ADMIN_COOKIE, MEMBER_COOKIE } from '$lib/server/session.js';
import { initDb } from '$lib/server/db/index.js';

await initDb();

export const handle: Handle = async ({ event, resolve }) => {
    event.locals.adminSession  = null;
    event.locals.memberSession = null;

    const adminToken  = event.cookies.get(ADMIN_COOKIE);
    const memberToken = event.cookies.get(MEMBER_COOKIE);

    if (adminToken)  event.locals.adminSession  = await getSession(adminToken,  'admin');
    if (memberToken) event.locals.memberSession = await getSession(memberToken, 'member');

    return resolve(event);
};
