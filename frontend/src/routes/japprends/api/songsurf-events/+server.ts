import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { db } from '$lib/server/db/index.js';
import { songsurfEvents } from '$lib/server/db/schema.js';
import {
    checkEventsSecret,
    eventsSecret,
    parseEvents,
    MAX_BATCH,
    MAX_BODY_BYTES,
} from '$lib/server/songsurf-events.js';

// Ingestion des événements d'activité poussés par le NAS (watcher + songsurf).
// Auth par secret partagé (X-Events-Secret), jamais par session admin :
// le body n'est lu qu'après validation du secret.
export const POST: RequestHandler = async ({ request }) => {
    if (!eventsSecret()) {
        return json({ success: false, error: 'Ingestion non configurée' }, { status: 503 });
    }
    if (!checkEventsSecret(request.headers.get('x-events-secret'))) {
        return json({ success: false, error: 'Non autorisé' }, { status: 401 });
    }

    const declared = parseInt(request.headers.get('content-length') ?? '0') || 0;
    if (declared > MAX_BODY_BYTES) {
        return json({ success: false, error: 'Payload trop volumineux' }, { status: 413 });
    }
    const raw = await request.text();
    if (raw.length > MAX_BODY_BYTES) {
        return json({ success: false, error: 'Payload trop volumineux' }, { status: 413 });
    }

    let body: unknown;
    try {
        body = JSON.parse(raw);
    } catch {
        return json({ success: false, error: 'JSON invalide' }, { status: 400 });
    }
    if (Array.isArray(body) && body.length > MAX_BATCH) {
        return json({ success: false, error: `Batch limité à ${MAX_BATCH}` }, { status: 413 });
    }

    const { events, rejected } = parseEvents(body);
    if (events.length > 0) {
        await db.insert(songsurfEvents).values(events);
    }
    return json({ success: true, accepted: events.length, rejected });
};
