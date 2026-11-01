import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { importSongsurfLog } from '$lib/server/songsurf-history.js';
import { writeAudit } from '$lib/server/audit.js';

// Reprise de l'ancien historique SongSurf dans la base du portail (admin).
// Sans doublon : relançable à volonté.
export const POST: RequestHandler = async ({ locals, request }) => {
    if (!locals.adminSession) return json({ ok: false, message: 'Non autorisé.' }, { status: 401 });

    const result = await importSongsurfLog(request.headers.get('cookie'));
    if (!result.ok) return json(result, { status: 502 });

    await writeAudit('songsurf.history_import', locals.adminSession.pseudo, '',
        `${result.imported} importés sur ${result.total} lignes`);
    return json(result);
};
