import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { db } from '$lib/server/db/index.js';
import { messages } from '$lib/server/db/schema.js';
import { eq, or, and, desc } from 'drizzle-orm';

export const GET: RequestHandler = async ({ url, locals }) => {
    if (!locals.memberSession) throw error(401, 'Non autorisé.');
    const pseudo = locals.memberSession.pseudo;
    const folder = url.searchParams.get('folder') ?? 'inbox';

    const rows = folder === 'sent'
        ? await db.select().from(messages).where(eq(messages.fromPseudo, pseudo)).orderBy(desc(messages.createdAt))
        : await db.select().from(messages).where(eq(messages.toPseudo, pseudo)).orderBy(desc(messages.createdAt));

    return json(rows);
};

export const POST: RequestHandler = async ({ request, locals }) => {
    if (!locals.memberSession) throw error(401, 'Non autorisé.');
    const { to, body } = await request.json();
    if (!to || !body?.trim()) return json({ ok: false }, { status: 400 });

    await db.insert(messages).values({
        fromPseudo: locals.memberSession.pseudo,
        toPseudo:   to.toLowerCase(),
        body,
    });
    return json({ ok: true });
};

export const PATCH: RequestHandler = async ({ request, locals }) => {
    if (!locals.memberSession) throw error(401, 'Non autorisé.');
    const { id } = await request.json();
    await db.update(messages)
        .set({ isRead: true })
        .where(and(eq(messages.id, id), eq(messages.toPseudo, locals.memberSession.pseudo)));
    return json({ ok: true });
};
