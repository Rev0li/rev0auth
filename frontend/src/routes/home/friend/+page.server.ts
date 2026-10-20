import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types.js';
import { findUserById, listUsers } from '$lib/server/users.js';
import { songsurfBaseUrl } from '$lib/server/songsurf.js';

export const load: PageServerLoad = async ({ locals }) => {
    if (!locals.memberSession) throw redirect(302, '/');

    const [user, all] = await Promise.all([findUserById(locals.memberSession.userId), listUsers()]);
    if (!user) throw redirect(302, '/');

    return {
        user: {
            pseudo:          user.name,
            role:            user.role,
            accessJellyfin:  user.accessJellyfin,
            accessSongsurf:  user.accessSongsurf,
            requestJellyfin: user.requestJellyfin,
            requestSongsurf: user.requestSongsurf,
        },
        songsurfUrl: songsurfBaseUrl() || null,
        members: all.filter((u) => u.active).map((u) => ({ pseudo: u.name, role: u.role, bio: u.bio })),
    };
};
