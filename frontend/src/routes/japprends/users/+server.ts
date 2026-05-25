import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { db } from '$lib/server/db/index.js';
import { users } from '$lib/server/db/schema.js';
import { eq, asc } from 'drizzle-orm';
import { hashPassword } from '$lib/server/auth.js';

function requireAdmin({ locals }: { locals: App.Locals }) {
    if (!locals.adminSession) throw error(401, 'Non autorisé.');
}

export const GET: RequestHandler = async ({ locals }) => {
    requireAdmin({ locals });
    const all = await db.select().from(users).orderBy(asc(users.createdAt));
    return json(all.map(u => ({ ...u, passwordHash: undefined, avatarBytes: undefined })));
};

export const POST: RequestHandler = async ({ request, locals }) => {
    requireAdmin({ locals });
    const { pseudo, role, password } = await request.json();
    const key = pseudo?.trim()?.toLowerCase();
    if (!key || !password) return json({ ok: false, message: 'Champs requis.' }, { status: 400 });

    const hash = await hashPassword(password);
    const now = Math.floor(Date.now() / 1000);
    await db.insert(users).values({
        pseudo:    key,
        role:      role ?? 'member',
        active:    true,
        passwordHash: hash,
        createdAt: now,
    });

    return json({ ok: true, pseudo: key });
};
