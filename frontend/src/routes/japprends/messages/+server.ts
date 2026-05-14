import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { db } from '$lib/server/db/index.js';
import { messages } from '$lib/server/db/schema.js';
import { desc } from 'drizzle-orm';

export const GET: RequestHandler = async ({ locals }) => {
    if (!locals.adminSession) throw error(401, 'Non autorisé.');
    const rows = await db.select().from(messages).orderBy(desc(messages.createdAt));
    return json(rows);
};
