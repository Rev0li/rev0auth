// Qui est en ligne en ce moment (Activité › Journal). En mémoire seulement :
// chaque requête d'un membre connecté (portail, console, et chaque passage
// par la porte d'un service, /auth/verify) met à jour sa dernière activité à
// cet endroit ; les pages connectées envoient en plus un battement par minute
// (/auth/ping), pour qu'un onglet ouvert mais inactif compte. Repart de zéro
// au redémarrage du portail (déploiement) : c'est une vue « en direct », pas
// un historique.

export type Place = 'portail' | 'console' | string; // + nom du service (songsurf…)

type Seen = { pseudo: string; role: string; places: Map<Place, number> };

// En ligne = activité depuis moins de 5 min ; gardé 24 h pour « plus tôt »
export const ONLINE_WINDOW_S = 5 * 60;
const KEEP_S = 24 * 60 * 60;

const seen = new Map<string, Seen>();
const now = () => Math.floor(Date.now() / 1000);

export function touch(userId: string, pseudo: string, role: string, place: Place, at = now()): void {
    let s = seen.get(userId);
    if (!s) seen.set(userId, s = { pseudo, role, places: new Map() });
    s.pseudo = pseudo; s.role = role;
    s.places.set(place, at);
}

export type Presence = {
    pseudo: string;
    role: string;
    last: number;
    online: boolean;
    places: { place: Place; last: number; online: boolean }[];
};

export function presence(at = now()): Presence[] {
    const out: Presence[] = [];
    for (const [id, s] of seen) {
        for (const [p, t] of s.places) if (at - t > KEEP_S) s.places.delete(p);
        if (s.places.size === 0) { seen.delete(id); continue; }
        const places = [...s.places]
            .map(([place, last]) => ({ place, last, online: at - last <= ONLINE_WINDOW_S }))
            .sort((a, b) => b.last - a.last);
        const last = places[0].last;
        out.push({ pseudo: s.pseudo, role: s.role, last, online: at - last <= ONLINE_WINDOW_S, places });
    }
    return out.sort((a, b) => b.last - a.last);
}

// Pour les tests
export function clearPresence(): void { seen.clear(); }
