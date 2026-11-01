import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types.js';
import { findUserById, getAvatarFilename } from '$lib/server/users.js';
import { songsurfBaseUrl } from '$lib/server/songsurf.js';
import { consoleEntryUrl } from '$lib/server/console-host.js';

// Espace membre sur une seule page (AUTH-20) : services + compte
export const load: PageServerLoad = async ({ locals, request }) => {
    if (!locals.memberSession) throw redirect(302, '/');
    const userId = locals.memberSession.userId;

    const [user, avatarFilename] = await Promise.all([findUserById(userId), getAvatarFilename(userId)]);
    if (!user) throw redirect(302, '/');

    return {
        user: { pseudo: user.name, role: user.role, avatarFilename },
        songsurfUrl: songsurfBaseUrl() || null,
        // Relatif sur le nom de la console (AUTH-29) : même site, transition
        consoleUrl:  consoleEntryUrl(request.headers.get('host')),
    };
};
