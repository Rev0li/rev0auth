import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { updateUser } from '$lib/server/users.js';

export const PUT: RequestHandler = async ({ request, locals }) => {
    if (!locals.memberSession) throw error(401, 'Non autorisé.');

    // Seule la bio est éditable ici. GitHub/LinkedIn : soumis via la demande
    // d'accès aux services (/members/access/request).
    const { bio } = await request.json();
    if (typeof bio !== 'string') return json({ ok: false, message: 'Bio requise.' }, { status: 400 });

    await updateUser(locals.memberSession.userId, { bio: bio.trim() });
    return json({ ok: true });
};
