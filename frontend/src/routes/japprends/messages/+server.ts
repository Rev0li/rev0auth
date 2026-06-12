import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { db } from '$lib/server/db/index.js';
import { messages } from '$lib/server/db/schema.js';
import { or, and, desc, sql } from 'drizzle-orm';

function requireAdmin({ locals }: { locals: App.Locals }) {
    if (!locals.adminSession) throw error(401, 'Non autorisé.');
}

const adminLow = () => (process.env.ADMIN_DASH_PSEUDO ?? 'admin').toLowerCase();

// Seules les conversations impliquant l'admin sont visibles ici : les DM
// membre↔membre (popup de /home/friend) sont privés et n'apparaissent pas.

export const GET: RequestHandler = async ({ locals }) => {
    requireAdmin({ locals });
    const a = adminLow();
    const rows = await db.select().from(messages)
        .where(or(
            sql`LOWER(${messages.fromPseudo}) = ${a}`,
            sql`LOWER(${messages.toPseudo}) = ${a}`,
        ))
        .orderBy(desc(messages.createdAt));
    return json(rows);
};

// Mark all messages in an admin thread as read (pseudo = the other participant)
export const PATCH: RequestHandler = async ({ request, locals }) => {
    requireAdmin({ locals });
    const { pseudo } = await request.json();
    const p = pseudo?.trim()?.toLowerCase();
    if (!p) return json({ ok: false }, { status: 400 });
    await db.update(messages)
        .set({ isRead: true })
        .where(and(
            sql`LOWER(${messages.fromPseudo}) = ${p}`,
            sql`LOWER(${messages.toPseudo}) = ${adminLow()}`,
        ));
    return json({ ok: true });
};

// Delete the admin thread with a member (pseudo = the other participant)
export const DELETE: RequestHandler = async ({ request, locals }) => {
    requireAdmin({ locals });
    const { pseudo } = await request.json();
    const p = pseudo?.trim()?.toLowerCase();
    if (!p) return json({ ok: false }, { status: 400 });
    const a = adminLow();
    await db.delete(messages)
        .where(or(
            and(sql`LOWER(${messages.fromPseudo}) = ${p}`, sql`LOWER(${messages.toPseudo}) = ${a}`),
            and(sql`LOWER(${messages.fromPseudo}) = ${a}`, sql`LOWER(${messages.toPseudo}) = ${p}`),
        ));
    return json({ ok: true });
};
