import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { db } from '$lib/server/db/index.js';
import { signupRequests, auditLog } from '$lib/server/db/schema.js';
import { eq } from 'drizzle-orm';

export const POST: RequestHandler = async ({ locals, params }) => {
    if (!locals.adminSession) throw error(401, 'Non autorisé.');
    const id = parseInt(params.id);

    await db.update(signupRequests).set({ status: 'rejected' }).where(eq(signupRequests.id, id));
    await db.insert(auditLog).values({
        timestampEpoch: Date.now(),
        action: 'reject_signup',
        target: String(id),
    });

    return json({ ok: true });
};
