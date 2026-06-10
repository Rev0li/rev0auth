import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { db } from '$lib/server/db/index.js';
import { wallPosts } from '$lib/server/db/schema.js';
import { eq } from 'drizzle-orm';
import { writeAudit } from '$lib/server/audit.js';

export const DELETE: RequestHandler = async ({ params, locals }) => {
    if (!locals.adminSession) throw error(401, 'Non autorisé.');
    const id = parseInt(params.id ?? '');
    if (isNaN(id)) return json({ ok: false, message: 'ID invalide.' }, { status: 400 });

    const deleted = await db.delete(wallPosts)
        .where(eq(wallPosts.id, id))
        .returning({ id: wallPosts.id });

    if (deleted.length === 0) {
        return json({ ok: false, message: 'Introuvable.' });
    }
    await writeAudit('wall_delete', locals.adminSession.pseudo, String(id), 'wall post removed');
    return json({ ok: true, message: 'Message supprimé.' });
};
