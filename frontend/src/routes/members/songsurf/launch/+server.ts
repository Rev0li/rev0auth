import { redirect, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { db } from '$lib/server/db/index.js';
import { users } from '$lib/server/db/schema.js';
import { eq } from 'drizzle-orm';
import { songsurfConfigured, songsurfBaseUrl, signSongsurfJwt } from '$lib/server/songsurf.js';

// Lancement SongSurf depuis la zone membre. On re-signe un JWT frais (8h) à
// chaque clic plutôt que de dépendre du sessionStorage rempli au login : le
// lien reste valide après refresh, lien direct ou expiration du token initial.
export const GET: RequestHandler = async ({ locals }) => {
    if (!locals.memberSession) throw redirect(302, '/');
    if (!songsurfConfigured()) throw error(503, 'SongSurf indisponible.');

    const pseudo = locals.memberSession.pseudo;
    const rows = await db.select().from(users).where(eq(users.pseudo, pseudo)).limit(1);
    const user = rows[0];
    if (!user || !user.active || !user.accessSongsurf) throw error(403, 'Accès SongSurf non accordé.');

    const token = await signSongsurfJwt(user.pseudo, user.role, 8 * 3600);
    throw redirect(303, `${songsurfBaseUrl()}?token=${token}`);
};
