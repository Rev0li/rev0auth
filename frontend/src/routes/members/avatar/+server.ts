import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { db } from '$lib/server/db/index.js';
import { users } from '$lib/server/db/schema.js';
import { eq } from 'drizzle-orm';

const ALLOWED_MIME = new Set(['image/jpeg', 'image/png', 'image/webp', 'image/gif', 'image/svg+xml']);
const MAX_BYTES    = 512 * 1024; // 512 KB

export const POST: RequestHandler = async ({ request, locals }) => {
    if (!locals.memberSession) throw error(401, 'Non autorisé.');

    const formData = await request.formData();
    const file = formData.get('avatar') as File | null;
    if (!file) return json({ ok: false, message: 'Fichier requis.' }, { status: 400 });
    if (!ALLOWED_MIME.has(file.type)) return json({ ok: false, message: 'Format non supporté.' }, { status: 415 });

    const buf = Buffer.from(await file.arrayBuffer());
    if (buf.byteLength > MAX_BYTES) return json({ ok: false, message: 'Fichier trop volumineux (max 512 KB).' }, { status: 413 });

    await db.update(users)
        .set({ avatarBytes: buf, avatarMime: file.type })
        .where(eq(users.pseudo, locals.memberSession.pseudo));

    return json({ ok: true });
};

export const DELETE: RequestHandler = async ({ locals }) => {
    if (!locals.memberSession) throw error(401, 'Non autorisé.');
    await db.update(users)
        .set({ avatarBytes: null, avatarMime: null })
        .where(eq(users.pseudo, locals.memberSession.pseudo));
    return json({ ok: true });
};
