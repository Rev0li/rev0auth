import { redirect } from '@sveltejs/kit';
import { db } from '$lib/server/db/index.js';
import { songsurfEvents } from '$lib/server/db/schema.js';
import { and, desc, eq, inArray } from 'drizzle-orm';
import { EVENT_TYPES } from '$lib/server/songsurf-events.js';
import type { PageServerLoad } from './$types.js';

export const load: PageServerLoad = async ({ locals, url }) => {
    if (!locals.adminSession) throw redirect(303, '/japprends/login');

    const pseudo = url.searchParams.get('pseudo') ?? '';
    const type   = url.searchParams.get('type') ?? '';
    const limit  = Math.min(parseInt(url.searchParams.get('limit') ?? '100') || 100, 500);

    const filters = [
        pseudo ? eq(songsurfEvents.pseudo, pseudo) : undefined,
        type ? eq(songsurfEvents.eventType, type) : undefined,
    ].filter(Boolean);

    const events = await db.select().from(songsurfEvents)
        .where(filters.length ? and(...filters) : undefined)
        .orderBy(desc(songsurfEvents.receivedAt), desc(songsurfEvents.id))
        .limit(limit);

    const logins = await db.select().from(songsurfEvents)
        .where(inArray(songsurfEvents.eventType, ['login_success', 'login_rejected']))
        .orderBy(desc(songsurfEvents.receivedAt), desc(songsurfEvents.id))
        .limit(50);

    const pseudos = (await db.selectDistinct({ pseudo: songsurfEvents.pseudo })
        .from(songsurfEvents))
        .map(r => r.pseudo)
        .filter(p => p !== '')
        .sort();

    return { events, logins, pseudos, filterPseudo: pseudo, filterType: type, limit, eventTypes: EVENT_TYPES };
};
