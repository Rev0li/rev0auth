import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { db } from '$lib/server/db/index.js';
import { users } from '$lib/server/db/schema.js';
import { eq } from 'drizzle-orm';

export const POST: RequestHandler = async ({ params, locals }) => {
    if (!locals.adminSession) throw error(401, 'Non autorisé.');
    const updated = await db
        .update(users)
        .set({ status: 'occupe' })
        .where(eq(users.pseudo, params.pseudo!))
        .returning({ pseudo: users.pseudo });
    if (updated.length === 0) {
        return json({ ok: false, message: 'Utilisateur introuvable.' });
    }
    return json({ ok: true, message: 'Statut change en occupe.' });
};
