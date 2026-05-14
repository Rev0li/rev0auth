import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { db } from '$lib/server/db/index.js';
import { users } from '$lib/server/db/schema.js';
import { eq } from 'drizzle-orm';

// Public list of active members (no sensitive fields)
export const GET: RequestHandler = async ({ locals }) => {
    if (!locals.memberSession && !locals.adminSession) throw error(401, 'Non autorisé.');

    const rows = await db
        .select({
            pseudo:     users.pseudo,
            role:       users.role,
            status:     users.status,
            bio:        users.bio,
            avatarMime: users.avatarMime,
        })
        .from(users)
        .where(eq(users.active, true));

    return json(rows);
};
