import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { db } from '$lib/server/db/index.js';
import { users } from '$lib/server/db/schema.js';
import { eq } from 'drizzle-orm';
import { verifyPassword, hashPassword } from '$lib/server/auth.js';
import { createSession, MEMBER_COOKIE, MEMBER_COOKIE_OPTS } from '$lib/server/session.js';

export const PUT: RequestHandler = async ({ request, locals, cookies }) => {
    if (!locals.memberSession) throw error(401, 'Non autorisé.');
    const { currentPassword, newPassword } = await request.json();
    if (!currentPassword || !newPassword) return json({ ok: false, message: 'Champs requis.' }, { status: 400 });

    const rows = await db.select().from(users).where(eq(users.pseudo, locals.memberSession.pseudo)).limit(1);
    const user = rows[0];
    if (!user) throw error(404);

    const valid = await verifyPassword(currentPassword, user.passwordHash);
    if (!valid) return json({ ok: false, message: 'Mot de passe actuel incorrect.' }, { status: 401 });

    const hash = await hashPassword(newPassword);
    await db.update(users).set({ passwordHash: hash }).where(eq(users.pseudo, user.pseudo));

    // Re-issue session with updated state
    const token = await createSession(user.pseudo, 'member');
    cookies.set(MEMBER_COOKIE, token, MEMBER_COOKIE_OPTS);

    return json({ ok: true, message: 'Mot de passe mis à jour.' });
};
