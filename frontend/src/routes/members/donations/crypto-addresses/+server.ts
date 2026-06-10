import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';

type CryptoAddress = { name: string; address: string };

const FALLBACK: CryptoAddress[] = [
    { name: 'Bitcoin (BTC)',  address: 'À configurer' },
    { name: 'Ethereum (ETH)', address: 'À configurer' },
    { name: 'Solana (SOL)',   address: 'À configurer' },
];

function parseEnv(): CryptoAddress[] {
    const raw = (process.env.DONATION_CRYPTO_ADDRESSES ?? '').trim();
    if (!raw) return [];
    return raw.split(',')
        .map(entry => {
            const idx = entry.indexOf(':');
            if (idx < 0) return null;
            const name = entry.slice(0, idx).trim();
            const address = entry.slice(idx + 1).trim();
            if (!name || !address) return null;
            return { name, address };
        })
        .filter((x): x is CryptoAddress => x !== null);
}

export const GET: RequestHandler = async ({ locals }) => {
    if (!locals.memberSession) throw error(401, 'Non autorisé.');
    const fromEnv = parseEnv();
    return json(fromEnv.length > 0 ? fromEnv : FALLBACK);
};
