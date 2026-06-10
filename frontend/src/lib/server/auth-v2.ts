import { betterAuth } from 'better-auth';
import { drizzleAdapter } from 'better-auth/adapters/drizzle';
import { username } from 'better-auth/plugins';
// Imports relatifs (pas $lib) : le CLI @better-auth/cli charge ce fichier hors Vite
import { db } from './db/index.js';

// Phase 2 — instance BetterAuth montée en PARALLÈLE de l'auth custom
// (auth.ts / session.ts / ratelimit.ts restent la référence tant que les flows
// ne sont pas migrés). Tables préfixées ba_* : aucune collision avec web_* ni
// avec les auth_* de crates/api.
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
