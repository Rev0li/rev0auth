import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { db } from '$lib/server/db/index.js';
import { donations } from '$lib/server/db/schema.js';
import { eq } from 'drizzle-orm';

export const POST: RequestHandler = async ({ request, locals, params }) => {
    if (!locals.adminSession) throw error(401, 'Non autorisé.');
    const id = parseInt(params.id);
    const { approved } = await request.json();

    await db.update(donations).set({ reviewed: true, approved }).where(eq(donations.id, id));

    return json({ ok: true });
};
