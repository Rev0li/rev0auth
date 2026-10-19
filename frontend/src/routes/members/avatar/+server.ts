import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { setAvatar, deleteAvatar } from '$lib/server/users.js';
import { fetchAvatarSvg, SEED_RE } from '$lib/server/dicebear.js';
import { buildAvatarParams, encodeAvatarFilename } from '$lib/avatar-options.js';

// Le client n'envoie jamais de fichier : un seed DiceBear (= le visage) +
// la couleur de fond (whitelist $lib/avatar-options). Le SVG est généré
// côté serveur (initial-face) et stocké en DB (user_avatars) ; le choix est
// encodé dans filename pour restaurer le composeur au chargement du profil.
export const POST: RequestHandler = async ({ request, locals }) => {
    if (!locals.memberSession) throw error(401, 'Non autorisé.');

    const { seed, options } = await request.json().catch(() => ({}));
    if (typeof seed !== 'string' || !SEED_RE.test(seed)) {
        return json({ ok: false, message: 'Avatar invalide.' }, { status: 400 });
    }

    const params = buildAvatarParams(options);
    if (!params) return json({ ok: false, message: 'Options invalides.' }, { status: 400 });

    const svg = await fetchAvatarSvg(seed, params);
    if (!svg) return json({ ok: false, message: 'Génération indisponible, réessaie.' }, { status: 502 });

    await setAvatar(locals.memberSession.userId, {
        svg,
        filename: encodeAvatarFilename(seed, params.get('backgroundColor')!),
    });

    return json({ ok: true });
};

export const DELETE: RequestHandler = async ({ locals }) => {
    if (!locals.memberSession) throw error(401, 'Non autorisé.');
    await deleteAvatar(locals.memberSession.userId);
    return json({ ok: true });
};
