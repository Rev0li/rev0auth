import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { db } from '$lib/server/db/index.js';
import { users, auditLog } from '$lib/server/db/schema.js';
import { eq } from 'drizzle-orm';
import { hashPassword } from '$lib/server/auth.js';

export const POST: RequestHandler = async ({ request, locals, params }) => {
    if (!locals.adminSession) throw error(401, 'Non autorisé.');
    const key = params.pseudo.toLowerCase();
    const { password } = await request.json();
    if (!password) return json({ ok: false, message: 'Mot de passe requis.' }, { status: 400 });

    const hash = await hashPassword(password);
    await db.update(users)
        .set({ passwordHash: hash, mustChangePassword: true })
        .where(eq(users.pseudo, key));

    await db.insert(auditLog).values({
        timestampEpoch: Date.now(),
        action: 'reset_password',
        target: key,
    });

    return json({ ok: true });
};
