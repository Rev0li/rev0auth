import { betterAuth } from 'better-auth';
import { drizzleAdapter } from 'better-auth/adapters/drizzle';
import { verifyPassword as verifyScrypt } from 'better-auth/crypto';
import { username } from 'better-auth/plugins';
import argon2 from 'argon2';
// Imports relatifs (pas $lib) : le CLI @better-auth/cli charge ce fichier hors Vite
import { db } from './db/index.js';

// Instance BetterAuth : identité membre (login pseudo + mot de passe).
// Tables préfixées ba_* : aucune collision avec les tables web_*.
//
// Décision login : le plugin username permet pseudo + mot de passe comme
// aujourd'hui ; l'email reste requis par le core → synthétisé en
// `<pseudo>@local.invalid` à la migration des comptes (pas d'envoi d'email).
export const auth = betterAuth({
    database: drizzleAdapter(db, { provider: 'pg' }),
    secret: process.env.BETTER_AUTH_SECRET ?? process.env.AUTH_JWT_SECRET,
    baseURL: process.env.ORIGIN ?? 'http://localhost:5173',

    emailAndPassword: {
        enabled: true,
        minPasswordLength: 8,
        // Hash Argon2 (cohérent avec les comptes migrés depuis web_users et
        // avec auth.ts pour le login admin). Le fallback scrypt couvre les
        // comptes créés via BetterAuth avant ce commit.
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

    plugins: [username()],

    user: {
        modelName: 'ba_users',
        additionalFields: {
            // RBAC existant : guest / member / mod / admin
            role: { type: 'string', defaultValue: 'member', input: false },
        },
    },
    session: {
        modelName: 'ba_sessions',
        expiresIn: 24 * 60 * 60, // aligné sur MEMBER_TTL_S
    },
    account: { modelName: 'ba_accounts' },
    verification: { modelName: 'ba_verifications' },

    rateLimit: {
        enabled: true,
        window: 60,
        max: 20,
    },
});
