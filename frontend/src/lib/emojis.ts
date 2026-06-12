// 15 emojis OpenMoji (https://openmoji.org — CC BY-SA 4.0)
// SVG auto-hébergés dans static/openmoji/ (pas de CDN tiers).
// L'insertion dans les messages reste le caractère unicode ; EmojiText.svelte
// remplace ces caractères par leur rendu OpenMoji à l'affichage.

export type EmojiDef = { char: string; src: string; name: string };

export const EMOJIS: EmojiDef[] = [
    { char: '😀', src: '/openmoji/1F600.svg', name: 'sourire' },
    { char: '🤣', src: '/openmoji/1F923.svg', name: 'mort de rire' },
    { char: '😍', src: '/openmoji/1F60D.svg', name: 'yeux en cœur' },
    { char: '🥳', src: '/openmoji/1F973.svg', name: 'fête' },
    { char: '🤔', src: '/openmoji/1F914.svg', name: 'réflexion' },
    { char: '👍', src: '/openmoji/1F44D.svg', name: 'pouce levé' },
    { char: '❤️', src: '/openmoji/2764.svg',  name: 'cœur' },
    { char: '🔥', src: '/openmoji/1F525.svg', name: 'feu' },
    { char: '🎉', src: '/openmoji/1F389.svg', name: 'confettis' },
    { char: '💡', src: '/openmoji/1F4A1.svg', name: 'idée' },
    { char: '🎬', src: '/openmoji/1F3AC.svg', name: 'clap cinéma' },
    { char: '🍿', src: '/openmoji/1F37F.svg', name: 'popcorn' },
    { char: '🎵', src: '/openmoji/1F3B5.svg', name: 'musique' },
    { char: '🚀', src: '/openmoji/1F680.svg', name: 'fusée' },
    { char: '🦄', src: '/openmoji/1F984.svg', name: 'licorne' },
];

const BY_CHAR = new Map(EMOJIS.map((e) => [e.char, e]));
// ❤️ peut arriver avec ou sans le variation selector U+FE0F
BY_CHAR.set('❤', BY_CHAR.get('❤️')!);

const SPLIT_RE = new RegExp(
    `(${[...BY_CHAR.keys()].sort((a, b) => b.length - a.length).map((c) => c.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')).join('|')})`,
    'gu'
);

// Découpe un texte en segments { text } | { emoji } pour rendu sans @html
export function splitEmojis(text: string): ({ text: string } | { emoji: EmojiDef })[] {
    return text
        .split(SPLIT_RE)
        .filter((part) => part !== '')
        .map((part) => {
            const emoji = BY_CHAR.get(part);
            return emoji ? { emoji } : { text: part };
        });
}
