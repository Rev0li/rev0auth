import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { db } from '$lib/server/db/index.js';
import { users } from '$lib/server/db/schema.js';
import { eq } from 'drizzle-orm';

export const PUT: RequestHandler = async ({ request, locals }) => {
    if (!locals.memberSession) throw error(401, 'Non autorisé.');

    const { bio, commentary, githubUsername, linkedinName } = await request.json();

    await db
        .update(users)
        .set({
            bio:          typeof bio          === 'string' ? bio.trim()          : undefined,
            commentary:   typeof commentary   === 'string' ? commentary.trim()   : undefined,
            githubUsername: typeof githubUsername === 'string' ? githubUsername.trim() || null : undefined,
            linkedinName:   typeof linkedinName   === 'string' ? linkedinName.trim()   || null : undefined,
        })
        .where(eq(users.pseudo, locals.memberSession.pseudo));

    return json({ ok: true });
};
