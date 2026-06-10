import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { db } from '$lib/server/db/index.js';
import { wallPosts } from '$lib/server/db/schema.js';
import { and, eq, desc } from 'drizzle-orm';

export const GET: RequestHandler = async ({ locals }) => {
    if (!locals.memberSession) throw error(401, 'Non autorisé.');
    const rows = await db.select().from(wallPosts).orderBy(desc(wallPosts.createdAt)).limit(50);
    return json(rows);
};

export const POST: RequestHandler = async ({ request, locals }) => {
    if (!locals.memberSession) throw error(401, 'Non autorisé.');
    const { body } = await request.json();
    if (!body?.trim()) return json({ ok: false }, { status: 400 });

    await db.insert(wallPosts).values({ pseudo: locals.memberSession.pseudo, body });
    return json({ ok: true });
};

export const DELETE: RequestHandler = async ({ url, locals }) => {
    if (!locals.memberSession) throw error(401, 'Non autorisé.');
    const id = parseInt(url.searchParams.get('id') ?? '');
    if (isNaN(id)) return json({ ok: false }, { status: 400 });

    const deleted = await db.delete(wallPosts)
        .where(and(eq(wallPosts.id, id), eq(wallPosts.pseudo, locals.memberSession.pseudo)))
        .returning({ id: wallPosts.id });

    if (deleted.length === 0) {
        return json({ ok: false, message: 'Introuvable ou non autorisé.' }, { status: 404 });
    }
    return json({ ok: true, message: 'Message supprimé.' });
};
