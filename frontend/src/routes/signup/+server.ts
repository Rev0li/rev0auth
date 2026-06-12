import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { db } from '$lib/server/db/index.js';
import { users, invites } from '$lib/server/db/schema.js';
import { eq, sql } from 'drizzle-orm';
import { hashPassword } from '$lib/server/auth.js';
import { setBaPassword } from '$lib/server/ba-sync.js';
import { AVATAR_SVG, isAvatarId } from '$lib/avatars.js';

const PSEUDO_RE = /^[a-zA-Z0-9_-]{3,20}$/;


export const POST: RequestHandler = async ({ request }) => {
    const { pseudo, password, invite_code, avatar_id } = await request.json();

    const cleanPseudo = (pseudo ?? '').trim();
    const cleanCode   = (invite_code ?? '').trim();

    if (!PSEUDO_RE.test(cleanPseudo)) {
        return json({ ok: false, message: 'Pseudo invalide (3-20 caractères, lettres, chiffres, _ ou -).' }, { status: 400 });
    }
    if (!password || password.length < 8) {
        return json({ ok: false, message: 'Mot de passe trop court (8 caractères minimum).' }, { status: 400 });
    }

    const now = Math.floor(Date.now() / 1000);

    const inviteRows = await db.select().from(invites).where(eq(invites.code, cleanCode)).limit(1);
    const invite = inviteRows[0];
    if (!invite || invite.usedBy !== null || invite.expiresAt <= now) {
        return json({ ok: false, message: "Lien d'invitation invalide ou expiré." }, { status: 403 });
    }

    const existing = await db
        .select({ pseudo: users.pseudo })
        .from(users)
        .where(sql`LOWER(${users.pseudo}) = LOWER(${cleanPseudo})`)
        .limit(1);
    if (existing.length > 0) {
        return json({ ok: false, message: 'Ce pseudo est déjà utilisé.' }, { status: 409 });
    }

    // Pseudo stocké en lowercase : les lookups (portal/login, avatars…) comparent en lowercase
    const storedPseudo = cleanPseudo.toLowerCase();
    const passwordHash = await hashPassword(password);
    await db.insert(users).values({ pseudo: storedPseudo, passwordHash, approved: true, status: 'actif' });
    await setBaPassword(storedPseudo, passwordHash);

    if (isAvatarId(avatar_id)) {
        const svg = AVATAR_SVG[avatar_id];
        await db
            .update(users)
            .set({
                avatarBytes:    Buffer.from(svg),
                avatarMime:     'image/svg+xml',
                avatarFilename: `${avatar_id}.svg`,
                avatarSizeBytes: svg.length,
            })
            .where(sql`LOWER(${users.pseudo}) = LOWER(${cleanPseudo})`);
    }

    await db
        .update(invites)
        .set({ usedBy: storedPseudo, usedAt: now })
        .where(eq(invites.id, invite.id));

    return json({ ok: true, message: 'Compte créé avec succès.' });
};
