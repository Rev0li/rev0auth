import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { SignJWT } from 'jose';

export const GET: RequestHandler = async ({ url, locals }) => {
    if (!locals.adminSession) throw error(401, 'Non autorisé.');

    const jwtSecret  = (process.env.SONGSURF_JWT_SECRET ?? '').trim();
    const songsurfBase = (process.env.SONGSURF_URL ?? '').trim();
    if (!jwtSecret || !songsurfBase) {
        return json({ success: false, error: 'SongSurf non configuré' }, { status: 503 });
    }

    const now = Math.floor(Date.now() / 1000);
    const token = await new SignJWT({
        sub: 'admin',
        role: 'admin',
        email: '',
        token_type: 'access',
    })
        .setProtectedHeader({ alg: 'HS256' })
        .setIssuedAt(now)
        .setExpirationTime(now + 120)
        .sign(new TextEncoder().encode(jwtSecret));

    const pseudo = url.searchParams.get('pseudo') ?? '';
    const limit  = Math.min(parseInt(url.searchParams.get('limit') ?? '100') || 100, 500);
    const target = `${songsurfBase.replace(/\/+$/, '')}/api/admin/dl-logs?pseudo=${encodeURIComponent(pseudo)}&limit=${limit}`;

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
