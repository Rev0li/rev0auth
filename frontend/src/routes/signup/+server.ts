import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { and, eq, gt, isNull } from 'drizzle-orm';
import { db } from '$lib/server/db/index.js';
import { invites } from '$lib/server/db/schema.js';
import { hashPassword } from '$lib/server/auth.js';
import { createUser, findUserByPseudo, setAvatar, PSEUDO_RE } from '$lib/server/users.js';
import { fetchAvatarSvg, SEED_RE } from '$lib/server/dicebear.js';

// Code Postgres 23505 (unique_violation), éventuellement enveloppé par Drizzle
function isUniqueViolation(e: unknown): boolean {
    const err = e as { code?: string; cause?: { code?: string } } | null;
    return err?.code === '23505' || err?.cause?.code === '23505';
}

export const POST: RequestHandler = async ({ request }) => {
    const { pseudo, password, invite_code, avatar_seed } = await request.json();

    const cleanPseudo = (pseudo ?? '').trim();
    const cleanCode   = (invite_code ?? '').trim();

    if (!PSEUDO_RE.test(cleanPseudo)) {
        return json({ ok: false, message: 'Pseudo invalide (3-20 caractères, lettres, chiffres, _ ou -).' }, { status: 400 });
    }
    if (typeof password !== 'string' || password.length < 8) {
        return json({ ok: false, message: 'Mot de passe trop court (8 caractères minimum).' }, { status: 400 });
    }

    // Pseudo stocké en minuscules (connexion insensible à la casse)
    const storedPseudo = cleanPseudo.toLowerCase();
    const passwordHash = await hashPassword(password);
    // Appel réseau DiceBear hors transaction ; un échec n'est pas bloquant
    // (l'avatar par défaut, seedé par le pseudo, prend le relais)
    const svg = typeof avatar_seed === 'string' && SEED_RE.test(avatar_seed) ? await fetchAvatarSvg(avatar_seed) : null;
    const now = Math.floor(Date.now() / 1000);

    // Pseudo libre, puis invitation consommée de façon atomique, puis compte :
    // tout ou rien, un lien ne sert qu'une fois.
    const outcome = await db.transaction(async (tx) => {
        if (await findUserByPseudo(storedPseudo, tx)) return 'taken';

        const consumed = await tx
            .update(invites)
            .set({ usedBy: storedPseudo, usedAt: now })
            .where(and(eq(invites.code, cleanCode), isNull(invites.usedBy), gt(invites.expiresAt, now)))
            .returning({ id: invites.id });
        if (consumed.length === 0) return 'invite';

        const user = await createUser({ pseudo: storedPseudo, passwordHash, approved: true }, tx);
        if (svg) await setAvatar(user.id, { svg, filename: `${avatar_seed}.svg` }, tx);
        return 'created';
    }).catch((e) => {
        if (isUniqueViolation(e)) return 'taken'; // course sur le même pseudo
        throw e;
    });

    if (outcome === 'invite') {
        return json({ ok: false, message: "Lien d'invitation invalide ou expiré." }, { status: 403 });
    }
    if (outcome === 'taken') {
        return json({ ok: false, message: 'Ce pseudo est déjà utilisé.' }, { status: 409 });
    }
    return json({ ok: true, message: 'Compte créé avec succès.' });
};
