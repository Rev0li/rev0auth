import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { db } from '$lib/server/db/index.js';
import { users } from '$lib/server/db/schema.js';
import { eq } from 'drizzle-orm';
import { verifyPassword } from '$lib/server/auth.js';
import { createSession, MEMBER_COOKIE, MEMBER_COOKIE_OPTS } from '$lib/server/session.js';

export const POST: RequestHandler = async ({ request, cookies }) => {
    const { pseudo, password } = await request.json();
    const key = pseudo?.trim()?.toLowerCase();
    if (!key || !password?.trim()) {
        return json({ ok: false, state: 'invalid', message: 'Champs requis.' });
    }

    const rows = await db.select().from(users).where(eq(users.pseudo, key)).limit(1);
    const user = rows[0];
    if (!user || !user.active) {
        return json({ ok: false, state: 'invalid', message: 'Mot de passe incorrect.' });
    }

    const valid = await verifyPassword(password, user.passwordHash);
    if (!valid) return json({ ok: false, state: 'invalid', message: 'Mot de passe incorrect.' });

    if (user.mustChangePassword) {
        return json({ ok: true, state: 'onboarding', message: 'Onboarding requis.' });
    }

    const token = await createSession(user.pseudo, 'member');
    cookies.set(MEMBER_COOKIE, token, MEMBER_COOKIE_OPTS);

    return json({ ok: true, state: 'ok', message: 'Connexion autorisée.', pseudo: user.pseudo });
};
