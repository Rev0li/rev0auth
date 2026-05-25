import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { db } from '$lib/server/db/index.js';
import { messages } from '$lib/server/db/schema.js';
import { eq, or, and, desc } from 'drizzle-orm';

function requireAdmin({ locals }: { locals: App.Locals }) {
    if (!locals.adminSession) throw error(401, 'Non autorisé.');
}

export const GET: RequestHandler = async ({ locals }) => {
    requireAdmin({ locals });
    const rows = await db.select().from(messages).orderBy(desc(messages.createdAt));
    return json(rows);
};

// Mark all messages in a thread as read (pseudo = the other participant)
export const PATCH: RequestHandler = async ({ request, locals }) => {
    requireAdmin({ locals });
    const { pseudo } = await request.json();
    if (!pseudo) return json({ ok: false }, { status: 400 });
    await db.update(messages)
        .set({ isRead: true })
        .where(or(eq(messages.fromPseudo, pseudo), eq(messages.toPseudo, pseudo)));
    return json({ ok: true });
};

// Delete all messages in a thread (pseudo = the other participant)
export const DELETE: RequestHandler = async ({ request, locals }) => {
    requireAdmin({ locals });
    const { pseudo } = await request.json();
    if (!pseudo) return json({ ok: false }, { status: 400 });
    await db.delete(messages)
        .where(or(eq(messages.fromPseudo, pseudo), eq(messages.toPseudo, pseudo)));
    return json({ ok: true });
};
