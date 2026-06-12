import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { db } from '$lib/server/db/index.js';
import { users } from '$lib/server/db/schema.js';
import { sql } from 'drizzle-orm';
import { hashPassword } from '$lib/server/auth.js';
import { setBaPassword, removeBaPassword } from '$lib/server/ba-sync.js';
import { writeAudit } from '$lib/server/audit.js';

export const POST: RequestHandler = async ({ request, locals, params }) => {
    if (!locals.adminSession) throw error(401, 'Non autorisé.');
    const { password } = await request.json();
    if (!password) return json({ ok: false, message: 'Mot de passe requis.' }, { status: 400 });

    const hash = await hashPassword(password);
    const updated = await db.update(users)
        .set({ passwordHash: hash })
        .where(sql`LOWER(${users.pseudo}) = LOWER(${params.pseudo})`)
        .returning({ pseudo: users.pseudo, role: users.role });

    if (updated.length === 0) {
        return json({ ok: false, message: 'Utilisateur introuvable.' });
    }

    await setBaPassword(updated[0].pseudo, hash, updated[0].role);
    await writeAudit('set_password', locals.adminSession.pseudo, updated[0].pseudo, 'password replaced');
    return json({ ok: true });
};

export const DELETE: RequestHandler = async ({ locals, params }) => {
    if (!locals.adminSession) throw error(401, 'Non autorisé.');
    const pseudo = params.pseudo!;

    const found = await db.select({ passwordHash: users.passwordHash })
        .from(users)
        .where(sql`LOWER(${users.pseudo}) = LOWER(${pseudo})`)
        .limit(1);

    if (found.length === 0) {
        return json({ ok: false, message: 'Utilisateur introuvable.' });
    }
    if (!found[0].passwordHash) {
        return json({ ok: false, message: 'Pas de mot de passe pour cet utilisateur.' });
    }

    await db.update(users)
        .set({ passwordHash: '' })
        .where(sql`LOWER(${users.pseudo}) = LOWER(${pseudo})`);
    await removeBaPassword(pseudo);

    await writeAudit('remove_password', locals.adminSession.pseudo, pseudo, 'password cleared');
    return json({ ok: true, message: 'Mot de passe supprime.' });
};
