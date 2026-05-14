import { describe, it, expect, beforeEach } from 'vitest';

// Import module fresh each test by re-requiring to reset module state
// We test the logic by importing the functions directly
import { checkRateLimit, recordFailure, clearAttempts } from '../ratelimit.js';

describe('rate limiter', () => {
    const IP = '1.2.3.4';
    const UNKNOWN = '99.99.99.99';

    beforeEach(() => {
        clearAttempts(IP);
        clearAttempts(UNKNOWN);
    });

    it('allows fresh IPs', () => {
        expect(checkRateLimit(UNKNOWN).blocked).toBe(false);
    });

    it('blocks after 5 failures', () => {
        for (let i = 0; i < 5; i++) recordFailure(IP);
        expect(checkRateLimit(IP).blocked).toBe(true);
    });

    it('clears after success', () => {
        for (let i = 0; i < 5; i++) recordFailure(IP);
        clearAttempts(IP);
        expect(checkRateLimit(IP).blocked).toBe(false);
    });

    it('does not block after 4 failures', () => {
        for (let i = 0; i < 4; i++) recordFailure(IP);
        expect(checkRateLimit(IP).blocked).toBe(false);
    });
});
