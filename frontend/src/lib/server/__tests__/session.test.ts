import { describe, it, expect } from 'vitest';
import { generateToken } from '../auth.js';

// DB integration tests for sessions require a live PostgreSQL connection.
// Run against the real DB in development; tested end-to-end via the app.

describe('generateToken', () => {
    it('returns a hex string of the correct length', () => {
        const t = generateToken(32);
        expect(t).toMatch(/^[0-9a-f]{64}$/);
    });

    it('produces unique tokens', () => {
        const tokens = new Set(Array.from({ length: 100 }, () => generateToken(32)));
        expect(tokens.size).toBe(100);
    });
});
