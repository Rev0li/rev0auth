import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { db } from '$lib/server/db/index.js';
import { invites } from '$lib/server/db/schema.js';
import { asc } from 'drizzle-orm';
import { generateToken } from '$lib/server/auth.js';

export const GET: RequestHandler = async ({ locals }) => {
    if (!locals.adminSession) throw error(401, 'Non autorisé.');
    const rows = await db.select().from(invites).orderBy(asc(invites.createdAt));
    return json(rows);
};

// Create a new invite link
export const POST: RequestHandler = async ({ request, locals }) => {
    if (!locals.adminSession) throw error(401, 'Non autorisé.');
    const { note, ttlDays = 7 } = await request.json();
    const now = Math.floor(Date.now() / 1000);
    const code = generateToken(16);

    await db.insert(invites).values({
        code,
        note: note ?? '',
        createdAt: now,
        expiresAt: now + ttlDays * 86400,
    });

    return json({ ok: true, code });
};
