import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { db } from '$lib/server/db/index.js';
import { wallPosts } from '$lib/server/db/schema.js';
import { desc } from 'drizzle-orm';

function requireAdmin({ locals }: { locals: App.Locals }) {
    if (!locals.adminSession) throw error(401, 'Non autorisé.');
}

export const GET: RequestHandler = async ({ locals }) => {
    requireAdmin({ locals });
    const rows = await db.select().from(wallPosts).orderBy(desc(wallPosts.createdAt)).limit(50);
    return json(rows);
};

export const POST: RequestHandler = async ({ request, locals }) => {
    requireAdmin({ locals });
    const { body } = await request.json();
    if (!body?.trim()) return json({ ok: false }, { status: 400 });
    await db.insert(wallPosts).values({ pseudo: locals.adminSession!.pseudo, body: body.trim() });
    return json({ ok: true });
};
