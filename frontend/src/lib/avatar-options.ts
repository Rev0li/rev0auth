// Options du style DiceBear « initial-face » (API 10.x).
// Constat vérifié : l'API n'applique QUE backgroundColor — les variantes
// eyes/head listées dans la définition sont ignorées (MD5 identiques).
// Le visage (initiale, yeux, tête) est entièrement dérivé du seed.
// → Le composeur fait donc varier le SEED (visages déterministes par pseudo)
//   et la couleur de fond. Licence : CC0.

export const BG_COLORS = ['ffa3b5', 'ffb382', 'ffe08a', 'e2f299', 'a8f0b0', '86eadf', '7fd4f5', '9cbeff', 'cfbeff'];

// Nombre de visages proposés par pseudo (seed = pseudo, pseudo-2, …)
export const FACE_COUNT = 12;

export function faceSeeds(pseudo: string): string[] {
    const p = pseudo.toLowerCase();
    return Array.from({ length: FACE_COUNT }, (_, i) => (i === 0 ? p : `${p}-${i + 1}`));
}

export type AvatarOptions = { backgroundColor: string };

// Construit la query string validée ; null si la couleur n'est pas au catalogue
export function buildAvatarParams(opts: unknown): URLSearchParams | null {
    if (typeof opts !== 'object' || opts === null) return null;
    const v = (opts as Record<string, unknown>).backgroundColor;
    if (typeof v !== 'string' || !BG_COLORS.includes(v)) return null;
    const params = new URLSearchParams();
    params.set('backgroundColor', v);
    return params;
}

// Persistance du choix dans web_users.avatar_filename : "<seed>--<bg>.svg"
export function encodeAvatarFilename(seed: string, bg: string): string {
    return `${seed}--${bg}.svg`;
}

export function decodeAvatarFilename(filename: string | null | undefined): { seed: string; backgroundColor: string } | null {
    const m = filename?.match(/^([a-zA-Z0-9_-]{1,48})--([0-9a-f]{6})\.svg$/);
    if (!m || !BG_COLORS.includes(m[2])) return null;
    return { seed: m[1], backgroundColor: m[2] };
}
