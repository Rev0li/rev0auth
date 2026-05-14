import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types.js';
import { db } from '$lib/server/db/index.js';
import { users, signupRequests, messages, donations, testRuns, auditLog } from '$lib/server/db/schema.js';
import { eq, desc, count, and } from 'drizzle-orm';

export const load: PageServerLoad = async ({ locals }) => {
    if (!locals.adminSession) throw redirect(302, '/');

    const [allUsers, pending, allMessages, allDonations, runs, audits, dbStatus] = await Promise.all([
        db.select().from(users).orderBy(users.createdAt),
        db.select().from(signupRequests).where(eq(signupRequests.status, 'pending')).orderBy(signupRequests.createdAt),
        db.select().from(messages).orderBy(desc(messages.createdAt)).limit(100),
        db.select().from(donations).orderBy(desc(donations.createdAt)),
        db.select().from(testRuns).orderBy(desc(testRuns.executedAt)).limit(10),
        db.select().from(auditLog).orderBy(desc(auditLog.timestampEpoch)).limit(50),
        Promise.resolve({ ok: true }),
    ]);

    return {
        users:    allUsers.map(({ passwordHash: _, avatarBytes: __, ...u }) => u),
        pending,
        messages: allMessages,
        donations: allDonations,
        testRuns: runs.map(r => ({ ...r, cases: JSON.parse(r.cases) as { name: string; ok: boolean }[] })),
        auditLog: audits,
        dbOk:     dbStatus.ok,
    };
};
