import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { songsurfConfigured, fetchSongsurfDlLogs } from '$lib/server/songsurf.js';

export const GET: RequestHandler = async ({ url, request, locals }) => {
    if (!locals.adminSession) throw error(401, 'Non autorisé.');

    if (!songsurfConfigured()) {
        return json({ success: false, error: 'SongSurf non configuré' }, { status: 503 });
    }

    const pseudo = url.searchParams.get('pseudo') ?? '';
    const limit  = Math.min(parseInt(url.searchParams.get('limit') ?? '100') || 100, 500);

    try {
        const resp = await fetchSongsurfDlLogs(request.headers.get('cookie'), pseudo, limit, 10_000);

        if (!resp.ok) {
            return json({ success: false, error: `SongSurf: ${resp.status}` }, { status: 502 });
        }
        const data = await resp.json().catch(() => null);
        if (data === null) {
            return json({ success: false, error: 'Réponse SongSurf invalide' }, { status: 502 });
        }
        return json(data);
    } catch {
        return json({ success: false, error: 'SongSurf injoignable' }, { status: 502 });
    }
};
