import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { db } from '$lib/server/db/index.js';
import { signupRequests, users, auditLog } from '$lib/server/db/schema.js';
import { eq } from 'drizzle-orm';
import { hashPassword } from '$lib/server/auth.js';

export const POST: RequestHandler = async ({ request, locals, params }) => {
    if (!locals.adminSession) throw error(401, 'Non autorisé.');
    const id = parseInt(params.id);
    const { password } = await request.json(); // admin sets the initial password

    const rows = await db.select().from(signupRequests).where(eq(signupRequests.id, id)).limit(1);
    const req  = rows[0];
    if (!req) return json({ ok: false, message: 'Demande introuvable.' }, { status: 404 });

    const hash = await hashPassword(password ?? req.tempPassword);
    await db.insert(users).values({
        pseudo:        req.pseudo,
        role:          'member',
        active:        true,
        passwordHash:  hash,
        mustChangePassword: true,
    });
    await db.update(signupRequests).set({ status: 'approved' }).where(eq(signupRequests.id, id));

    await db.insert(auditLog).values({
        timestampEpoch: Date.now(),
        action: 'approve_signup',
        target: req.pseudo,
    });

    return json({ ok: true, pseudo: req.pseudo });
};
