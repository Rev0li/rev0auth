import { error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { fetchAvatarSvg, SEED_RE } from '$lib/server/dicebear.js';
import { buildAvatarParams } from '$lib/avatar-options.js';

// Aperçus DiceBear pour les grilles/composeur (profil + signup).
// Sans query params : avatar aléatoire dérivé du seed. Avec params :
// composition validée par whitelist (cf $lib/avatar-options).
// Proxy serveur : le navigateur ne parle jamais à api.dicebear.com.
export const GET: RequestHandler = async ({ params, url }) => {
    const seed = params.seed ?? '';
    if (!SEED_RE.test(seed)) throw error(400, 'Seed invalide.');

    let options: URLSearchParams | undefined;
    if ([...url.searchParams.keys()].some((k) => k !== 'v')) {
        const built = buildAvatarParams(Object.fromEntries(url.searchParams));
        if (!built) throw error(400, 'Options invalides.');
        options = built;
    }

    const svg = await fetchAvatarSvg(seed, options);
    if (!svg) throw error(502, 'Génération indisponible.');

    return new Response(svg, {
        headers: {
            'content-type': 'image/svg+xml',
            'cache-control': 'public, max-age=86400, immutable',
        },
    });
};
