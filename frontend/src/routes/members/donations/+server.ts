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

const VALID_METHODS = new Set(['crypto', 'pcs']);

export const POST: RequestHandler = async ({ request, locals }) => {
    if (!locals.memberSession) throw error(401, 'Non autorisé.');
    const { method, code } = await request.json();
    const m = typeof method === 'string' ? method.trim().toLowerCase() : '';
    const c = typeof code === 'string' ? code.trim() : '';
    if (!VALID_METHODS.has(m)) {
        return json({ ok: false, message: 'Methode invalide. Utilise crypto ou pcs.' }, { status: 400 });
    }
    if (!c) {
        return json({ ok: false, message: 'Code/reference donation manquant.' }, { status: 400 });
    }

    await db.insert(donations).values({
        pseudo: locals.memberSession.pseudo,
        method: m,
        code: c,
    });
    return json({ ok: true });
};
