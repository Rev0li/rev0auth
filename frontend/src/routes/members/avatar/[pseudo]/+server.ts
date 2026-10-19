import { error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { findUserByPseudo, getAvatar } from '$lib/server/users.js';
import { fetchAvatarSvg } from '$lib/server/dicebear.js';

export const GET: RequestHandler = async ({ params, locals }) => {
    if (!locals.memberSession && !locals.adminSession) throw error(401, 'Non autorisé.');

    const user = await findUserByPseudo(params.pseudo!);
    const avatar = user ? await getAvatar(user.id) : null;
    if (!avatar) {
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

    return new Response(new Uint8Array(avatar.bytes), {
        headers: {
            'Content-Type': avatar.mimeType,
            // SVG = XML actif : sandbox pour neutraliser tout script embarqué
            'Content-Security-Policy': "sandbox; default-src 'none'; style-src 'unsafe-inline'",
        },
    });
};
