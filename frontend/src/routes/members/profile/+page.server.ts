import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types.js';
import { findUserById, getAvatarFilename } from '$lib/server/users.js';

export const load: PageServerLoad = async ({ locals }) => {
    if (!locals.memberSession) throw redirect(302, '/');
    const userId = locals.memberSession.userId;

    const [user, avatarFilename] = await Promise.all([findUserById(userId), getAvatarFilename(userId)]);
    if (!user) throw redirect(302, '/');

    return { user: { pseudo: user.name, bio: user.bio, avatarFilename } };
};
