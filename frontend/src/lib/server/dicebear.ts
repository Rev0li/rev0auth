// Avatars DiceBear "initial-face" (https://www.dicebear.com/styles/initial-face/)
// Le style n'existe que via l'API HTTP 10.x (pas de package npm) : on fetch
// côté serveur UNIQUEMENT au choix d'avatar / aperçu de grille, avec cache
// mémoire. Les avatars choisis sont stockés en DB (web_users.avatar_bytes)
// et servis localement — aucun appel tiers côté visiteur.

const API = 'https://api.dicebear.com/10.x/initial-face/svg';

export const SEED_RE = /^[a-zA-Z0-9_-]{1,48}$/;

const cache = new Map<string, string>();
const CACHE_MAX = 300;

export async function fetchAvatarSvg(seed: string): Promise<string | null> {
    if (!SEED_RE.test(seed)) return null;

    const hit = cache.get(seed);
    if (hit) return hit;

    try {
        const ctrl = new AbortController();
        const timer = setTimeout(() => ctrl.abort(), 5000);
        const res = await fetch(`${API}?seed=${encodeURIComponent(seed)}`, { signal: ctrl.signal });
        clearTimeout(timer);
        if (!res.ok) return null;
        const svg = await res.text();
        if (!svg.startsWith('<svg')) return null;

        if (cache.size >= CACHE_MAX) {
            const first = cache.keys().next().value;
            if (first !== undefined) cache.delete(first);
        }
        cache.set(seed, svg);
        return svg;
    } catch {
        return null;
    }
}
