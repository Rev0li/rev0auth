import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { db } from '$lib/server/db/index.js';
import { invites } from '$lib/server/db/schema.js';
import { eq } from 'drizzle-orm';
import { writeAudit } from '$lib/server/audit.js';

export const DELETE: RequestHandler = async ({ locals, params }) => {
    if (!locals.adminSession) throw error(401, 'Non autorisé.');
    const id = parseInt(params.id);
    if (isNaN(id)) return json({ ok: false }, { status: 400 });
    const [gone] = await db.delete(invites).where(eq(invites.id, id))
        .returning({ note: invites.note });
    if (gone) await writeAudit('delete_invite', locals.adminSession.pseudo, gone.note, `invite #${id}`);
    return json({ ok: true });
};
