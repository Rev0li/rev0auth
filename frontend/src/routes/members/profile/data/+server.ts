import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { db } from '$lib/server/db/index.js';
import { users } from '$lib/server/db/schema.js';
import { eq } from 'drizzle-orm';

export const PUT: RequestHandler = async ({ request, locals }) => {
    if (!locals.memberSession) throw error(401, 'Non autorisé.');

    // Seule la bio est éditable ici. Commentaire : supprimé. GitHub/LinkedIn :
    // soumis via la demande d'accès aux services (/members/access/request).
    const { bio } = await request.json();
    if (typeof bio !== 'string') return json({ ok: false, message: 'Bio requise.' }, { status: 400 });

    await db
        .update(users)
        .set({ bio: bio.trim() })
        .where(eq(users.pseudo, locals.memberSession.pseudo));

    return json({ ok: true });
};
