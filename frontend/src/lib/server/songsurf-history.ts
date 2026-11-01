import { and, eq, min, ne } from 'drizzle-orm';
import { db } from './db/index.js';
import { songsurfEvents } from './db/schema.js';
import { songsurfConfigured, fetchSongsurfDlLogs } from './songsurf.js';
import { IMPORT_SOURCE, importKey, parisWallToEpoch, planImport } from './songsurf-import.js';
import type { DlRow } from '$lib/songsurf-taste.js';

// Historique des téléchargements SongSurf, stocké sur le portail (VPS) : les
// évènements `download_success` poussés par le NAS, plus l'ancien historique
// importé une fois (songsurf-import.ts). Lisible même quand le NAS, le
// Watcher ou SongSurf sont arrêtés.

const isDownload = eq(songsurfEvents.eventType, 'download_success');

export type HistoryStatus = { total: number; imported: number; since: number | null };

// Tous les téléchargements stockés + l'état de l'historique (une seule lecture)
export async function downloadHistory(): Promise<{ rows: DlRow[]; status: HistoryStatus }> {
    const rows = await db.select({
        source: songsurfEvents.source,
        pseudo: songsurfEvents.pseudo, artist: songsurfEvents.artist,
        album: songsurfEvents.album, title: songsurfEvents.title, ts: songsurfEvents.eventTs,
    }).from(songsurfEvents).where(isDownload);
    return {
        rows: rows.map(({ source: _source, ...r }) => r),
        status: {
            total: rows.length,
            imported: rows.filter((r) => r.source === IMPORT_SOURCE).length,
            since: rows.length ? rows.reduce((m, r) => Math.min(m, r.ts), Infinity) : null,
        },
    };
}

export type ImportResult =
    | { ok: true; imported: number; total: number }
    | { ok: false; message: string };

type DlEntry = { album?: string; artist?: string; pseudo?: string; timestamp?: string; title?: string };

// Importe dans la base les téléchargements du journal SongSurf antérieurs au
// premier évènement poussé. SongSurf doit répondre à ce moment-là.
export async function importSongsurfLog(cookieHeader: string | null): Promise<ImportResult> {
    if (!songsurfConfigured()) return { ok: false, message: 'SongSurf non configuré.' };

    let entries: DlEntry[];
    try {
        // SongSurf peut dormir : le Watcher le réveille, d'où le délai généreux
        const resp = await fetchSongsurfDlLogs(cookieHeader, '', 'all', 30_000);
        if (!resp.ok) return { ok: false, message: `SongSurf a répondu ${resp.status} : réessaie dans un instant.` };
        const data = (await resp.json().catch(() => null)) as { entries?: DlEntry[] } | null;
        if (!Array.isArray(data?.entries)) return { ok: false, message: 'Réponse SongSurf illisible.' };
        entries = data.entries;
    } catch {
        return { ok: false, message: 'SongSurf ne répond pas (NAS ou Watcher arrêté ?) : réessaie plus tard.' };
    }

    const logRows: DlRow[] = [];
    for (const e of entries) {
        const ts = parisWallToEpoch(e.timestamp);
        if (ts === null) continue;
        // Mêmes longueurs que les évènements poussés (songsurf-events.ts) : la
        // comparaison avec les lignes déjà importées reste exacte
        logRows.push({
            pseudo: (e.pseudo ?? '').slice(0, 64), artist: (e.artist ?? '').slice(0, 300),
            album: (e.album ?? '').slice(0, 300), title: (e.title ?? '').slice(0, 300), ts,
        });
    }

    const [{ first }] = await db.select({ first: min(songsurfEvents.eventTs) })
        .from(songsurfEvents).where(and(isDownload, ne(songsurfEvents.source, IMPORT_SOURCE)));
    const imported = await db.select({
        pseudo: songsurfEvents.pseudo, artist: songsurfEvents.artist,
        album: songsurfEvents.album, title: songsurfEvents.title, ts: songsurfEvents.eventTs,
    }).from(songsurfEvents).where(and(isDownload, eq(songsurfEvents.source, IMPORT_SOURCE)));

    const todo = planImport(logRows, first ?? null, new Set(imported.map(importKey)));
    for (let i = 0; i < todo.length; i += 500) {
        await db.insert(songsurfEvents).values(todo.slice(i, i + 500).map((r) => ({
            source: IMPORT_SOURCE, eventType: 'download_success', eventTs: r.ts, receivedAt: r.ts,
            pseudo: r.pseudo, artist: r.artist, album: r.album, title: r.title,
        })));
    }
    return { ok: true, imported: todo.length, total: logRows.length };
}
