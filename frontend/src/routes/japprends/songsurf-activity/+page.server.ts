import { redirect } from '@sveltejs/kit';
import { db } from '$lib/server/db/index.js';
import { songsurfEvents } from '$lib/server/db/schema.js';
import type { SongsurfEvent } from '$lib/server/db/schema.js';
import { and, desc, eq, inArray } from 'drizzle-orm';
import { EVENT_TYPES } from '$lib/server/songsurf-events.js';
import { songsurfConfigured, songsurfBaseUrl, signSongsurfJwt } from '$lib/server/songsurf.js';
import type { PageServerLoad } from './$types.js';

// La table songsurf_events n'est remplie que par le push du NAS. Les
// téléchargements, eux, vivent dans le journal interne de SongSurf : on les
// récupère en direct (même proxy que /songsurf-logs) et on les fusionne dans
// la section "Téléchargements & exports" pour que le dashboard les montre même
// si le pipeline push n'est pas (encore) en place.
type DlEntry = { album?: string; artist?: string; pseudo?: string; timestamp?: string; title?: string };

// Le NAS loggue une heure murale "Y-M-D H:M:S" en Europe/Paris (sans fuseau).
// On la convertit en epoch en retranchant l'offset Paris réel à cet instant
// (gère l'heure d'été via Intl), indépendamment du fuseau du serveur VPS.
function parisWallToEpoch(s?: string): number {
    const now = () => Math.floor(Date.now() / 1000);
    if (!s) return now();
    const m = s.match(/(\d{4})-(\d{2})-(\d{2})[ T](\d{2}):(\d{2}):(\d{2})/);
    if (!m) { const t = Date.parse(s); return Number.isNaN(t) ? now() : Math.floor(t / 1000); }
    const [Y, Mo, D, h, mi, se] = m.slice(1).map(Number);
    const utcGuess = Date.UTC(Y, Mo - 1, D, h, mi, se);
    const dtf = new Intl.DateTimeFormat('en-US', {
        timeZone: 'Europe/Paris', hour12: false,
        year: 'numeric', month: '2-digit', day: '2-digit',
        hour: '2-digit', minute: '2-digit', second: '2-digit',
    });
    const f: Record<string, number> = {};
    for (const p of dtf.formatToParts(new Date(utcGuess))) if (p.type !== 'literal') f[p.type] = Number(p.value);
    const offsetMs = Date.UTC(f.year, f.month - 1, f.day, f.hour, f.minute, f.second) - utcGuess;
    return Math.floor((utcGuess - offsetMs) / 1000);
}

async function fetchDownloads(pseudo: string, limit: number): Promise<SongsurfEvent[]> {
    if (!songsurfConfigured()) return [];
    try {
        const token = await signSongsurfJwt('admin', 'admin', 120);
        const target = `${songsurfBaseUrl()}/api/admin/dl-logs?pseudo=${encodeURIComponent(pseudo)}&limit=${limit}`;
        const ctrl = new AbortController();
        const timer = setTimeout(() => ctrl.abort(), 8000);
        const resp = await fetch(target, { headers: { Cookie: `access_token=${token}` }, signal: ctrl.signal });
        clearTimeout(timer);
        if (!resp.ok) return [];
        const data = (await resp.json().catch(() => null)) as { entries?: DlEntry[] } | null;
        const entries = Array.isArray(data?.entries) ? data!.entries : [];
        return entries.map((e, i) => ({
            id:         -(i + 1), // id synthétique négatif : pas de collision avec les ids DB
            source:     'songsurf',
            eventType:  'download_success',
            eventTs:    parisWallToEpoch(e.timestamp),
            pseudo:     e.pseudo ?? '',
            role:       '',
            artist:     e.artist ?? '',
            album:      e.album ?? '',
            title:      e.title ?? '',
            detail:     '{}',
            ip:         '',
            receivedAt: parisWallToEpoch(e.timestamp),
        }));
    } catch {
        return [];
    }
}

export const load: PageServerLoad = async ({ locals, url }) => {
    if (!locals.adminSession) throw redirect(303, '/japprends/login');

    const pseudo = url.searchParams.get('pseudo') ?? '';
    const type   = url.searchParams.get('type') ?? '';
    const limit  = Math.min(parseInt(url.searchParams.get('limit') ?? '100') || 100, 500);

    const filters = [
        pseudo ? eq(songsurfEvents.pseudo, pseudo) : undefined,
        type ? eq(songsurfEvents.eventType, type) : undefined,
    ].filter(Boolean);

    const pushed = await db.select().from(songsurfEvents)
        .where(filters.length ? and(...filters) : undefined)
        .orderBy(desc(songsurfEvents.receivedAt), desc(songsurfEvents.id))
        .limit(limit);

    // Pull live des téléchargements (seulement si le filtre type le permet —
    // le journal SongSurf ne contient que des téléchargements réussis).
    const pulled = (!type || type === 'download_success')
        ? await fetchDownloads(pseudo, limit)
        : [];

    const events = [...pushed, ...pulled]
        .sort((a, b) => b.receivedAt - a.receivedAt || b.id - a.id)
        .slice(0, limit);

    const logins = await db.select().from(songsurfEvents)
        .where(inArray(songsurfEvents.eventType, ['login_success', 'login_rejected']))
        .orderBy(desc(songsurfEvents.receivedAt), desc(songsurfEvents.id))
        .limit(50);

    const dbPseudos = (await db.selectDistinct({ pseudo: songsurfEvents.pseudo })
        .from(songsurfEvents))
        .map(r => r.pseudo);
    const pseudos = [...new Set([...dbPseudos, ...pulled.map(e => e.pseudo)])]
        .filter(p => p !== '')
        .sort();

    return { events, logins, pseudos, filterPseudo: pseudo, filterType: type, limit, eventTypes: EVENT_TYPES };
};
