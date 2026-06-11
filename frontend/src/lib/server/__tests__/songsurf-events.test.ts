import { describe, it, expect, beforeEach } from 'vitest';

import {
    checkEventsSecret,
    parseEvents,
    EVENT_TYPES,
    MAX_BATCH,
} from '../songsurf-events.js';

const SECRET = 'test-events-secret-0123456789abcdef';

function validEvent(overrides: Record<string, unknown> = {}) {
    return {
        source: 'songsurf',
        type: 'download_success',
        ts: '2026-06-11T12:00:00+00:00',
        pseudo: 'oliver',
        role: 'member',
        artist: 'Daft Punk',
        album: 'Discovery',
        title: 'One More Time',
        detail: {},
        ip: '100.64.0.1',
        ...overrides,
    };
}

describe('checkEventsSecret', () => {
    beforeEach(() => {
        process.env.SONGSURF_EVENTS_SECRET = SECRET;
    });

    it('accepte le bon secret', () => {
        expect(checkEventsSecret(SECRET)).toBe(true);
    });

    it('rejette un mauvais secret', () => {
        expect(checkEventsSecret('wrong')).toBe(false);
    });

    it('rejette un header absent', () => {
        expect(checkEventsSecret(null)).toBe(false);
    });

    it('rejette tout si le secret env est absent', () => {
        process.env.SONGSURF_EVENTS_SECRET = '';
        expect(checkEventsSecret(SECRET)).toBe(false);
        expect(checkEventsSecret('')).toBe(false);
    });
});

describe('parseEvents', () => {
    it('accepte un événement seul (objet)', () => {
        const { events, rejected } = parseEvents(validEvent());
        expect(rejected).toBe(0);
        expect(events).toHaveLength(1);
        expect(events[0].eventType).toBe('download_success');
        expect(events[0].pseudo).toBe('oliver');
        expect(events[0].eventTs).toBe(Math.floor(Date.parse('2026-06-11T12:00:00+00:00') / 1000));
    });

    it('accepte un batch (array) et compte les rejets', () => {
        const { events, rejected } = parseEvents([
            validEvent(),
            validEvent({ type: 'unknown_type' }),
            validEvent({ type: 'login_success' }),
            'pas un objet',
        ]);
        expect(events).toHaveLength(2);
        expect(rejected).toBe(2);
    });

    it('rejette les types hors whitelist', () => {
        const { events, rejected } = parseEvents(validEvent({ type: 'rm_rf' }));
        expect(events).toHaveLength(0);
        expect(rejected).toBe(1);
    });

    it('accepte tous les types de la whitelist', () => {
        for (const t of EVENT_TYPES) {
            expect(parseEvents(validEvent({ type: t })).events).toHaveLength(1);
        }
    });

    it('tronque les champs trop longs', () => {
        const { events } = parseEvents(validEvent({
            pseudo: 'x'.repeat(200),
            artist: 'y'.repeat(500),
            detail: { error: 'z'.repeat(5000) },
        }));
        expect(events[0].pseudo).toHaveLength(64);
        expect(events[0].artist).toHaveLength(300);
        expect((events[0].detail as string).length).toBeLessThanOrEqual(2048);
    });

    it('fallback sur maintenant si ts invalide', () => {
        const before = Math.floor(Date.now() / 1000);
        const { events } = parseEvents(validEvent({ ts: 'pas-une-date' }));
        expect(events[0].eventTs).toBeGreaterThanOrEqual(before);
    });

    it('sérialise detail en JSON string', () => {
        const { events } = parseEvents(validEvent({ detail: { count: 12, size_mb: 34.5 } }));
        expect(JSON.parse(events[0].detail as string)).toEqual({ count: 12, size_mb: 34.5 });
    });

    it('detail absent → "{}"', () => {
        const { events } = parseEvents(validEvent({ detail: undefined }));
        expect(events[0].detail).toBe('{}');
    });

    it('MAX_BATCH est cohérent avec le replay client (50 ≤ 100)', () => {
        expect(MAX_BATCH).toBeGreaterThanOrEqual(50);
    });
});
