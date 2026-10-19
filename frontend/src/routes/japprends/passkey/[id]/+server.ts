import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { deletePasskey } from '$lib/server/admin-passkey.js';
import { writeAudit } from '$lib/server/audit.js';

export const DELETE: RequestHandler = async ({ locals, params }) => {
    if (!locals.adminSession || !locals.memberSession) throw error(401, 'Non autorisé.');
    const id = Number(params.id);
    if (!Number.isSafeInteger(id)) throw error(400, 'Identifiant invalide.');

    const name = await deletePasskey(locals.memberSession.userId, id);
    if (name === null) throw error(404, 'Clé introuvable.');
    await writeAudit('admin_passkey_deleted', locals.adminSession.pseudo, name);
    return json({ ok: true });
};
