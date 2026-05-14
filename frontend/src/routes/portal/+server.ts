import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { db } from '$lib/server/db/index.js';
import { signupRequests, users } from '$lib/server/db/schema.js';
import { eq } from 'drizzle-orm';
import { hashPassword } from '$lib/server/auth.js';
import { randomBytes } from 'crypto';

export const POST: RequestHandler = async ({ request }) => {
    const { pseudo, referral } = await request.json();
    const key = pseudo?.trim()?.toLowerCase();
    if (!key) return json({ ok: false, message: 'Pseudo requis.' }, { status: 400 });

    const exists = await db.select({ pseudo: users.pseudo }).from(users).where(eq(users.pseudo, key)).limit(1);
    if (exists[0]) return json({ ok: false, message: 'Pseudo déjà pris.' }, { status: 409 });

    const pending = await db.select({ id: signupRequests.id })
        .from(signupRequests)
        .where(eq(signupRequests.pseudo, key))
        .limit(1);
    if (pending[0]) return json({ ok: false, message: 'Demande déjà en attente.' }, { status: 409 });

    const tempPassword = randomBytes(6).toString('hex'); // admin sets real password on approve
    await db.insert(signupRequests).values({
        pseudo: key,
        referral: referral?.trim() || null,
        tempPassword,
        status: 'pending',
    });

    return json({ ok: true, message: 'Demande envoyée.' });
};
