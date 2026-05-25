import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { db } from '$lib/server/db/index.js';
import { sql } from 'drizzle-orm';

export const GET: RequestHandler = async () => {
    try {
        await db.execute(sql`SELECT 1`);
        return json({ ok: true, db: 'ok', ts: Date.now() });
    } catch {
        return json({ ok: false, db: 'down', ts: Date.now() }, { status: 503 });
    }
};
