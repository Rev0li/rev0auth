import { redirect } from '@sveltejs/kit';
import { downloadHistory } from '$lib/server/songsurf-history.js';
import { songsurfConfigured } from '$lib/server/songsurf.js';
import { buildTasteProfiles } from '$lib/songsurf-taste.js';
import type { PageServerLoad } from './$types.js';

// Activité › SongSurf : fiches de goûts par membre (artistes, albums), sur
// l'historique stocké dans la base du portail (lisible NAS éteint).
// Connexions et évènements techniques : Activité › Journal.
export const load: PageServerLoad = async ({ locals }) => {
    if (!locals.adminSession) throw redirect(303, '/japprends/login');

    const { rows, status } = await downloadHistory();
    return { ...buildTasteProfiles(rows), status, canImport: songsurfConfigured() };
};
