import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types.js';
import { db } from '$lib/server/db/index.js';
import { users } from '$lib/server/db/schema.js';
import { eq } from 'drizzle-orm';

export const load: PageServerLoad = async ({ locals }) => {
    if (!locals.memberSession) throw redirect(302, '/');
    const pseudo = locals.memberSession.pseudo;

    const userRows = await db.select().from(users).where(eq(users.pseudo, pseudo)).limit(1);

    const user = userRows[0];
    if (!user) throw redirect(302, '/');
    const { passwordHash: _h, avatarBytes: _b, ...safeUser } = user;

    return { user: safeUser };
};
