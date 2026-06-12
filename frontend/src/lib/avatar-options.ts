// Catalogue du style DiceBear « adventurer » (API 10.x) — généré depuis la
// définition officielle @dicebear/styles. Partagé client (composeur du profil)
// et serveur (validation whitelist : l'API ignore silencieusement les valeurs
// inconnues, on refuse donc tout ce qui n'est pas listé ici).

const variants = (n: number) =>
    Array.from({ length: n }, (_, i) => `variant${String(i + 1).padStart(2, '0')}`);

export const HAIR = [
    ...Array.from({ length: 26 }, (_, i) => `long${String(i + 1).padStart(2, '0')}`),
    ...Array.from({ length: 19 }, (_, i) => `short${String(i + 1).padStart(2, '0')}`),
];
export const EYES     = variants(26);
export const MOUTH    = variants(30);
export const EYEBROWS = variants(15);
export const GLASSES  = variants(5);
export const EARRINGS = variants(6);
export const DETAILS  = ['birthmark', 'blush', 'freckles', 'mustache'];

export const HAIR_COLORS = ['0e0e0e', 'e5d7a3', 'b9a05f', '796a45', '85c2c6', '6a4e35', '562306', 'afafaf', '3eac2c', 'dba3be', '592454', 'ac6511', 'cb6820', 'ab2a18'];
export const SKIN_COLORS = ['f2d3b1', 'ecad80', '9e5622', '763900'];

// '' = aucun(e) pour les sections optionnelles (lunettes, boucles, détails)
export type AvatarOptions = {
    hair: string;
    eyes: string;
    mouth: string;
    eyebrows: string;
    glasses: string;
    earrings: string;
    details: string;
    hairColor: string;
    skinColor: string;
};

export const SECTIONS = [
    { key: 'hair',     label: 'Coiffure', values: HAIR,     optional: false },
    { key: 'eyes',     label: 'Yeux',     values: EYES,     optional: false },
    { key: 'eyebrows', label: 'Sourcils', values: EYEBROWS, optional: false },
    { key: 'mouth',    label: 'Bouche',   values: MOUTH,    optional: false },
    { key: 'glasses',  label: 'Lunettes', values: GLASSES,  optional: true  },
    { key: 'earrings', label: 'Boucles',  values: EARRINGS, optional: true  },
    { key: 'details',  label: 'Détails',  values: DETAILS,  optional: true  },
] as const;

export function defaultOptions(): AvatarOptions {
    return {
        hair: HAIR[0], eyes: EYES[0], mouth: MOUTH[0], eyebrows: EYEBROWS[0],
        glasses: '', earrings: '', details: '',
        hairColor: HAIR_COLORS[0], skinColor: SKIN_COLORS[0],
    };
}

export function randomOptions(): AvatarOptions {
    const pick = <T,>(arr: readonly T[]) => arr[Math.floor(Math.random() * arr.length)];
    const maybe = <T,>(arr: readonly T[], p: number) => (Math.random() < p ? pick(arr) : '');
    return {
        hair: pick(HAIR), eyes: pick(EYES), mouth: pick(MOUTH), eyebrows: pick(EYEBROWS),
        glasses: maybe(GLASSES, 0.3), earrings: maybe(EARRINGS, 0.3), details: maybe(DETAILS, 0.25),
        hairColor: pick(HAIR_COLORS), skinColor: pick(SKIN_COLORS),
    };
}

// Construit la query string validée ; null si une valeur n'est pas au catalogue
export function buildAvatarParams(opts: unknown): URLSearchParams | null {
    if (typeof opts !== 'object' || opts === null) return null;
    const o = opts as Record<string, unknown>;
    const params = new URLSearchParams();

    const req: [string, readonly string[]][] = [
        ['hair', HAIR], ['eyes', EYES], ['mouth', MOUTH], ['eyebrows', EYEBROWS],
    ];
    for (const [key, allowed] of req) {
        const v = o[key];
        if (typeof v !== 'string' || !allowed.includes(v)) return null;
        params.set(key, v);
    }

    const opt: [string, readonly string[]][] = [
        ['glasses', GLASSES], ['earrings', EARRINGS], ['details', DETAILS],
    ];
    for (const [key, allowed] of opt) {
        const v = o[key];
        if (v === '' || v === undefined || v === null) {
            params.set(`${key}Probability`, '0');
        } else if (typeof v === 'string' && allowed.includes(v)) {
            params.set(key, v);
            params.set(`${key}Probability`, '100');
        } else return null;
    }

    const colors: [string, readonly string[]][] = [
        ['hairColor', HAIR_COLORS], ['skinColor', SKIN_COLORS],
    ];
    for (const [key, allowed] of colors) {
        const v = o[key];
        if (typeof v !== 'string' || !allowed.includes(v)) return null;
        params.set(key, v);
    }

    return params;
}
