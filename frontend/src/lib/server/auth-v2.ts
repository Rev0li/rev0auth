import { betterAuth, type BetterAuthPlugin } from 'better-auth';
import { createAuthEndpoint, APIError } from 'better-auth/api';
import { setSessionCookie } from 'better-auth/cookies';
import { drizzleAdapter } from 'better-auth/adapters/drizzle';
import { verifyPassword as verifyScrypt } from 'better-auth/crypto';
import { username } from 'better-auth/plugins';
import argon2 from 'argon2';
import { z } from 'zod';
// Imports relatifs (pas $lib) : le CLI @better-auth/cli charge ce fichier hors Vite
import { db } from './db/index.js';

// Instance BetterAuth : le seul modèle utilisateur (ba_users = identité +
// champs métier). Connexion pseudo + mot de passe via le plugin username ;
// l'email reste requis par le core → `<pseudo>@local.invalid` (pas d'envoi).
// Comptes créés uniquement côté serveur : voir users.ts.

const origin = process.env.ORIGIN ?? 'http://localhost:5173';

// Session partagée par tous les sous-domaines du portail (songsurf.,
// jellyfin.…) : Caddy la transmet à l'endpoint de vérification (forward_auth).
// Domaine déduit d'ORIGIN ; pas en dev (http://localhost), où le cookie reste
// lié à l'hôte.
export const cookieDomain = origin.startsWith('https://') ? new URL(origin).hostname : undefined;

// Console admin, connexion par la clé seule (AUTH-21) : une fois la clé
// vérifiée (admin-passkey.ts), ouvre une session BetterAuth normale pour cet
// admin et pose son cookie. SERVER_ONLY : jamais routée, appelable seulement
// par auth.api côté serveur.
const adminKeySession = {
    id: 'admin-key-session',
    endpoints: {
        createAdminKeySession: createAuthEndpoint(
            '/admin-key-session',
            { method: 'POST', body: z.object({ userId: z.string() }), metadata: { SERVER_ONLY: true } },
            async (ctx) => {
                const user = await ctx.context.internalAdapter.findUserById(ctx.body.userId);
                if (!user) throw new APIError('NOT_FOUND');
                const session = await ctx.context.internalAdapter.createSession(user.id);
                if (!session) throw new APIError('INTERNAL_SERVER_ERROR');
                await setSessionCookie(ctx, { session, user });
                return ctx.json({ sessionId: session.id });
            },
        ),
    },
} satisfies BetterAuthPlugin;

export const auth = betterAuth({
    database: drizzleAdapter(db, { provider: 'pg' }),
    secret: process.env.BETTER_AUTH_SECRET,
    baseURL: origin,
    // Console sous son propre nom (AUTH-11) : déconnexion appelée depuis ce nom
    trustedOrigins: process.env.ADMIN_ORIGIN ? [process.env.ADMIN_ORIGIN] : [],

    emailAndPassword: {
        enabled: true,
        // Comptes créés uniquement côté serveur (invitation, admin) : pas
        // d'inscription directe par BetterAuth.
        disableSignUp: true,
        minPasswordLength: 8,
        // Hash Argon2, comme hashPassword (auth.ts) utilisé à la création des
        // comptes côté serveur. Le fallback scrypt couvre les plus anciens
        // comptes BetterAuth.
        password: {
            hash: (password) => argon2.hash(password),
            verify: async ({ hash, password }) => {
                if (hash.startsWith('$argon2')) {
                    try {
                        return await argon2.verify(hash, password);
                    } catch {
                        return false; // hash corrompu → 401, pas 500 (cf. S1)
                    }
                }
                try {
                    return await verifyScrypt({ hash, password });
                } catch {
                    return false;
                }
            },
        },
    },

    plugins: [username(), adminKeySession],

    user: {
        modelName: 'ba_users',
        // Champs métier (ex-web_users) : renvoyés avec la session, jamais
        // modifiables par le client (input: false).
        additionalFields: {
            // RBAC : guest / member / mod / admin
            role:            { type: 'string',  defaultValue: 'member', input: false },
            active:          { type: 'boolean', defaultValue: true,     input: false },
            approved:        { type: 'boolean', defaultValue: false,    input: false },
        },
    },
    session: {
        modelName: 'ba_sessions',
        expiresIn: 24 * 60 * 60, // aligné sur MEMBER_TTL_S
    },
    account: { modelName: 'ba_accounts' },
    verification: { modelName: 'ba_verifications' },

    advanced: {
        crossSubDomainCookies: { enabled: !!cookieDomain, domain: cookieDomain },
        defaultCookieAttributes: { httpOnly: true, sameSite: 'lax' },
    },

    rateLimit: {
        enabled: true,
        window: 60,
        max: 20,
    },
});
