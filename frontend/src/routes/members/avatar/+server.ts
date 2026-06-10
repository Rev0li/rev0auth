import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { db } from '$lib/server/db/index.js';
import { users } from '$lib/server/db/schema.js';
import { eq } from 'drizzle-orm';
import { AVATAR_SVG, isAvatarId } from '$lib/avatars.js';

// Le client n'envoie jamais de fichier : seulement un id du catalogue $lib/avatars.
// Les bytes sont stockés en DB (le Rust web les sert encore pendant la migration).
export const POST: RequestHandler = async ({ request, locals }) => {
    if (!locals.memberSession) throw error(401, 'Non autorisé.');

    const { avatar_id } = await request.json().catch(() => ({}));
    if (!isAvatarId(avatar_id)) {
        return json({ ok: false, message: 'Avatar inconnu.' }, { status: 400 });
    }

    const svg = AVATAR_SVG[avatar_id];
    await db.update(users)
        .set({
            avatarBytes:     Buffer.from(svg),
            avatarMime:      'image/svg+xml',
            avatarFilename:  `${avatar_id}.svg`,
            avatarSizeBytes: svg.length,
        })
        .where(eq(users.pseudo, locals.memberSession.pseudo));

    return json({ ok: true });
};

export const DELETE: RequestHandler = async ({ locals }) => {
    if (!locals.memberSession) throw error(401, 'Non autorisé.');
    await db.update(users)
        .set({ avatarBytes: null, avatarMime: null, avatarFilename: null, avatarSizeBytes: null })
        .where(eq(users.pseudo, locals.memberSession.pseudo));
    return json({ ok: true });
};
