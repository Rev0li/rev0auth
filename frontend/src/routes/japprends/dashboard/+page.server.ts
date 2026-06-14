import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types.js';
import { db } from '$lib/server/db/index.js';
import { users } from '$lib/server/db/schema.js';
import { asc } from 'drizzle-orm';

export const load: PageServerLoad = async ({ locals }) => {
    if (!locals.adminSession) throw redirect(302, '/japprends/login');

    const allUsers = await db.select({
        pseudo:           users.pseudo,
        role:             users.role,
        active:           users.active,
        approved:         users.approved,
        status:           users.status,
        createdAt:        users.createdAt,
        accessJellyfin:   users.accessJellyfin,
        accessSongsurf:   users.accessSongsurf,
        requestJellyfin:  users.requestJellyfin,
        requestSongsurf:  users.requestSongsurf,
        githubUsername:   users.githubUsername,
        linkedinName:     users.linkedinName,
    }).from(users).orderBy(asc(users.createdAt));

    return {
        users: allUsers,
        admin: locals.adminSession.pseudo,
        songsurfEnabled: !!(process.env.SONGSURF_URL && process.env.AUTH_JWT_SECRET),
    };
};
