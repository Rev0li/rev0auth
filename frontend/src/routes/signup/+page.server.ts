import type { PageServerLoad } from './$types.js';
import { db } from '$lib/server/db/index.js';
import { invites } from '$lib/server/db/schema.js';
import { eq } from 'drizzle-orm';

export const load: PageServerLoad = async ({ url }) => {
    const code = url.searchParams.get('invite') ?? '';
    if (!code) return { inviteCode: '', invalid: true };

    const now = Math.floor(Date.now() / 1000);
    const rows = await db.select().from(invites).where(eq(invites.code, code)).limit(1);
    const invite = rows[0];

    if (!invite || invite.usedBy !== null || invite.expiresAt <= now) {
        return { inviteCode: code, invalid: true };
    }

    return { inviteCode: code, invalid: false };
};
