import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { db } from '$lib/server/db/index.js';
import { donations, auditLog } from '$lib/server/db/schema.js';
import { eq } from 'drizzle-orm';

export const POST: RequestHandler = async ({ request, locals, params }) => {
    if (!locals.adminSession) throw error(401, 'Non autorisé.');
    const id = parseInt(params.id);
    const { approved } = await request.json();

    await db.update(donations).set({ reviewed: true, approved }).where(eq(donations.id, id));
    await db.insert(auditLog).values({
        timestampEpoch: Date.now(),
        action: approved ? 'approve_donation' : 'reject_donation',
        target: String(id),
    });

    return json({ ok: true });
};
