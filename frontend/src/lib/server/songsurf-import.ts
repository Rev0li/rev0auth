import type { DlRow } from '$lib/songsurf-taste.js';

// Reprise de l'ancien historique SongSurf (downloads.log du NAS) dans la base
// du portail. Depuis que le NAS pousse ses évènements, chaque téléchargement
// arrive dans songsurf_events ; seuls ceux d'avant manquent. On n'importe donc
// que les lignes antérieures au premier évènement poussé, et jamais deux fois
// la même ligne (relance sans risque).

// Marque des lignes importées (les évènements poussés ont source 'songsurf')
export const IMPORT_SOURCE = 'songsurf-log';
// Le journal et l'évènement poussé du même téléchargement peuvent différer
// d'une seconde ou deux : marge avant le premier évènement poussé.
const CUTOVER_MARGIN_S = 5;

export const importKey = (r: DlRow) =>
    [r.pseudo.trim().toLowerCase(), r.artist.trim(), r.album.trim(), r.title.trim(), r.ts].join('\u0000');

export function planImport(
    logRows: DlRow[],
    firstPushedTs: number | null,
    alreadyImported: Set<string>,
): DlRow[] {
    const cutover = firstPushedTs === null ? Infinity : firstPushedTs - CUTOVER_MARGIN_S;
    const seen = new Set(alreadyImported);
    const out: DlRow[] = [];
    for (const r of logRows) {
        if (!r.pseudo.trim() || r.ts >= cutover) continue;
        const k = importKey(r);
        if (seen.has(k)) continue;
        seen.add(k);
        out.push(r);
    }
    return out;
}

// Le NAS loggue une heure murale "Y-M-D H:M:S" en Europe/Paris (sans fuseau).
// On la convertit en epoch en retranchant l'offset Paris réel à cet instant
// (gère l'heure d'été via Intl), indépendamment du fuseau du serveur VPS.
// null si illisible : une ligne sans date n'est pas importée.
export function parisWallToEpoch(s?: string): number | null {
    if (!s) return null;
    const m = s.match(/(\d{4})-(\d{2})-(\d{2})[ T](\d{2}):(\d{2}):(\d{2})/);
    if (!m) return null;
    const [Y, Mo, D, h, mi, se] = m.slice(1).map(Number);
    const utcGuess = Date.UTC(Y, Mo - 1, D, h, mi, se);
    const dtf = new Intl.DateTimeFormat('en-US', {
        timeZone: 'Europe/Paris', hour12: false,
        year: 'numeric', month: '2-digit', day: '2-digit',
        hour: '2-digit', minute: '2-digit', second: '2-digit',
    });
    const f: Record<string, number> = {};
    for (const p of dtf.formatToParts(new Date(utcGuess))) if (p.type !== 'literal') f[p.type] = Number(p.value);
    const offsetMs = Date.UTC(f.year, f.month - 1, f.day, f.hour === 24 ? 0 : f.hour, f.minute, f.second) - utcGuess;
    return Math.floor((utcGuess - offsetMs) / 1000);
}
