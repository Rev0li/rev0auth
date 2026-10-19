import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { db } from '$lib/server/db/index.js';
import { users } from '$lib/server/db/schema.js';
import { sql } from 'drizzle-orm';
import { deleteBaUser } from '$lib/server/ba-sync.js';

export const DELETE: RequestHandler = async ({ locals }) => {
    if (!locals.memberSession) throw error(401, 'Non autorisé.');
    const pseudo = locals.memberSession.pseudo;

    const deleted = await db.delete(users)
        .where(sql`LOWER(${users.pseudo}) = LOWER(${pseudo})`)
        .returning({ pseudo: users.pseudo });

    if (deleted.length === 0) {
        return json({ ok: false, message: 'Utilisateur introuvable.' });
    }

    await deleteBaUser(pseudo); // révoque aussi les ba_sessions (cascade SQL)

    return json({ ok: true, message: 'Compte supprime.' });
};
