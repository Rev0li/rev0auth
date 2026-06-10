import { redirect } from '@sveltejs/kit';
import { db } from '$lib/server/db/index.js';
import { auditLog } from '$lib/server/db/schema.js';
import { desc } from 'drizzle-orm';
import type { PageServerLoad } from './$types.js';

export const load: PageServerLoad = async ({ locals }) => {
    if (!locals.adminSession) throw redirect(303, '/japprends/login');
    const entries = await db.select().from(auditLog).orderBy(desc(auditLog.createdAt)).limit(200);
    return { entries };
};
