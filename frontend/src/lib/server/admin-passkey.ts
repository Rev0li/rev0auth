import { createHash, randomUUID } from 'node:crypto';
import { error } from '@sveltejs/kit';
import { and, asc, eq, gt } from 'drizzle-orm';
import {
    generateRegistrationOptions,
    verifyRegistrationResponse,
    generateAuthenticationOptions,
    verifyAuthenticationResponse,
} from '@simplewebauthn/server';
import type {
    AuthenticationResponseJSON,
    AuthenticatorTransportFuture,
    RegistrationResponseJSON,
} from '@simplewebauthn/server';
import { db } from './db/index.js';
import { adminPasskeys } from './db/schema.js';
import { ba_sessions, ba_users, ba_verifications } from './db/auth-schema.js';
import { consoleWebAuthnOrigin } from './console-host.js';

// Console admin = rôle admin + clé de sécurité (WebAuthn) validée dans la
// session BetterAuth en cours. La validation est datée sur ba_sessions
// (admin_verified_at) : elle disparaît avec la session (déconnexion, expiration).
//
// Première clé (ou clé perdue) : code d'installation à usage unique généré sur
// le serveur par scripts/admin-setup.mjs. Clé supplémentaire : depuis une
// session déjà validée.
//
// Connexion par la clé seule (AUTH-21) : sur le nom de la console, la clé
// identifie l'admin, PIN de la clé exigé ; la session ouverte est validée.

export const ADMIN_ELEVATION_TTL_MS = 12 * 60 * 60 * 1000;
const CHALLENGE_TTL_MS = 5 * 60 * 1000;

const RP_NAME = 'rev0auth';

// La console peut avoir son propre nom (ADMIN_ORIGIN, AUTH-11) ; le RP ID
// reste le nom du portail, domaine parent : les clés déjà enregistrées valent
// sur les deux noms.
function expectedOrigin(): string {
    return consoleWebAuthnOrigin();
}

function rpID(): string {
    return new URL(process.env.ORIGIN ?? 'http://localhost:5173').hostname;
}

function splitTransports(value: string): AuthenticatorTransportFuture[] | undefined {
    return value ? (value.split(',') as AuthenticatorTransportFuture[]) : undefined;
}

// ─── Codes d'installation ───────────────────────────────────────────────────

const SETUP_PREFIX = 'admin-setup:';

// Saisie tolérante : casse, espaces et tirets ignorés
export function normalizeSetupCode(code: string): string {
    return code.toUpperCase().replace(/[^A-Z0-9]/g, '');
}

export function hashSetupCode(code: string): string {
    return createHash('sha256').update(normalizeSetupCode(code)).digest('hex');
}

async function findSetupCode(username: string) {
    const rows = await db
        .select()
        .from(ba_verifications)
        .where(eq(ba_verifications.identifier, SETUP_PREFIX + username))
        .limit(1);
    const row = rows[0];
    return row && row.expiresAt.getTime() > Date.now() ? row : null;
}

export async function setupCodeValid(username: string, code: string): Promise<boolean> {
    const row = await findSetupCode(username);
    return !!row && !!code && row.value === hashSetupCode(code);
}

// Consommation atomique : un code ne sert qu'une fois, même en parallèle
async function consumeSetupCode(username: string): Promise<boolean> {
    const rows = await db
        .delete(ba_verifications)
        .where(and(
            eq(ba_verifications.identifier, SETUP_PREFIX + username),
            gt(ba_verifications.expiresAt, new Date()),
        ))
        .returning({ id: ba_verifications.id });
    return rows.length > 0;
}

// ─── Défis WebAuthn : un seul par session (ou par connexion) et par type ────
// Usage unique. `owner` = id de la session, ou pour la connexion par la clé
// seule un identifiant aléatoire gardé dans un cookie le temps du défi.

type ChallengeKind = 'register' | 'authenticate' | 'signin';
interface StoredChallenge { challenge: string; viaCode?: boolean }

async function saveChallenge(kind: ChallengeKind, owner: string, data: StoredChallenge) {
    const identifier = `webauthn-${kind}:${owner}`;
    await db.delete(ba_verifications).where(eq(ba_verifications.identifier, identifier));
    await db.insert(ba_verifications).values({
        id: randomUUID(),
        identifier,
        value: JSON.stringify(data),
        expiresAt: new Date(Date.now() + CHALLENGE_TTL_MS),
    });
}

async function takeChallenge(kind: ChallengeKind, owner: string): Promise<StoredChallenge | null> {
    const rows = await db
        .delete(ba_verifications)
        .where(eq(ba_verifications.identifier, `webauthn-${kind}:${owner}`))
        .returning();
    const row = rows[0];
    if (!row || row.expiresAt.getTime() <= Date.now()) return null;
    return JSON.parse(row.value) as StoredChallenge;
}

// ─── Clés ───────────────────────────────────────────────────────────────────

export async function listPasskeys(userId: string) {
    return db
        .select({
            id:         adminPasskeys.id,
            name:       adminPasskeys.name,
            createdAt:  adminPasskeys.createdAt,
            lastUsedAt: adminPasskeys.lastUsedAt,
        })
        .from(adminPasskeys)
        .where(eq(adminPasskeys.userId, userId))
        .orderBy(asc(adminPasskeys.createdAt));
}

export async function deletePasskey(userId: string, id: number): Promise<string | null> {
    const rows = await db
        .delete(adminPasskeys)
        .where(and(eq(adminPasskeys.id, id), eq(adminPasskeys.userId, userId)))
        .returning({ name: adminPasskeys.name });
    return rows[0]?.name ?? null;
}

async function userCredentials(userId: string) {
    return db.select().from(adminPasskeys).where(eq(adminPasskeys.userId, userId));
}

interface AdminUser { userId: string; sessionId: string; pseudo: string }

// Membre connecté avec le rôle admin (clé pas forcément encore validée)
export function requireAdminMember(locals: App.Locals): AdminUser {
    const member = locals.memberSession;
    if (!member) throw error(401, 'Non autorisé.');
    if (member.role !== 'admin') throw error(403, 'Accès réservé aux administrateurs.');
    return { userId: member.userId, sessionId: member.sessionId, pseudo: member.pseudo };
}

export async function registrationOptions(user: AdminUser, viaCode: boolean) {
    const existing = await userCredentials(user.userId);
    const options = await generateRegistrationOptions({
        rpName: RP_NAME,
        rpID: rpID(),
        userName: user.pseudo,
        userID: new TextEncoder().encode(user.userId),
        attestationType: 'none',
        excludeCredentials: existing.map((c) => ({ id: c.credentialId, transports: splitTransports(c.transports) })),
        authenticatorSelection: {
            authenticatorAttachment: 'cross-platform',
            residentKey: 'discouraged',
            userVerification: 'preferred',
        },
        preferredAuthenticatorType: 'securityKey',
    });
    await saveChallenge('register', user.sessionId, { challenge: options.challenge, viaCode });
    return options;
}

// Enregistre la clé ; un code d'installation utilisé est consommé.
// Retourne null si la vérification échoue.
export async function verifyRegistration(
    user: AdminUser,
    response: RegistrationResponseJSON,
    name: string,
): Promise<{ viaCode: boolean } | null> {
    const stored = await takeChallenge('register', user.sessionId);
    if (!stored) return null;

    let verification;
    try {
        verification = await verifyRegistrationResponse({
            response,
            expectedChallenge: stored.challenge,
            expectedOrigin: expectedOrigin(),
            expectedRPID: rpID(),
            requireUserVerification: false,
        });
    } catch {
        return null;
    }
    if (!verification.verified) return null;

    if (stored.viaCode && !(await consumeSetupCode(user.pseudo.toLowerCase()))) return null;

    const { credential } = verification.registrationInfo;
    try {
        await db.insert(adminPasskeys).values({
            userId:       user.userId,
            credentialId: credential.id,
            publicKey:    Buffer.from(credential.publicKey).toString('base64url'),
            counter:      credential.counter,
            transports:   (credential.transports ?? []).join(','),
            name:         name.trim().slice(0, 60) || 'Clé de sécurité',
        });
    } catch {
        return null; // clé déjà enregistrée
    }
    return { viaCode: !!stored.viaCode };
}

// null si l'utilisateur n'a encore aucune clé
export async function authenticationOptions(user: AdminUser) {
    const credentials = await userCredentials(user.userId);
    if (credentials.length === 0) return null;
    const options = await generateAuthenticationOptions({
        rpID: rpID(),
        allowCredentials: credentials.map((c) => ({ id: c.credentialId, transports: splitTransports(c.transports) })),
        userVerification: 'preferred',
    });
    await saveChallenge('authenticate', user.sessionId, { challenge: options.challenge });
    return options;
}

export async function verifyAuthentication(user: AdminUser, response: AuthenticationResponseJSON): Promise<boolean> {
    const stored = await takeChallenge('authenticate', user.sessionId);
    if (!stored || typeof response?.id !== 'string') return false;

    // Seules les clés de l'utilisateur connecté sont acceptées
    const rows = await db
        .select()
        .from(adminPasskeys)
        .where(and(eq(adminPasskeys.credentialId, response.id), eq(adminPasskeys.userId, user.userId)))
        .limit(1);
    const passkey = rows[0];
    if (!passkey) return false;

    let verification;
    try {
        verification = await verifyAuthenticationResponse({
            response,
            expectedChallenge: stored.challenge,
            expectedOrigin: expectedOrigin(),
            expectedRPID: rpID(),
            credential: {
                id:         passkey.credentialId,
                publicKey:  new Uint8Array(Buffer.from(passkey.publicKey, 'base64url')),
                counter:    passkey.counter,
                transports: splitTransports(passkey.transports),
            },
            requireUserVerification: false,
        });
    } catch {
        return false;
    }
    if (!verification.verified) return false;

    await db
        .update(adminPasskeys)
        .set({ counter: verification.authenticationInfo.newCounter, lastUsedAt: new Date() })
        .where(eq(adminPasskeys.id, passkey.id));
    return true;
}

// ─── Connexion par la clé seule (AUTH-21) ───────────────────────────────────

// Clés des admins actifs : la clé présentée désigne l'admin
async function activeAdminCredentials() {
    return db
        .select({ passkey: adminPasskeys, pseudo: ba_users.name })
        .from(adminPasskeys)
        .innerJoin(ba_users, eq(ba_users.id, adminPasskeys.userId))
        .where(and(eq(ba_users.role, 'admin'), eq(ba_users.active, true)));
}

export const SIGNIN_CHALLENGE_TTL_S = CHALLENGE_TTL_MS / 1000;
export const SIGNIN_COOKIE = 'admin_signin';

// null si aucun admin n'a de clé ; `challengeId` va dans un cookie httpOnly
export async function signinOptions() {
    const credentials = await activeAdminCredentials();
    if (credentials.length === 0) return null;
    const options = await generateAuthenticationOptions({
        rpID: rpID(),
        allowCredentials: credentials.map(({ passkey: c }) => ({ id: c.credentialId, transports: splitTransports(c.transports) })),
        userVerification: 'required',
    });
    const challengeId = randomUUID();
    await saveChallenge('signin', challengeId, { challenge: options.challenge });
    return { options, challengeId };
}

// L'admin identifié par sa clé, ou null. PIN (vérification de l'utilisateur)
// exigé : la clé seule ne suffit pas.
export async function verifySignin(
    challengeId: string,
    response: AuthenticationResponseJSON,
): Promise<{ userId: string; pseudo: string } | null> {
    const stored = await takeChallenge('signin', challengeId);
    if (!stored || typeof response?.id !== 'string') return null;

    const match = (await activeAdminCredentials()).find(({ passkey }) => passkey.credentialId === response.id);
    if (!match) return null;
    const { passkey, pseudo } = match;

    let verification;
    try {
        verification = await verifyAuthenticationResponse({
            response,
            expectedChallenge: stored.challenge,
            expectedOrigin: expectedOrigin(),
            expectedRPID: rpID(),
            credential: {
                id:         passkey.credentialId,
                publicKey:  new Uint8Array(Buffer.from(passkey.publicKey, 'base64url')),
                counter:    passkey.counter,
                transports: splitTransports(passkey.transports),
            },
            requireUserVerification: true,
        });
    } catch {
        return null;
    }
    if (!verification.verified || !verification.authenticationInfo.userVerified) return null;

    await db
        .update(adminPasskeys)
        .set({ counter: verification.authenticationInfo.newCounter, lastUsedAt: new Date() })
        .where(eq(adminPasskeys.id, passkey.id));
    return { userId: passkey.userId, pseudo };
}

// ─── Validation de la session ───────────────────────────────────────────────

export async function elevateSession(sessionId: string): Promise<void> {
    await db.update(ba_sessions).set({ adminVerifiedAt: new Date() }).where(eq(ba_sessions.id, sessionId));
}

export async function isSessionElevated(sessionId: string): Promise<boolean> {
    const rows = await db
        .select({ verifiedAt: ba_sessions.adminVerifiedAt })
        .from(ba_sessions)
        .where(eq(ba_sessions.id, sessionId))
        .limit(1);
    const verifiedAt = rows[0]?.verifiedAt;
    return !!verifiedAt && Date.now() - verifiedAt.getTime() < ADMIN_ELEVATION_TTL_MS;
}
