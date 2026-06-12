import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { db } from '$lib/server/db/index.js';
import { users } from '$lib/server/db/schema.js';
import { eq } from 'drizzle-orm';
import { fetchAvatarSvg, SEED_RE } from '$lib/server/dicebear.js';
import { buildAvatarParams } from '$lib/avatar-options.js';

// Le client n'envoie jamais de fichier : un seed DiceBear + les options de
// composition (whitelist $lib/avatar-options). Le SVG est généré côté
// serveur (adventurer) et stocké en DB.
export const POST: RequestHandler = async ({ request, locals }) => {
    if (!locals.memberSession) throw error(401, 'Non autorisé.');

    const { seed, options } = await request.json().catch(() => ({}));
    if (typeof seed !== 'string' || !SEED_RE.test(seed)) {
        return json({ ok: false, message: 'Avatar invalide.' }, { status: 400 });
    }

    let params: URLSearchParams | undefined;
    if (options !== undefined && options !== null) {
        const built = buildAvatarParams(options);
        if (!built) return json({ ok: false, message: 'Options invalides.' }, { status: 400 });
        params = built;
    }

    const svg = await fetchAvatarSvg(seed, params);
    if (!svg) return json({ ok: false, message: 'Génération indisponible, réessaie.' }, { status: 502 });

    await db.update(users)
        .set({
            avatarBytes:     Buffer.from(svg),
            avatarMime:      'image/svg+xml',
            avatarFilename:  `${seed}.svg`,
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
