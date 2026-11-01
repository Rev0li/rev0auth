// Goûts musicaux des membres d'après leurs téléchargements SongSurf : fiches
// par membre (artistes classés, albums, derniers albums) et une fiche « tous ».
// Fonctions pures : le serveur les applique au journal complet de SongSurf.

export type DlRow = { pseudo: string; artist: string; album: string; title: string; ts: number };

export type AlbumStat = {
    album: string;
    artist: string;
    count: number;
    last: number;
    who: string[]; // membres qui l'ont téléchargé (fiche « tous »)
};

export type ArtistStat = { artist: string; count: number; last: number; albums: AlbumStat[] };

export type TasteProfile = {
    pseudo: string | null; // null = tout le monde
    titles: number;
    artistCount: number;
    albumCount: number;
    last: number;
    recentAlbums: AlbumStat[];
    artists: ArtistStat[];
};

export const NO_ALBUM = 'Titres seuls';
const UNKNOWN_ARTIST = 'Artiste inconnu';
const RECENT_ALBUMS = 6;

const key = (s: string) => s.trim().toLowerCase();

function profileOf(pseudo: string | null, rows: DlRow[]): TasteProfile {
    // artiste → album → stats ; clés insensibles à la casse, libellé du premier vu
    const artists = new Map<string, { artist: string; count: number; last: number; albums: Map<string, AlbumStat & { whoSet: Set<string> }> }>();
    for (const r of rows) {
        const artistName = r.artist.trim() || UNKNOWN_ARTIST;
        const albumName = r.album.trim() || NO_ALBUM;
        let a = artists.get(key(artistName));
        if (!a) artists.set(key(artistName), a = { artist: artistName, count: 0, last: 0, albums: new Map() });
        a.count++; a.last = Math.max(a.last, r.ts);
        let al = a.albums.get(key(albumName));
        if (!al) a.albums.set(key(albumName), al = { album: albumName, artist: artistName, count: 0, last: 0, who: [], whoSet: new Set() });
        al.count++; al.last = Math.max(al.last, r.ts);
        al.whoSet.add(r.pseudo);
    }

    const artistList: ArtistStat[] = [...artists.values()].map((a) => ({
        artist: a.artist,
        count: a.count,
        last: a.last,
        albums: [...a.albums.values()]
            .map(({ whoSet, ...al }) => ({ ...al, who: [...whoSet].sort((x, y) => x.localeCompare(y)) }))
            .sort((x, y) => y.last - x.last),
    }));
    // Plus téléchargés d'abord ; à égalité, le plus récent
    artistList.sort((x, y) => y.count - x.count || y.last - x.last || x.artist.localeCompare(y.artist));

    const albums = artistList.flatMap((a) => a.albums);
    return {
        pseudo,
        titles: rows.length,
        artistCount: artistList.length,
        albumCount: albums.filter((al) => al.album !== NO_ALBUM).length,
        last: rows.reduce((m, r) => Math.max(m, r.ts), 0),
        recentAlbums: albums
            .filter((al) => al.album !== NO_ALBUM)
            .sort((x, y) => y.last - x.last)
            .slice(0, RECENT_ALBUMS),
        artists: artistList,
    };
}

// Une fiche par membre (les plus actifs d'abord) + la fiche de tout le monde
export function buildTasteProfiles(rows: DlRow[]): { all: TasteProfile; members: TasteProfile[] } {
    const byMember = new Map<string, { pseudo: string; rows: DlRow[] }>();
    for (const r of rows) {
        const pseudo = r.pseudo.trim();
        if (!pseudo) continue;
        let m = byMember.get(key(pseudo));
        if (!m) byMember.set(key(pseudo), m = { pseudo, rows: [] });
        m.rows.push({ ...r, pseudo: m.pseudo });
    }
    const members = [...byMember.values()]
        .map((m) => profileOf(m.pseudo, m.rows))
        .sort((x, y) => y.titles - x.titles || (x.pseudo ?? '').localeCompare(y.pseudo ?? ''));
    const all = profileOf(null, [...byMember.values()].flatMap((m) => m.rows));
    return { all, members };
}
