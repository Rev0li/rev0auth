import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types.js';
import { db } from '$lib/server/db/index.js';
import { users, invites, messages, donations, testRuns } from '$lib/server/db/schema.js';
import { isNull, asc, desc } from 'drizzle-orm';

export const load: PageServerLoad = async ({ locals }) => {
    if (!locals.adminSession) throw redirect(302, '/');

    const now = Math.floor(Date.now() / 1000);

    const [allUsers, unusedInvites, allMessages, allDonations, runs] = await Promise.all([
        db.select().from(users).orderBy(asc(users.createdAt)),
        db.select().from(invites).where(isNull(invites.usedBy)).orderBy(asc(invites.createdAt)),
        db.select().from(messages).orderBy(desc(messages.createdAt)).limit(100),
        db.select().from(donations).orderBy(desc(donations.createdAt)),
        db.select().from(testRuns).orderBy(desc(testRuns.executedAt)).limit(10),
    ]);

    return {
        users:     allUsers.map(({ passwordHash: _, avatarBytes: __, ...u }) => u),
        pending:   unusedInvites,
        messages:  allMessages,
        donations: allDonations,
        testRuns:  runs.map(r => ({ ...r, cases: JSON.parse(r.cases) as { name: string; ok: boolean }[] })),
        auditLog:  [] as unknown[],
        dbOk:      true,
    };
};
