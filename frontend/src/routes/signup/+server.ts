import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { db } from '$lib/server/db/index.js';
import { users, invites } from '$lib/server/db/schema.js';
import { eq, sql } from 'drizzle-orm';
import { hashPassword } from '$lib/server/auth.js';

const PSEUDO_RE = /^[a-zA-Z0-9_-]{3,20}$/;

const AVATAR_SVG: Record<string, string> = {
    fox: `<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'><circle cx='50' cy='50' r='50' fill='#d4500a'/><polygon points='20,55 30,20 42,55' fill='#d4500a'/><polygon points='58,55 70,20 80,55' fill='#d4500a'/><polygon points='23,52 30,27 39,52' fill='#f9b084'/><polygon points='61,52 70,27 77,52' fill='#f9b084'/><circle cx='50' cy='60' r='22' fill='#f9b084'/><ellipse cx='43' cy='54' rx='4' ry='4.5' fill='#1a1a1a'/><ellipse cx='57' cy='54' rx='4' ry='4.5' fill='#1a1a1a'/><circle cx='44' cy='53' r='1.2' fill='white'/><circle cx='58' cy='53' r='1.2' fill='white'/><ellipse cx='50' cy='64' rx='3' ry='2' fill='#1a1a1a'/><ellipse cx='50' cy='68' rx='9' ry='5' fill='#fde4cc' opacity='0.7'/></svg>`,
    wolf: `<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'><circle cx='50' cy='50' r='50' fill='#4a5568'/><polygon points='18,52 28,15 40,52' fill='#4a5568'/><polygon points='60,52 72,15 82,52' fill='#4a5568'/><polygon points='21,50 28,22 37,50' fill='#9aa5b4'/><polygon points='63,50 72,22 79,50' fill='#9aa5b4'/><ellipse cx='50' cy='62' rx='24' ry='20' fill='#9aa5b4'/><ellipse cx='50' cy='71' rx='13' ry='9' fill='#bec5cf'/><ellipse cx='42' cy='54' rx='4.5' ry='4' fill='#1a1a1a'/><ellipse cx='58' cy='54' rx='4.5' ry='4' fill='#1a1a1a'/><circle cx='43' cy='53' r='1.3' fill='#e8f0fe'/><circle cx='59' cy='53' r='1.3' fill='#e8f0fe'/><ellipse cx='50' cy='65' rx='4' ry='2.5' fill='#2d3748'/></svg>`,
    cat: `<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'><circle cx='50' cy='50' r='50' fill='#6b46c1'/><polygon points='22,52 32,18 44,52' fill='#6b46c1'/><polygon points='56,52 68,18 78,52' fill='#6b46c1'/><polygon points='25,50 32,25 41,50' fill='#f9a8d4'/><polygon points='59,50 68,25 75,50' fill='#f9a8d4'/><circle cx='50' cy='60' r='22' fill='#9f7aea'/><ellipse cx='42' cy='54' rx='5' ry='4' fill='#1a1a1a'/><ellipse cx='58' cy='54' rx='5' ry='4' fill='#1a1a1a'/><ellipse cx='42' cy='54' rx='2' ry='3.5' fill='#52b788'/><ellipse cx='58' cy='54' rx='2' ry='3.5' fill='#52b788'/><circle cx='43' cy='53' r='1' fill='white'/><polygon points='50,62 47,65 53,65' fill='#f9a8d4'/><line x1='28' y1='64' x2='43' y2='67' stroke='white' stroke-width='0.8' opacity='0.7'/><line x1='28' y1='68' x2='43' y2='68' stroke='white' stroke-width='0.8' opacity='0.7'/><line x1='57' y1='67' x2='72' y2='64' stroke='white' stroke-width='0.8' opacity='0.7'/><line x1='57' y1='68' x2='72' y2='68' stroke='white' stroke-width='0.8' opacity='0.7'/></svg>`,
    eagle: `<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'><circle cx='50' cy='50' r='50' fill='#1a202c'/><circle cx='50' cy='56' r='24' fill='#744210'/><circle cx='50' cy='48' r='17' fill='#f7fafc'/><circle cx='44' cy='46' r='5' fill='#f6ad55'/><circle cx='44' cy='46' r='3' fill='#1a1a1a'/><circle cx='45' cy='45' r='1' fill='white'/><polygon points='35,52 50,48 37,60' fill='#f6ad55'/><ellipse cx='63' cy='62' rx='12' ry='8' fill='#2d3748'/><ellipse cx='37' cy='63' rx='10' ry='7' fill='#744210'/></svg>`,
    dragon: `<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'><circle cx='50' cy='50' r='50' fill='#065f46'/><polygon points='38,30 34,10 42,28' fill='#34d399'/><polygon points='62,30 66,10 58,28' fill='#34d399'/><circle cx='50' cy='58' r='24' fill='#059669'/><ellipse cx='50' cy='70' rx='12' ry='9' fill='#34d399'/><circle cx='46' cy='69' r='2' fill='#065f46'/><circle cx='54' cy='69' r='2' fill='#065f46'/><ellipse cx='41' cy='52' rx='5.5' ry='4' fill='#f59e0b'/><ellipse cx='59' cy='52' rx='5.5' ry='4' fill='#f59e0b'/><ellipse cx='41' cy='52' rx='1.5' ry='4' fill='#1a1a1a'/><ellipse cx='59' cy='52' rx='1.5' ry='4' fill='#1a1a1a'/><path d='M38,64 Q50,58 62,64' fill='none' stroke='#34d399' stroke-width='1.5' opacity='0.6'/></svg>`,
};

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

    const passwordHash = await hashPassword(password);
    await db.insert(users).values({ pseudo: cleanPseudo, passwordHash, approved: true, status: 'actif' });

    if (avatar_id && AVATAR_SVG[avatar_id]) {
        const svg = AVATAR_SVG[avatar_id];
        await db
            .update(users)
            .set({
                avatarBytes:    Buffer.from(svg),
                avatarMime:     'image/svg+xml',
                avatarFilename: 'avatar.svg',
                avatarSizeBytes: svg.length,
            })
            .where(sql`LOWER(${users.pseudo}) = LOWER(${cleanPseudo})`);
    }

    await db
        .update(invites)
        .set({ usedBy: cleanPseudo, usedAt: now })
        .where(eq(invites.id, invite.id));

    return json({ ok: true, message: 'Compte créé avec succès.' });
};
