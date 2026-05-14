import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { db } from '$lib/server/db/index.js';
import { users } from '$lib/server/db/schema.js';
import { eq } from 'drizzle-orm';

export const POST: RequestHandler = async ({ request }) => {
    const { pseudo } = await request.json();
    const key = pseudo?.trim()?.toLowerCase();
    if (!key) return json({ ok: false, state: 'invalid', message: 'Pseudo requis.' });

    // Admin detection (safe for private app)
    const adminPseudo = (process.env.ADMIN_DASH_PSEUDO ?? 'admin').toLowerCase();
    if (key === adminPseudo) {
        const totpEnabled = !!process.env.ADMIN_DASH_TOTP_SECRET;
        return json({ ok: true, state: 'admin', totpEnabled });
    }

    const user = await db.select().from(users).where(eq(users.pseudo, key)).limit(1);
    if (!user[0]) return json({ ok: false, state: 'missing', message: 'Compte introuvable.' });
    if (!user[0].active) return json({ ok: false, state: 'inactive', message: 'Compte inactif.' });

    return json({ ok: true, state: 'approved', message: 'Connexion autorisée.' });
};
