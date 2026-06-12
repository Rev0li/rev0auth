// Avatars DiceBear "initial-face" (https://www.dicebear.com/styles/initial-face/)
// Visage dérivé de l'initiale du seed ; composable : yeux, tête, couleur de
// fond (cf $lib/avatar-options). Pas de package npm v10 : on fetch l'API HTTP
// côté serveur UNIQUEMENT au choix d'avatar / aperçu, avec cache mémoire.
// Les avatars choisis sont stockés en DB (web_users.avatar_bytes) et servis
// localement — aucun appel tiers côté visiteur. Licence style : CC0.

const API = 'https://api.dicebear.com/10.x/initial-face/svg';

export const SEED_RE = /^[a-zA-Z0-9_-]{1,48}$/;

const cache = new Map<string, string>();
const CACHE_MAX = 300;

// `params` : query string déjà validée par buildAvatarParams (whitelist) —
// l'API DiceBear ignore silencieusement les valeurs inconnues, la validation
// est donc de notre responsabilité.
export async function fetchAvatarSvg(seed: string, params?: URLSearchParams): Promise<string | null> {
    if (!SEED_RE.test(seed)) return null;

    const qs = params ? `&${params.toString()}` : '';
    const key = seed + qs;
    const hit = cache.get(key);
    if (hit) return hit;

    try {
        const ctrl = new AbortController();
        const timer = setTimeout(() => ctrl.abort(), 5000);
        const res = await fetch(`${API}?seed=${encodeURIComponent(seed)}${qs}`, { signal: ctrl.signal });
        clearTimeout(timer);
        if (!res.ok) return null;
        const svg = await res.text();
        if (!svg.startsWith('<svg')) return null;

        if (cache.size >= CACHE_MAX) {
            const first = cache.keys().next().value;
            if (first !== undefined) cache.delete(first);
        }
        cache.set(key, svg);
        return svg;
    } catch {
        return null;
    }
}
