import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { db } from '$lib/server/db/index.js';
import { invites } from '$lib/server/db/schema.js';
import { desc } from 'drizzle-orm';
import { generateToken } from '$lib/server/auth.js';

function requireAdmin({ locals }: { locals: App.Locals }) {
    if (!locals.adminSession) throw error(401, 'Non autorisé.');
}

export const GET: RequestHandler = async ({ locals }) => {
    requireAdmin({ locals });
    const rows = await db.select().from(invites).orderBy(desc(invites.createdAt));
    return json(rows);
};

export const POST: RequestHandler = async ({ request, locals }) => {
    requireAdmin({ locals });
    const body = await request.json().catch(() => ({}));
    const note = (body?.note ?? '').trim();
    const code = generateToken(12);
    const now = Math.floor(Date.now() / 1000);
    const expiresAt = now + 7 * 24 * 60 * 60;
    await db.insert(invites).values({ code, note, createdAt: now, expiresAt });
    return json({ ok: true, code });
};
