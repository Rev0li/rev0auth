import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { db } from '$lib/server/db/index.js';
import { users } from '$lib/server/db/schema.js';
import { eq } from 'drizzle-orm';

const VALID_STATUSES = ['actif', 'occupe', 'inactif'] as const;

export const PUT: RequestHandler = async ({ request, locals }) => {
    if (!locals.memberSession) throw error(401, 'Non autorisé.');
    const { status } = await request.json();
    if (!VALID_STATUSES.includes(status)) return json({ ok: false }, { status: 400 });

    await db.update(users).set({ status }).where(eq(users.pseudo, locals.memberSession.pseudo));
    return json({ ok: true });
};
