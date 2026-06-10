import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { db } from '$lib/server/db/index.js';
import { auditLog } from '$lib/server/db/schema.js';
import { desc } from 'drizzle-orm';

export const GET: RequestHandler = async ({ url, locals }) => {
    if (!locals.adminSession) throw error(401, 'Non autorisé.');
    const limit = Math.min(parseInt(url.searchParams.get('limit') ?? '200') || 200, 1000);
    const rows = await db.select().from(auditLog).orderBy(desc(auditLog.createdAt)).limit(limit);
    return json(rows);
};
