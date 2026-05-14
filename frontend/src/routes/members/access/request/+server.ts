import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { db } from '$lib/server/db/index.js';
import { users } from '$lib/server/db/schema.js';
import { eq } from 'drizzle-orm';

const SERVICE_FIELDS: Record<string, 'requestSongsurf' | 'requestJellyfin'> = {
    songsurf: 'requestSongsurf',
    jellyfin: 'requestJellyfin',
};

export const POST: RequestHandler = async ({ request, locals }) => {
    if (!locals.memberSession) throw error(401, 'Non autorisé.');
    const { service } = await request.json();
    const field = SERVICE_FIELDS[service];
    if (!field) return json({ ok: false, message: 'Service inconnu.' }, { status: 400 });

    await db.update(users).set({ [field]: true }).where(eq(users.pseudo, locals.memberSession.pseudo));
    return json({ ok: true });
};
