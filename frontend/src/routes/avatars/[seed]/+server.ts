import { error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { fetchAvatarSvg, SEED_RE } from '$lib/server/dicebear.js';

// Aperçus DiceBear pour les grilles de sélection (profil + signup).
// Proxy serveur : le navigateur ne parle jamais à api.dicebear.com.
export const GET: RequestHandler = async ({ params }) => {
    const seed = params.seed ?? '';
    if (!SEED_RE.test(seed)) throw error(400, 'Seed invalide.');

    const svg = await fetchAvatarSvg(seed);
    if (!svg) throw error(502, 'Génération indisponible.');

    return new Response(svg, {
        headers: {
            'content-type': 'image/svg+xml',
            'cache-control': 'public, max-age=86400, immutable',
        },
    });
};
