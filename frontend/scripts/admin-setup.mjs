#!/usr/bin/env node
// Donne le rôle admin à un membre et affiche un code d'installation à usage
// unique (15 min) pour enregistrer sa clé de sécurité dans la console admin.
// Sert aussi de secours en cas de clé perdue. Si le membre n'existe pas encore
// (installation neuve), le compte est créé avec un mot de passe temporaire.
//
// Sur le serveur :  docker compose exec frontend node scripts/admin-setup.mjs <pseudo>
// En local :        node --env-file=.env scripts/admin-setup.mjs <pseudo>
import { createHash, randomBytes, randomUUID } from 'node:crypto';
import argon2 from 'argon2';
import postgres from 'postgres';

const SETUP_CODE_TTL_MIN = 15;
// 32 caractères sans ambiguïté (ni I, O, 0, 1) : 256 % 32 = 0, tirage sans biais
const ALPHABET = 'ABCDEFGHJKLMNPQRSTUVWXYZ23456789';

function randomString(length) {
    return [...randomBytes(length)].map((b) => ALPHABET[b % ALPHABET.length]).join('');
}

const pseudo = process.argv[2]?.trim();
if (!pseudo) {
    console.error('Usage : node scripts/admin-setup.mjs <pseudo>');
    process.exit(1);
}
const username = pseudo.toLowerCase();

const sql = postgres(process.env.DATABASE_URL ?? 'postgres://postgres:postgres@localhost:5432/rev0auth', {
    onnotice: () => {},
});

try {
    let tempPassword = null;

    const [member] = await sql`SELECT id, name, active FROM ba_users WHERE username = ${username}`;
    if (member && !member.active) {
        console.error(`Le compte « ${member.name} » est désactivé : réactive-le d'abord.`);
        process.exit(1);
    }

    const code = randomString(16);
    const codeHash = createHash('sha256').update(code).digest('hex');

    await sql.begin(async (tx) => {
        let userId = member?.id;
        if (!userId) {
            // Installation neuve : compte créé (mêmes conventions que users.ts)
            userId = randomUUID();
            await tx`
                INSERT INTO ba_users (id, name, email, username, display_username, role, active, approved)
                VALUES (${userId}, ${username}, ${username + '@local.invalid'}, ${username}, ${username}, 'admin', true, true)`;
        } else {
            await tx`UPDATE ba_users SET role = 'admin', updated_at = now() WHERE id = ${userId}`;
        }

        // Sans mot de passe, impossible de se connecter : on en pose un temporaire
        const [credential] = await tx`
            SELECT id FROM ba_accounts WHERE user_id = ${userId} AND provider_id = 'credential' AND password IS NOT NULL`;
        if (!credential) {
            tempPassword = randomString(20);
            await tx`DELETE FROM ba_accounts WHERE user_id = ${userId} AND provider_id = 'credential'`;
            await tx`
                INSERT INTO ba_accounts (id, account_id, provider_id, user_id, password)
                VALUES (${randomUUID()}, ${userId}, 'credential', ${userId}, ${await argon2.hash(tempPassword)})`;
        }

        await tx`DELETE FROM ba_verifications WHERE identifier = ${'admin-setup:' + username}`;
        await tx`
            INSERT INTO ba_verifications (id, identifier, value, expires_at)
            VALUES (${randomUUID()}, ${'admin-setup:' + username}, ${codeHash},
                    now() + make_interval(mins => ${SETUP_CODE_TTL_MIN}))`;

        await tx`
            INSERT INTO web_audit_log (action, actor_pseudo, target, detail)
            VALUES ('admin_setup_code', 'cli', ${member?.name ?? username}, ${'role admin + code ' + SETUP_CODE_TTL_MIN + ' min'})`;
    });

    console.log(`\nCompte « ${member?.name ?? username} » : rôle admin.`);
    if (tempPassword) console.log(`Mot de passe temporaire : ${tempPassword}   (à changer dans ton profil)`);
    console.log(`Code d'installation   : ${code.match(/.{4}/g).join('-')}   (valable ${SETUP_CODE_TTL_MIN} min, usage unique)`);
    console.log('\n1. Connecte-toi normalement avec ton pseudo et ton mot de passe.');
    console.log('2. Ouvre « Console admin », entre ce code, puis touche ta clé de sécurité.\n');
} finally {
    await sql.end();
}
