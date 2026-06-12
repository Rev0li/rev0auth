#!/usr/bin/env node
// Migration des comptes web_users → ba_users / ba_accounts (Phase 2 BetterAuth)
//
// Usage :
//   cd auth/frontend
//   node --env-file=.env scripts/migrate-web-users-to-ba.mjs --dry-run   # aperçu
//   node --env-file=.env scripts/migrate-web-users-to-ba.mjs             # migration
//
// Idempotent : un pseudo déjà présent dans ba_users.username est ignoré,
// le script peut être relancé sans risque (utile pour migrer les comptes
// créés côté Rust entre deux exécutions).
//
// Règles :
//   - username = pseudo en lowercase (clé de jointure avec web_users),
//     display_username = pseudo d'origine
//   - email synthétique <username>@local.invalid (décision Phase 2 : le core
//     BetterAuth exige un email, aucun envoi de mail n'est branché)
//   - password_hash Argon2 copié tel quel dans ba_accounts.password
//     (provider 'credential') — la vérification Argon2 est gérée par le
//     password.verify custom de auth-v2.ts
//   - password_hash vide (mot de passe retiré par l'admin) → pas de ligne
//     ba_accounts : connexion impossible jusqu'à réinitialisation, comme en Rust
//   - les flags métier (approved, active, access_*) restent dans web_users,
//     seuls identité + rôle + credential sont portés

import { randomUUID } from 'node:crypto';
import postgres from 'postgres';

const dryRun = process.argv.includes('--dry-run');

const url = process.env.DATABASE_URL;
if (!url) {
    console.error('DATABASE_URL manquant — lancer avec `node --env-file=.env`');
    process.exit(1);
}

const sql = postgres(url, { onnotice: () => {} });

const webUsers = await sql`
    SELECT pseudo, role, password_hash, created_at_epoch
    FROM web_users
    ORDER BY created_at_epoch
`;
const alreadyMigrated = new Set(
    (await sql`SELECT username FROM ba_users WHERE username IS NOT NULL`).map((r) => r.username)
);

let migrated = 0;
let skipped = 0;
let withoutCredential = 0;
let suspectHash = 0;

for (const u of webUsers) {
    const username = u.pseudo.toLowerCase();
    if (alreadyMigrated.has(username)) {
        skipped++;
        console.log(`=  ${u.pseudo} — déjà dans ba_users, ignoré`);
        continue;
    }

    const hasPassword = u.password_hash !== '';
    if (hasPassword && !u.password_hash.startsWith('$argon2')) {
        suspectHash++;
        console.warn(
            `!  ${u.pseudo} — hash non-Argon2 (${u.password_hash.slice(0, 12)}…) : copié tel quel, la connexion échouera tant que le mot de passe n'est pas réinitialisé`
        );
    }
    if (!hasPassword) {
        withoutCredential++;
        console.warn(`!  ${u.pseudo} — password_hash vide : migré sans credential (connexion impossible jusqu'à réinitialisation)`);
    }

    if (dryRun) {
        migrated++;
        console.log(`+  ${u.pseudo} → username=${username}, role=${u.role}${hasPassword ? '' : ', sans credential'} (dry-run)`);
        continue;
    }

    const userId = randomUUID();
    const createdAt = new Date(Number(u.created_at_epoch) * 1000);

    await sql.begin(async (tx) => {
        await tx`
            INSERT INTO ba_users (id, name, email, email_verified, created_at, updated_at, username, display_username, role)
            VALUES (${userId}, ${u.pseudo}, ${`${username}@local.invalid`}, FALSE, ${createdAt}, NOW(), ${username}, ${u.pseudo}, ${u.role})
        `;
        if (hasPassword) {
            await tx`
                INSERT INTO ba_accounts (id, account_id, provider_id, user_id, password, created_at, updated_at)
                VALUES (${randomUUID()}, ${userId}, 'credential', ${userId}, ${u.password_hash}, ${createdAt}, NOW())
            `;
        }
    });

    migrated++;
    console.log(`+  ${u.pseudo} → ba_users ${userId}${hasPassword ? ' + credential' : ''}`);
}

console.log('');
console.log(`${dryRun ? '[dry-run] ' : ''}Terminé : ${migrated} migré(s), ${skipped} déjà présent(s), ${withoutCredential} sans mot de passe, ${suspectHash} hash suspect(s) sur ${webUsers.length} compte(s) web_users.`);

await sql.end();
