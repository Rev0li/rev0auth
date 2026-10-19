import { randomUUID } from 'node:crypto';
import { and, asc, eq } from 'drizzle-orm';
import { db } from './db/index.js';
import { ba_users, ba_accounts, ba_sessions } from './db/auth-schema.js';
import { userAvatars } from './db/schema.js';

// Un seul modèle utilisateur : ba_users (identité BetterAuth + champs métier).
// Le pseudo y figure deux fois : username = LOWER(pseudo), clé de connexion
// unique ; name / displayUsername = pseudo affiché, casse d'origine.
// Mot de passe : credential BetterAuth dans ba_accounts (hash Argon2).

export type User = typeof ba_users.$inferSelect;
type UserPatch = Partial<Pick<User,
    'role' | 'active' | 'approved' | 'bio' | 'accessJellyfin' | 'accessSongsurf' |
    'requestJellyfin' | 'requestSongsurf' | 'githubUsername' | 'linkedinName'>>;
type Tx = Parameters<Parameters<typeof db.transaction>[0]>[0];

export const ROLES = ['guest', 'member', 'mod', 'admin'] as const;
export const PSEUDO_RE = /^[a-zA-Z0-9_-]{3,20}$/;

export function usernameOf(pseudo: string): string {
    return pseudo.trim().toLowerCase();
}

export async function findUserByPseudo(pseudo: string, tx: Tx | typeof db = db): Promise<User | null> {
    const rows = await tx.select().from(ba_users).where(eq(ba_users.username, usernameOf(pseudo))).limit(1);
    return rows[0] ?? null;
}

export async function findUserById(id: string): Promise<User | null> {
    const rows = await db.select().from(ba_users).where(eq(ba_users.id, id)).limit(1);
    return rows[0] ?? null;
}

export async function listUsers(): Promise<User[]> {
    return db.select().from(ba_users).orderBy(asc(ba_users.createdAt));
}

// Vue envoyée à la console admin (même forme qu'avant la fusion des tables)
export function adminView(u: User) {
    return {
        pseudo:          u.name,
        role:            u.role,
        active:          u.active,
        approved:        u.approved,
        createdAt:       Math.floor(u.createdAt.getTime() / 1000),
        accessJellyfin:  u.accessJellyfin,
        accessSongsurf:  u.accessSongsurf,
        requestJellyfin: u.requestJellyfin,
        requestSongsurf: u.requestSongsurf,
        githubUsername:  u.githubUsername,
        linkedinName:    u.linkedinName,
    };
}

export async function createUser(
    input: { pseudo: string; passwordHash: string; role?: string; approved?: boolean },
    tx: Tx | typeof db = db,
): Promise<User> {
    const username = usernameOf(input.pseudo);
    const id = randomUUID();
    const [user] = await tx.insert(ba_users).values({
        id,
        name:            input.pseudo.trim(),
        email:           `${username}@local.invalid`,
        username,
        displayUsername: input.pseudo.trim(),
        role:            input.role ?? 'member',
        approved:        input.approved ?? false,
        updatedAt:       new Date(),
    }).returning();
    await tx.insert(ba_accounts).values({
        id:         randomUUID(),
        accountId:  id,
        providerId: 'credential',
        userId:     id,
        password:   input.passwordHash,
        updatedAt:  new Date(),
    });
    return user;
}

export async function updateUser(userId: string, patch: UserPatch): Promise<void> {
    await db.update(ba_users).set({ ...patch, updatedAt: new Date() }).where(eq(ba_users.id, userId));
}

// Remplace (ou crée) le mot de passe : pas de credential = connexion impossible
export async function setPassword(userId: string, passwordHash: string): Promise<void> {
    const updated = await db
        .update(ba_accounts)
        .set({ password: passwordHash, updatedAt: new Date() })
        .where(and(eq(ba_accounts.userId, userId), eq(ba_accounts.providerId, 'credential')))
        .returning({ id: ba_accounts.id });
    if (updated.length === 0) {
        await db.insert(ba_accounts).values({
            id:         randomUUID(),
            accountId:  userId,
            providerId: 'credential',
            userId,
            password:   passwordHash,
            updatedAt:  new Date(),
        });
    }
}

// true si un mot de passe existait
export async function removePassword(userId: string): Promise<boolean> {
    const rows = await db
        .delete(ba_accounts)
        .where(and(eq(ba_accounts.userId, userId), eq(ba_accounts.providerId, 'credential')))
        .returning({ id: ba_accounts.id });
    return rows.length > 0;
}

export async function revokeSessions(userId: string): Promise<void> {
    await db.delete(ba_sessions).where(eq(ba_sessions.userId, userId));
}

// Sessions, credential, clés admin et avatar partent en cascade (FK)
export async function deleteUser(userId: string): Promise<void> {
    await db.delete(ba_users).where(eq(ba_users.id, userId));
}

// ─── Avatars ────────────────────────────────────────────────────────────────

export async function getAvatar(userId: string) {
    const rows = await db.select().from(userAvatars).where(eq(userAvatars.userId, userId)).limit(1);
    return rows[0] ?? null;
}

export async function getAvatarFilename(userId: string): Promise<string | null> {
    const rows = await db
        .select({ filename: userAvatars.filename })
        .from(userAvatars)
        .where(eq(userAvatars.userId, userId))
        .limit(1);
    return rows[0]?.filename ?? null;
}

export async function setAvatar(
    userId: string,
    avatar: { svg: string; filename: string },
    tx: Tx | typeof db = db,
): Promise<void> {
    const values = {
        bytes:     Buffer.from(avatar.svg),
        mimeType:  'image/svg+xml',
        filename:  avatar.filename,
        sizeBytes: avatar.svg.length,
        updatedAt: new Date(),
    };
    await tx.insert(userAvatars)
        .values({ userId, ...values })
        .onConflictDoUpdate({ target: userAvatars.userId, set: values });
}

export async function deleteAvatar(userId: string): Promise<void> {
    await db.delete(userAvatars).where(eq(userAvatars.userId, userId));
}
