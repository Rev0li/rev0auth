import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { db } from '$lib/server/db/index.js';
import { invites } from '$lib/server/db/schema.js';
import { desc } from 'drizzle-orm';
import { generateToken } from '$lib/server/auth.js';
import { writeAudit } from '$lib/server/audit.js';

const INVITE_TTL_DAYS = 7;

function requireAdmin({ locals }: { locals: App.Locals }) {
    if (!locals.adminSession) throw error(401, 'Non autorisé.');
}

export const GET: RequestHandler = async ({ locals }) => {
    requireAdmin({ locals });
    const rows = await db.select().from(invites).orderBy(desc(invites.createdAt));
    return json(rows);
};

// Seule façon d'ajouter quelqu'un (AUTH-12) : un lien d'invitation, 7 jours
export const POST: RequestHandler = async ({ request, locals }) => {
    requireAdmin({ locals });
    const body = await request.json().catch(() => ({}));
    const note = typeof body?.note === 'string' ? body.note.trim().slice(0, 80) : '';
    const code = generateToken(12);
    const now = Math.floor(Date.now() / 1000);
    const expiresAt = now + INVITE_TTL_DAYS * 24 * 60 * 60;
    const [row] = await db.insert(invites).values({ code, note, createdAt: now, expiresAt })
        .returning({ id: invites.id });
    await writeAudit('create_invite', locals.adminSession!.pseudo, note, `invite #${row.id}, ${INVITE_TTL_DAYS} days`);
    return json({ ok: true, id: row.id, code });
};
