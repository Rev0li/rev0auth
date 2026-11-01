import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types.js';
import { desc, isNull } from 'drizzle-orm';
import { db } from '$lib/server/db/index.js';
import { invites } from '$lib/server/db/schema.js';
import { listUsers, adminView } from '$lib/server/users.js';
import { portalUrl } from '$lib/server/console-host.js';

export const load: PageServerLoad = async ({ locals }) => {
    if (!locals.adminSession) throw redirect(302, '/japprends/login');

    // Invitations pas encore utilisées (une invitation utilisée est devenue un membre)
    const open = await db.select({
        id: invites.id, code: invites.code, note: invites.note, expiresAt: invites.expiresAt,
    }).from(invites).where(isNull(invites.usedBy)).orderBy(desc(invites.createdAt));

    return {
        users:   (await listUsers()).map(adminView),
        invites: open,
        // Liens d'invitation et avatars sur le portail public, pas sur le nom de la console
        signupUrl:  portalUrl('/signup'),
        avatarBase: portalUrl('/members/avatar/'),
    };
};
