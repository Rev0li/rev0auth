import { redirect, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { songsurfConfigured, songsurfBaseUrl, signSongsurfJwt } from '$lib/server/songsurf.js';

export const GET: RequestHandler = async ({ locals }) => {
    if (!locals.adminSession) throw error(401, 'Non autorisé');

    if (!songsurfConfigured()) throw error(503, 'SongSurf non configuré');

    const token = await signSongsurfJwt('rev0admin', 'admin', 8 * 3600);

    throw redirect(302, `${songsurfBaseUrl()}?token=${token}`);
};
