import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { db } from '$lib/server/db/index.js';
import { users } from '$lib/server/db/schema.js';
import { eq } from 'drizzle-orm';
import { hashPassword } from '$lib/server/auth.js';

export const POST: RequestHandler = async ({ request, locals }) => {
    if (!locals.memberSession) throw error(401, 'Non autorisé.');

    const { newPassword } = await request.json();
    if (!newPassword || newPassword.length < 8) {
        return json({ ok: false, message: 'Minimum 8 caractères.' }, { status: 400 });
    }

    const hash = await hashPassword(newPassword);
    await db.update(users)
        .set({ passwordHash: hash })
        .where(eq(users.pseudo, locals.memberSession.pseudo));

    return json({ ok: true });
};
