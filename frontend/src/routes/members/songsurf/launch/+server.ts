import { redirect, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { findUserById } from '$lib/server/users.js';
import { songsurfConfigured, songsurfBaseUrl, signSongsurfJwt } from '$lib/server/songsurf.js';

// Lancement SongSurf depuis la zone membre. On re-signe un JWT frais (8h) à
// chaque clic plutôt que de dépendre du sessionStorage rempli au login : le
// lien reste valide après refresh, lien direct ou expiration du token initial.
export const GET: RequestHandler = async ({ locals }) => {
    if (!locals.memberSession) throw redirect(302, '/');
    if (!songsurfConfigured()) throw error(503, 'SongSurf indisponible.');

    const user = await findUserById(locals.memberSession.userId);
    if (!user || !user.active || !user.accessSongsurf) throw error(403, 'Accès SongSurf non accordé.');

    const token = await signSongsurfJwt(user.name, user.role, 8 * 3600);
    throw redirect(303, `${songsurfBaseUrl()}?token=${token}`);
};
