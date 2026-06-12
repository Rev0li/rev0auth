import { error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { db } from '$lib/server/db/index.js';
import { users } from '$lib/server/db/schema.js';
import { sql } from 'drizzle-orm';
import { fetchAvatarSvg } from '$lib/server/dicebear.js';

export const GET: RequestHandler = async ({ params, locals }) => {
    if (!locals.memberSession && !locals.adminSession) throw error(401, 'Non autorisé.');

    const rows = await db
        .select({ avatarBytes: users.avatarBytes, avatarMime: users.avatarMime })
        .from(users)
        .where(sql`LOWER(${users.pseudo}) = LOWER(${params.pseudo})`)
        .limit(1);

    const user = rows[0];
    if (!user?.avatarBytes) {
        // Pas d'avatar choisi : initial-face par défaut, seedé par le pseudo
        const fallback = await fetchAvatarSvg(params.pseudo!.toLowerCase());
        if (!fallback) throw error(404, 'Pas d\'avatar.');
        return new Response(fallback, {
            headers: {
                'Content-Type': 'image/svg+xml',
                'Content-Security-Policy': "sandbox; default-src 'none'; style-src 'unsafe-inline'",
            },
        });
    }

    const bytes = user.avatarBytes;

    return new Response(new Uint8Array(bytes), {
        headers: {
            'Content-Type': user.avatarMime ?? 'image/jpeg',
            // SVG = XML actif : sandbox pour neutraliser tout script embarqué
            'Content-Security-Policy': "sandbox; default-src 'none'; style-src 'unsafe-inline'",
        },
    });
};
