import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { db } from '$lib/server/db/index.js';
import { messages } from '$lib/server/db/schema.js';

export const POST: RequestHandler = async ({ request, locals }) => {
    if (!locals.adminSession) throw error(401, 'Non autorisé.');
    const { to, body } = await request.json();
    if (!to || !body?.trim()) return json({ ok: false }, { status: 400 });

    const adminPseudo = process.env.ADMIN_DASH_PSEUDO ?? 'admin';
    await db.insert(messages).values({ fromPseudo: adminPseudo, toPseudo: to.toLowerCase(), body });
    return json({ ok: true });
};
