import { describe, it, expect } from 'vitest';
import { planImport, importKey, parisWallToEpoch } from '../songsurf-import.js';
import type { DlRow } from '$lib/songsurf-taste.js';

const row = (pseudo: string, title: string, ts: number): DlRow =>
    ({ pseudo, artist: 'Air', album: 'Moon Safari', title, ts });

describe('reprise de l\'ancien historique SongSurf', () => {
    const LOG = [row('alice', 'La Femme d\'Argent', 1000), row('bob', 'Sexy Boy', 2000), row('alice', 'Kelly', 3000)];

    it('sans évènement poussé : tout le journal', () => {
        expect(planImport(LOG, null, new Set())).toHaveLength(3);
    });

    it('seulement avant le premier évènement poussé (marge de quelques secondes)', () => {
        // premier évènement poussé à 3002 : la ligne 3000 est le même téléchargement
        expect(planImport(LOG, 3002, new Set()).map((r) => r.ts)).toEqual([1000, 2000]);
    });

    it('relance : les lignes déjà importées ne reviennent pas (casse du pseudo ignorée)', () => {
        const done = new Set([importKey(row('Alice', 'La Femme d\'Argent', 1000))]);
        expect(planImport(LOG, null, done).map((r) => r.ts)).toEqual([2000, 3000]);
    });

    it('doublons dans le journal et lignes sans pseudo ignorés', () => {
        expect(planImport([row('bob', 'Sexy Boy', 2000), row('bob', 'Sexy Boy', 2000), row(' ', 'x', 5)], null, new Set()))
            .toHaveLength(1);
    });
});

describe('heure murale de Paris → epoch', () => {
    it('heure d\'hiver (UTC+1) et d\'été (UTC+2)', () => {
        expect(parisWallToEpoch('2026-01-15 13:00:00')).toBe(Date.UTC(2026, 0, 15, 12) / 1000);
        expect(parisWallToEpoch('2026-07-15 13:00:00')).toBe(Date.UTC(2026, 6, 15, 11) / 1000);
    });

    it('minuit', () => {
        expect(parisWallToEpoch('2026-07-15 00:00:00')).toBe(Date.UTC(2026, 6, 14, 22) / 1000);
    });

    it('illisible → null', () => {
        expect(parisWallToEpoch('hier')).toBeNull();
        expect(parisWallToEpoch(undefined)).toBeNull();
    });
});
