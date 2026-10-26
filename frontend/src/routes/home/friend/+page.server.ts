import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types.js';
import { findUserById, getAvatarFilename } from '$lib/server/users.js';
import { songsurfBaseUrl } from '$lib/server/songsurf.js';
import { consoleUrl } from '$lib/server/console-host.js';

// Espace membre sur une seule page (AUTH-20) : services + compte
export const load: PageServerLoad = async ({ locals }) => {
    if (!locals.memberSession) throw redirect(302, '/');
    const userId = locals.memberSession.userId;

    const [user, avatarFilename] = await Promise.all([findUserById(userId), getAvatarFilename(userId)]);
    if (!user) throw redirect(302, '/');

    return {
        user: { pseudo: user.name, role: user.role, avatarFilename },
        songsurfUrl: songsurfBaseUrl() || null,
        consoleUrl:  consoleUrl('/japprends/login'),
    };
};
