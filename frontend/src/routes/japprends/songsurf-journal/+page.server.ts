import { redirect } from '@sveltejs/kit';
import { and, desc, inArray, ne } from 'drizzle-orm';
import { db } from '$lib/server/db/index.js';
import { songsurfEvents } from '$lib/server/db/schema.js';
import type { PageServerLoad } from './$types.js';

// Activité › Journal : connexions à SongSurf et évènements techniques
// (échecs, exports ZIP, démarrage/arrêt du conteneur) poussés par le NAS.
// Les téléchargements réussis sont sur Activité › SongSurf.
const LOGIN_TYPES = ['login_success', 'login_rejected'];

export const load: PageServerLoad = async ({ locals }) => {
    if (!locals.adminSession) throw redirect(303, '/japprends/login');

    const [logins, others] = await Promise.all([
        db.select().from(songsurfEvents)
            .where(inArray(songsurfEvents.eventType, LOGIN_TYPES))
            .orderBy(desc(songsurfEvents.receivedAt), desc(songsurfEvents.id))
            .limit(50),
        db.select().from(songsurfEvents)
            .where(and(
                ne(songsurfEvents.eventType, 'download_success'),
                ...LOGIN_TYPES.map((t) => ne(songsurfEvents.eventType, t)),
            ))
            .orderBy(desc(songsurfEvents.receivedAt), desc(songsurfEvents.id))
            .limit(200),
    ]);
    return { logins, others };
};
