import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { db } from '$lib/server/db/index.js';
import { messages, users } from '$lib/server/db/schema.js';
import { eq, or, and, asc, sql } from 'drizzle-orm';

// Messagerie membre, organisée en conversations (popup sur /home/friend) :
//   GET                → { threads: [{ peer, lastBody, lastAt, lastFromMe, unread }] }
//   GET ?with=<peer>   → { messages: [...] } (chronologique)
//   POST { to, body }  → envoi (destinataire = admin ou membre actif)
//   PATCH { with }     → marque lus tous les messages reçus de <peer>

const adminPseudo = () => process.env.ADMIN_DASH_PSEUDO ?? 'admin';

export const GET: RequestHandler = async ({ url, locals }) => {
    if (!locals.memberSession) throw error(401, 'Non autorisé.');
    const me     = locals.memberSession.pseudo;
    const meLow  = me.toLowerCase();
    const withPeer = url.searchParams.get('with')?.trim().toLowerCase();

    if (withPeer) {
        const rows = await db.select().from(messages)
            .where(or(
                and(sql`LOWER(${messages.fromPseudo}) = ${meLow}`,    sql`LOWER(${messages.toPseudo}) = ${withPeer}`),
                and(sql`LOWER(${messages.fromPseudo}) = ${withPeer}`, sql`LOWER(${messages.toPseudo}) = ${meLow}`),
            ))
            .orderBy(asc(messages.createdAt));
        return json({ messages: rows });
    }

    // Toutes mes conversations, groupées par interlocuteur
    const rows = await db.select().from(messages)
        .where(or(
            sql`LOWER(${messages.fromPseudo}) = ${meLow}`,
            sql`LOWER(${messages.toPseudo}) = ${meLow}`,
        ))
        .orderBy(asc(messages.createdAt));

    const threads = new Map<string, { peer: string; lastBody: string; lastAt: number; lastFromMe: boolean; unread: number }>();
    for (const m of rows) {
        const fromMe = m.fromPseudo.toLowerCase() === meLow;
        const peerRaw = fromMe ? m.toPseudo : m.fromPseudo;
        const key = peerRaw.toLowerCase();
        const t = threads.get(key) ?? { peer: peerRaw, lastBody: '', lastAt: 0, lastFromMe: false, unread: 0 };
        t.peer       = peerRaw;
        t.lastBody   = m.body;
        t.lastAt     = m.createdAt;
        t.lastFromMe = fromMe;
        if (!fromMe && !m.isRead) t.unread++;
        threads.set(key, t);
    }

    return json({
        threads: [...threads.values()].sort((a, b) => b.lastAt - a.lastAt),
    });
};

export const POST: RequestHandler = async ({ request, locals }) => {
    if (!locals.memberSession) throw error(401, 'Non autorisé.');
    const { to, body } = await request.json();
    const dest = to?.trim()?.toLowerCase();
    if (!dest || !body?.trim()) return json({ ok: false, message: 'Destinataire et message requis.' }, { status: 400 });
    if (dest === locals.memberSession.pseudo.toLowerCase()) {
        return json({ ok: false, message: 'Tu ne peux pas t\'écrire à toi-même.' }, { status: 400 });
    }

    // Destinataire valide : l'admin, ou un membre actif
    if (dest !== adminPseudo().toLowerCase()) {
        const exists = await db.select({ pseudo: users.pseudo }).from(users)
            .where(and(sql`LOWER(${users.pseudo}) = ${dest}`, eq(users.active, true)))
            .limit(1);
        if (exists.length === 0) return json({ ok: false, message: 'Destinataire introuvable.' }, { status: 404 });
    }

    await db.insert(messages).values({
        fromPseudo: locals.memberSession.pseudo,
        toPseudo:   dest,
        body:       body.trim(),
    });
    return json({ ok: true });
};

export const PATCH: RequestHandler = async ({ request, locals }) => {
    if (!locals.memberSession) throw error(401, 'Non autorisé.');
    const meLow = locals.memberSession.pseudo.toLowerCase();
    const { with: peer } = await request.json();
    const peerLow = peer?.trim()?.toLowerCase();
    if (!peerLow) return json({ ok: false }, { status: 400 });

    await db.update(messages)
        .set({ isRead: true })
        .where(and(
            sql`LOWER(${messages.fromPseudo}) = ${peerLow}`,
            sql`LOWER(${messages.toPseudo}) = ${meLow}`,
            eq(messages.isRead, false),
        ));
    return json({ ok: true });
};
