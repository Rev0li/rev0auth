import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { db } from '$lib/server/db/index.js';
import { users, sessions, messages, donations, wallPosts } from '$lib/server/db/schema.js';
import { eq, or, sql } from 'drizzle-orm';
import { deleteBaUser, syncBaRole } from '$lib/server/ba-sync.js';
import { writeAudit } from '$lib/server/audit.js';

function requireAdmin({ locals }: { locals: App.Locals }) {
    if (!locals.adminSession) throw error(401, 'Non autorisé.');
}

export const PUT: RequestHandler = async ({ request, locals, params }) => {
    requireAdmin({ locals });
    const updates = await request.json();

    delete updates.passwordHash;
    delete updates.pseudo;
    delete updates.avatarBytes;

    await db.update(users).set(updates).where(eq(users.pseudo, params.pseudo));
    if (typeof updates.role === 'string') await syncBaRole(params.pseudo, updates.role);

    return json({ ok: true });
};

export const DELETE: RequestHandler = async ({ locals, params }) => {
    requireAdmin({ locals });
    const pseudo = params.pseudo;

    const deleted = await db.delete(users)
        .where(sql`LOWER(${users.pseudo}) = LOWER(${pseudo})`)
        .returning({ pseudo: users.pseudo });
    if (deleted.length === 0) {
        return json({ ok: false, message: 'Utilisateur introuvable.' });
    }

    // Cascade : sessions, messages, donations, posts mur (le Rust oublie les posts mur)
    await db.delete(sessions).where(sql`LOWER(${sessions.pseudo}) = LOWER(${pseudo})`);
    await db.delete(messages).where(
        or(
            sql`LOWER(${messages.fromPseudo}) = LOWER(${pseudo})`,
            sql`LOWER(${messages.toPseudo})   = LOWER(${pseudo})`,
        ),
    );
    await db.delete(donations).where(sql`LOWER(${donations.pseudo}) = LOWER(${pseudo})`);
    await db.delete(wallPosts).where(sql`LOWER(${wallPosts.pseudo}) = LOWER(${pseudo})`);
    await deleteBaUser(pseudo); // ba_sessions + ba_accounts en cascade SQL

    await writeAudit('delete_user', locals.adminSession!.pseudo, deleted[0].pseudo, 'user and all data deleted');
    return json({ ok: true });
};
