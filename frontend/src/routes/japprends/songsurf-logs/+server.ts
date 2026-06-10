import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { songsurfConfigured, songsurfBaseUrl, signSongsurfJwt } from '$lib/server/songsurf.js';

export const GET: RequestHandler = async ({ url, locals }) => {
    if (!locals.adminSession) throw error(401, 'Non autorisé.');

    if (!songsurfConfigured()) {
        return json({ success: false, error: 'SongSurf non configuré' }, { status: 503 });
    }

    const token = await signSongsurfJwt('admin', 'admin', 120);

    const pseudo = url.searchParams.get('pseudo') ?? '';
    const limit  = Math.min(parseInt(url.searchParams.get('limit') ?? '100') || 100, 500);
    const target = `${songsurfBaseUrl()}/api/admin/dl-logs?pseudo=${encodeURIComponent(pseudo)}&limit=${limit}`;

    try {
        const ctrl = new AbortController();
        const timer = setTimeout(() => ctrl.abort(), 10_000);
        const resp = await fetch(target, {
            headers: { Cookie: `access_token=${token}` },
            signal: ctrl.signal,
        });
        clearTimeout(timer);

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
