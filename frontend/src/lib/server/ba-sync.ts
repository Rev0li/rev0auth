import { randomUUID } from 'node:crypto';
import { and, eq } from 'drizzle-orm';
import { db } from './db/index.js';
import { ba_users, ba_accounts } from './db/auth-schema.js';

// Synchronisation web_users → ba_* pendant la coexistence des deux systèmes.
// web_users reste la source de vérité métier (approved, active, access_*) ;
// ba_* porte l'identité BetterAuth (sessions, credential). Jointure :
// ba_users.username = LOWER(web_users.pseudo), ba_users.name = pseudo exact.
// Mêmes conventions que scripts/migrate-web-users-to-ba.mjs.

export async function findBaUser(pseudo: string) {
    const rows = await db
        .select()
        .from(ba_users)
        .where(eq(ba_users.username, pseudo.toLowerCase()))
        .limit(1);
    return rows[0] ?? null;
}

export async function ensureBaUser(pseudo: string, role = 'member'): Promise<string> {
    const existing = await findBaUser(pseudo);
    if (existing) return existing.id;

    const username = pseudo.toLowerCase();
    const id = randomUUID();
    await db.insert(ba_users).values({
        id,
        name: pseudo,
        email: `${username}@local.invalid`,
        username,
        displayUsername: pseudo,
        role,
        updatedAt: new Date(),
    });
    return id;
}

export async function setBaPassword(pseudo: string, argonHash: string, role = 'member'): Promise<void> {
    const userId = await ensureBaUser(pseudo, role);
    const updated = await db
        .update(ba_accounts)
        .set({ password: argonHash, updatedAt: new Date() })
        .where(and(eq(ba_accounts.userId, userId), eq(ba_accounts.providerId, 'credential')))
        .returning({ id: ba_accounts.id });
    if (updated.length === 0) {
        await db.insert(ba_accounts).values({
            id: randomUUID(),
            accountId: userId,
            providerId: 'credential',
            userId,
            password: argonHash,
            updatedAt: new Date(),
        });
    }
}

// Pas de credential = connexion impossible (même sémantique que le script de migration)
export async function removeBaPassword(pseudo: string): Promise<void> {
    const user = await findBaUser(pseudo);
    if (!user) return;
    await db
        .delete(ba_accounts)
        .where(and(eq(ba_accounts.userId, user.id), eq(ba_accounts.providerId, 'credential')));
}

// Les FK ba_sessions/ba_accounts sont ON DELETE CASCADE
export async function deleteBaUser(pseudo: string): Promise<void> {
    const user = await findBaUser(pseudo);
    if (!user) return;
    await db.delete(ba_users).where(eq(ba_users.id, user.id));
}

export async function syncBaRole(pseudo: string, role: string): Promise<void> {
    await db
        .update(ba_users)
        .set({ role, updatedAt: new Date() })
        .where(eq(ba_users.username, pseudo.toLowerCase()));
}
