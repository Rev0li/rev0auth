import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { db } from '$lib/server/db/index.js';
import { donations } from '$lib/server/db/schema.js';
import { eq, desc } from 'drizzle-orm';

export const GET: RequestHandler = async ({ locals }) => {
    if (!locals.memberSession) throw error(401, 'Non autorisé.');
    const rows = await db.select().from(donations)
        .where(eq(donations.pseudo, locals.memberSession.pseudo))
        .orderBy(desc(donations.createdAt));
    return json(rows);
};

export const POST: RequestHandler = async ({ request, locals }) => {
    if (!locals.memberSession) throw error(401, 'Non autorisé.');
    const { method, code } = await request.json();
    if (!method || !code?.trim()) return json({ ok: false }, { status: 400 });

    await db.insert(donations).values({
        pseudo: locals.memberSession.pseudo,
        method,
        code,
    });
    return json({ ok: true });
};
