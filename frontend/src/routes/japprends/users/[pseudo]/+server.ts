import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { db } from '$lib/server/db/index.js';
import { users, sessions, auditLog } from '$lib/server/db/schema.js';
import { eq } from 'drizzle-orm';

function requireAdmin({ locals }: { locals: App.Locals }) {
    if (!locals.adminSession) throw error(401, 'Non autorisé.');
}

export const PUT: RequestHandler = async ({ request, locals, params }) => {
    requireAdmin({ locals });
    const key = params.pseudo.toLowerCase();
    const updates = await request.json();

    // Never allow overwriting passwordHash via this route
    delete updates.passwordHash;
    delete updates.pseudo;

    await db.update(users).set(updates).where(eq(users.pseudo, key));

    await db.insert(auditLog).values({
        timestampEpoch: Date.now(),
        action: 'update_user',
        target: key,
        detail: JSON.stringify(Object.keys(updates)),
    });

    return json({ ok: true });
};

export const DELETE: RequestHandler = async ({ locals, params }) => {
    requireAdmin({ locals });
    const key = params.pseudo.toLowerCase();

    await db.delete(sessions).where(eq(sessions.pseudo, key));
    await db.delete(users).where(eq(users.pseudo, key));

    await db.insert(auditLog).values({
        timestampEpoch: Date.now(),
        action: 'delete_user',
        target: key,
    });

    return json({ ok: true });
};
