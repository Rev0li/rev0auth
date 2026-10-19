import { describe, it, expect } from 'vitest';
import { createHash } from 'node:crypto';
import { normalizeSetupCode, hashSetupCode } from '../admin-passkey.js';

// Le code est généré et haché par scripts/admin-setup.mjs (16 caractères en
// majuscules, affichés par groupes de 4) puis saisi à la main dans la console.

describe("codes d'installation admin", () => {
    it('ignore la casse, les espaces et les tirets', () => {
        expect(normalizeSetupCode(' abcd-efgh jklm-NPQR ')).toBe('ABCDEFGHJKLMNPQR');
    });

    it('hache comme scripts/admin-setup.mjs', () => {
        const stored = createHash('sha256').update('ABCDEFGHJKLMNPQR').digest('hex');
        expect(hashSetupCode('abcd-efgh-jklm-npqr')).toBe(stored);
    });

    it('distingue deux codes différents', () => {
        expect(hashSetupCode('ABCD-EFGH-JKLM-NPQR')).not.toBe(hashSetupCode('ABCD-EFGH-JKLM-NPQS'));
    });
});
