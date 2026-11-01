import { describe, it, expect } from 'vitest';
import { buildTasteProfiles, NO_ALBUM, type DlRow } from '../songsurf-taste.js';

const row = (pseudo: string, artist: string, album: string, title: string, ts: number): DlRow =>
    ({ pseudo, artist, album, title, ts });

const ROWS: DlRow[] = [
    row('alice', 'Daft Punk', 'Discovery', 'One More Time', 100),
    row('alice', 'Daft Punk', 'Discovery', 'Aerodynamic', 101),
    row('Alice', 'daft punk', 'Homework', 'Da Funk', 300),
    row('alice', 'Khruangbin', '', 'Maria También', 200),
    row('bob', 'Daft Punk', 'Discovery', 'Digital Love', 400),
    row('bob', 'Tame Impala', 'Currents', 'Let It Happen', 50),
    row('', 'Personne', 'X', 'ignoré', 999),
];

describe('fiches de goûts SongSurf', () => {
    const { all, members } = buildTasteProfiles(ROWS);
    const alice = members.find((m) => m.pseudo === 'alice')!;

    it('une fiche par membre, insensible à la casse, les plus actifs d\'abord ; lignes sans pseudo ignorées', () => {
        expect(members.map((m) => [m.pseudo, m.titles])).toEqual([['alice', 4], ['bob', 2]]);
        expect(all.titles).toBe(6);
    });

    it('artistes classés par nombre de titres, albums regroupés sous l\'artiste', () => {
        expect(alice.artists.map((a) => [a.artist, a.count])).toEqual([['Daft Punk', 3], ['Khruangbin', 1]]);
        // albums du plus récent au plus ancien
        expect(alice.artists[0].albums.map((al) => [al.album, al.count])).toEqual([['Homework', 1], ['Discovery', 2]]);
        expect(alice.artistCount).toBe(2);
    });

    it('titre sans album rangé à part, hors des derniers albums et du compte d\'albums', () => {
        expect(alice.artists[1].albums[0].album).toBe(NO_ALBUM);
        expect(alice.recentAlbums.map((al) => al.album)).toEqual(['Homework', 'Discovery']);
        expect(alice.albumCount).toBe(2);
    });

    it('fiche « tous » : qui a téléchargé chaque album', () => {
        expect(all.pseudo).toBeNull();
        const discovery = all.artists[0].albums.find((al) => al.album === 'Discovery')!;
        expect(discovery.who).toEqual(['alice', 'bob']);
        expect(discovery.count).toBe(3);
        expect(all.recentAlbums[0]).toMatchObject({ album: 'Discovery', last: 400 });
    });

    it('historique vide', () => {
        const empty = buildTasteProfiles([]);
        expect(empty.members).toEqual([]);
        expect(empty.all).toMatchObject({ titles: 0, artists: [], recentAlbums: [] });
    });
});
