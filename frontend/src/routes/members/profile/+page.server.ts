import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types.js';
import { db } from '$lib/server/db/index.js';
import { users, messages, donations } from '$lib/server/db/schema.js';
import { eq, or, desc } from 'drizzle-orm';

export const load: PageServerLoad = async ({ locals }) => {
    if (!locals.memberSession) throw redirect(302, '/');
    const pseudo = locals.memberSession.pseudo;

    const [userRows, inbox, sent, myDonations] = await Promise.all([
        db.select().from(users).where(eq(users.pseudo, pseudo)).limit(1),
        db.select().from(messages).where(eq(messages.toPseudo, pseudo)).orderBy(desc(messages.createdAt)),
        db.select().from(messages).where(eq(messages.fromPseudo, pseudo)).orderBy(desc(messages.createdAt)),
        db.select().from(donations).where(eq(donations.pseudo, pseudo)).orderBy(desc(donations.createdAt)),
    ]);

    const user = userRows[0];
    if (!user) throw redirect(302, '/');
    const { passwordHash: _h, avatarBytes: _b, ...safeUser } = user;

    return { user: safeUser, inbox, sent, donations: myDonations };
};
