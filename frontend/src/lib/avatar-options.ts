// Catalogue du style DiceBear « initial-face » (API 10.x) — généré depuis la
// définition officielle @dicebear/styles. Partagé client (composeur du profil)
// et serveur (validation whitelist : l'API ignore silencieusement les valeurs
// inconnues, on refuse donc tout ce qui n'est pas listé ici).
// Le visage reste dérivé de l'initiale du seed (pseudo). Licence : CC0.

export const EYES = ['variant01', 'variant02', 'variant03', 'variant04', 'variant05', 'variant06', 'variant07', 'variant08'];
export const HEAD = ['default', 'alt'];
export const BG_COLORS = ['ffa3b5', 'ffb382', 'ffe08a', 'e2f299', 'a8f0b0', '86eadf', '7fd4f5', '9cbeff', 'cfbeff'];

export type AvatarOptions = {
    eyes: string;
    head: string;
    backgroundColor: string;
};

export const SECTIONS = [
    { key: 'eyes', label: 'Yeux', values: EYES, optional: false },
    { key: 'head', label: 'Tête', values: HEAD, optional: false },
] as const;

export function defaultOptions(): AvatarOptions {
    return { eyes: EYES[0], head: HEAD[0], backgroundColor: BG_COLORS[0] };
}

export function randomOptions(): AvatarOptions {
    const pick = <T,>(arr: readonly T[]) => arr[Math.floor(Math.random() * arr.length)];
    return { eyes: pick(EYES), head: pick(HEAD), backgroundColor: pick(BG_COLORS) };
}

// Construit la query string validée ; null si une valeur n'est pas au catalogue
export function buildAvatarParams(opts: unknown): URLSearchParams | null {
    if (typeof opts !== 'object' || opts === null) return null;
    const o = opts as Record<string, unknown>;
    const params = new URLSearchParams();

    const fields: [string, readonly string[]][] = [
        ['eyes', EYES],
        ['head', HEAD],
        ['backgroundColor', BG_COLORS],
    ];
    for (const [key, allowed] of fields) {
        const v = o[key];
        if (typeof v !== 'string' || !allowed.includes(v)) return null;
        params.set(key, v);
    }

    return params;
}
