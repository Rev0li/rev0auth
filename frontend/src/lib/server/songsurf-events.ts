import { createHash, timingSafeEqual } from 'node:crypto';
import type { NewSongsurfEvent } from '$lib/server/db/schema.js';

// Whitelist des types d'événements acceptés depuis le NAS (watcher + songsurf).
export const EVENT_TYPES = [
    'login_success',
    'login_rejected',
    'download_success',
    'download_failed',
    'zip_export',
    'container_start',
    'container_stop',
] as const;

export const MAX_BATCH = 100;
export const MAX_BODY_BYTES = 64 * 1024;

const EVENT_TYPE_SET = new Set<string>(EVENT_TYPES);

export function eventsSecret(): string {
    return (process.env.SONGSURF_EVENTS_SECRET ?? '').trim();
}

export function checkEventsSecret(header: string | null): boolean {
    const secret = eventsSecret();
    if (!secret || !header) return false;
    // sha256 des deux côtés : longueurs égales pour timingSafeEqual, pas de leak de longueur
    const a = createHash('sha256').update(header).digest();
    const b = createHash('sha256').update(secret).digest();
    return timingSafeEqual(a, b);
}

function asText(value: unknown, max: number): string {
    return typeof value === 'string' ? value.slice(0, max) : '';
}

function tsToEpoch(value: unknown): number {
    if (typeof value === 'string' && value) {
        const ms = Date.parse(value);
        if (!Number.isNaN(ms)) return Math.floor(ms / 1000);
    }
    if (typeof value === 'number' && Number.isFinite(value) && value > 0) {
        return Math.floor(value);
    }
    return Math.floor(Date.now() / 1000);
}

function parseOne(item: unknown): NewSongsurfEvent | null {
    if (typeof item !== 'object' || item === null || Array.isArray(item)) return null;
    const o = item as Record<string, unknown>;
    const eventType = typeof o.type === 'string' ? o.type : '';
    if (!EVENT_TYPE_SET.has(eventType)) return null;

    let detail = '{}';
    if (o.detail !== undefined && o.detail !== null) {
        try {
            detail = JSON.stringify(o.detail).slice(0, 2048);
        } catch {
            detail = '{}';
        }
    }

    return {
        source:    asText(o.source, 16),
        eventType,
        eventTs:   tsToEpoch(o.ts),
        pseudo:    asText(o.pseudo, 64),
        role:      asText(o.role, 64),
        artist:    asText(o.artist, 300),
        album:     asText(o.album, 300),
        title:     asText(o.title, 300),
        detail,
        ip:        asText(o.ip, 64),
    };
}

/** Accepte un événement seul ou un batch (array). Les items invalides sont comptés, pas bloquants. */
export function parseEvents(body: unknown): { events: NewSongsurfEvent[]; rejected: number } {
    const items = Array.isArray(body) ? body : [body];
    const events: NewSongsurfEvent[] = [];
    let rejected = 0;
    for (const item of items) {
        const parsed = parseOne(item);
        if (parsed) events.push(parsed);
        else rejected += 1;
    }
    return { events, rejected };
}
