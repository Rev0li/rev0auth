import { redirect, error } from '@sveltejs/kit';
import { SignJWT } from 'jose';
import type { RequestHandler } from './$types.js';

export const GET: RequestHandler = async ({ locals }) => {
    if (!locals.adminSession) throw error(401, 'Non autorisé');

    const jwtSecret = process.env.AUTH_JWT_SECRET ?? '';
    const songsurfUrl = (process.env.SONGSURF_URL ?? '').replace(/\/$/, '');

    if (!jwtSecret || !songsurfUrl) throw error(503, 'SongSurf non configuré');

    const now = Math.floor(Date.now() / 1000);
    const token = await new SignJWT({
        sub:        'rev0admin',
        role:       'admin',
        email:      '',
        token_type: 'access',
        iat:        now,
    })
        .setProtectedHeader({ alg: 'HS256' })
        .setExpirationTime(now + 8 * 3600)
        .sign(new TextEncoder().encode(jwtSecret));

    throw redirect(302, `${songsurfUrl}?token=${token}`);
};
