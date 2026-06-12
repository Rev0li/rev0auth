import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types.js';
import { db } from '$lib/server/db/index.js';
import { users, wallPosts, messages } from '$lib/server/db/schema.js';
import { eq, and, desc, count, sql } from 'drizzle-orm';

export const load: PageServerLoad = async ({ locals }) => {
    if (!locals.memberSession) throw redirect(302, '/');
    const pseudo = locals.memberSession.pseudo;

    const [userRows, wall, members, unreadRows] = await Promise.all([
        db.select().from(users).where(eq(users.pseudo, pseudo)).limit(1),
        db.select().from(wallPosts).orderBy(desc(wallPosts.createdAt)).limit(30),
        db.select({
            pseudo:    users.pseudo,
            role:      users.role,
            bio:       users.bio,
            avatarMime: users.avatarMime,
        }).from(users).where(eq(users.active, true)),
        db.select({ n: count() }).from(messages)
            .where(and(sql`LOWER(${messages.toPseudo}) = ${pseudo.toLowerCase()}`, eq(messages.isRead, false))),
    ]);

    const user = userRows[0];
    if (!user) throw redirect(302, '/');

    const { passwordHash: _h, avatarBytes: _b, ...safeUser } = user;

    return {
        user:        safeUser,
        wall,
        members,
        unreadCount: unreadRows[0]?.n ?? 0,
        // Fil de discussion permanent "Admin" dans la messagerie
        adminPseudo: process.env.ADMIN_DASH_PSEUDO ?? 'admin',
    };
};
