import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { db } from '$lib/server/db/index.js';
import { testRuns } from '$lib/server/db/schema.js';
import { desc } from 'drizzle-orm';

export const GET: RequestHandler = async ({ locals }) => {
    if (!locals.adminSession) throw error(401, 'Non autorisé.');
    const rows = await db.select().from(testRuns).orderBy(desc(testRuns.executedAt)).limit(20);
    return json(rows.map(r => ({ ...r, cases: JSON.parse(r.cases) })));
};
