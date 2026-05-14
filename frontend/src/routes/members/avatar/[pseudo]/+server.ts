import { error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { db } from '$lib/server/db/index.js';
import { users } from '$lib/server/db/schema.js';
import { eq } from 'drizzle-orm';

export const GET: RequestHandler = async ({ params, locals }) => {
    if (!locals.memberSession && !locals.adminSession) throw error(401, 'Non autorisé.');

    const rows = await db
        .select({ avatarBytes: users.avatarBytes, avatarMime: users.avatarMime })
        .from(users)
        .where(eq(users.pseudo, params.pseudo.toLowerCase()))
        .limit(1);

    const user = rows[0];
    if (!user?.avatarBytes) throw error(404, 'Pas d\'avatar.');

    // better-sqlite3 returns blobs as Buffer (subclass of Uint8Array)
    const bytes = user.avatarBytes as unknown as Buffer;

    return new Response(bytes.buffer.slice(bytes.byteOffset, bytes.byteOffset + bytes.byteLength) as ArrayBuffer, {
        headers: { 'Content-Type': user.avatarMime ?? 'image/jpeg' },
    });
};
