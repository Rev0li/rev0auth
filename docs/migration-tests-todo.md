# Tests à effectuer — handoff Claude

> **Ce fichier est réécrit à chaque nouvelle tâche par le Claude qui code.**
> Il décrit la **dernière tâche réalisée** et liste **les tests à exécuter** pour la valider.
> Branche : `dev/svelte` · Projet : `auth/frontend/`

---

## Dernière tâche réalisée

**Date** : 2026-06-12
**Tâche** : Phase 2 — script de migration des comptes `web_users` → `ba_users`/`ba_accounts` + support Argon2 dans BetterAuth.

### Fichiers touchés
- `frontend/scripts/migrate-web-users-to-ba.mjs` — **nouveau** : script de migration (idempotent, `--dry-run`)
- `frontend/src/lib/server/auth-v2.ts` — `password.hash`/`password.verify` custom (Argon2 + fallback scrypt)
- `docs/migration-svelte-betterauth.md` — point de reprise mis à jour

### État : testé en local ✅

| Test | Résultat |
|---|---|
| Dry-run → migration réelle → re-run (idempotence : tout "déjà présent") | PASS |
| Login BetterAuth compte migré (`POST /api/auth/sign-in/username`, pseudo casse mixte → username lowercase) | 200 + session |
| Mauvais mot de passe | 401 |
| Hash corrompu (non-Argon2) en DB | 401 (pas de 500) |
| Compte sans credential (mdp retiré par admin) | 401 |
| Rate limit BetterAuth sur sign-in en rafale | 429 |
| Sign-up neuf → hash stocké en **Argon2** (compatible Rust) | PASS |
| `npm run check` | 0 erreur / 22 warnings |
| `npm test` (vitest) | 26/26 |

Compte de test local laissé en DB : `MigrTester` / `MigrTest123!` (web_users + ba_users, role member).

### À savoir

1. **Le script ne touche pas aux flags métier** (`approved`, `active`, `access_*`) : ils restent dans `web_users`, jointure par `username = LOWER(pseudo)`.
2. **Email synthétique** `<username>@local.invalid` — aucun flow email branché.
3. **Les nouveaux mots de passe BetterAuth sont hashés en Argon2** (pas scrypt) pour rester lisibles par `crates/api` et `auth.ts` pendant la coexistence. Le fallback scrypt couvre les comptes de test créés avant ce changement.
4. **Pas encore de synchro continue** `web_users` ↔ `ba_users` : un compte créé côté Rust après l'exécution du script nécessite de relancer le script (il est idempotent, c'est prévu pour).

---

## Tests restants pour la prochaine session

### 1. Sur la DB VPS (au prochain deploy)
```bash
# Depuis le VPS, dans le container ou avec DATABASE_URL prod :
node --env-file=.env scripts/migrate-web-users-to-ba.mjs --dry-run   # vérifier le compte rendu
node --env-file=.env scripts/migrate-web-users-to-ba.mjs
# Puis login BetterAuth avec un compte réel :
curl -X POST https://rev0li.duckdns.org/api/auth/sign-in/username \
  -H 'Content-Type: application/json' -d '{"username":"<pseudo>","password":"<mdp>"}'
```

### 2. Hérités des sessions précédentes (toujours en attente)
- [ ] Navigateur : bouton "Rafraîchir" de `/japprends/audit`, grille d'avatars profil + signup
- [ ] Deploy VPS : healthcheck frontend healthy, `/status` api_ok:true, `ORIGIN` défini, flow SongSurf complet
- [ ] Nettoyer `SONGSURF_JWT_SECRET` des `.env`/`.secrets` (VPS + local)

### 3. Validation type-check
```bash
cd /home/revoli/dev/rev0Univers/auth/frontend
npm run check   # attendu : 0 ERRORS / 22 WARNINGS
npm test        # attendu : 26/26
```

---

## Prochaine étape : Phase 2 (suite)

Voir le point de reprise dans [`migration-svelte-betterauth.md`](migration-svelte-betterauth.md) : RBAC sur `ba_users.role`, puis bascule du flow login membre sur BetterAuth.
