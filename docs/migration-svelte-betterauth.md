# Migration SvelteKit + BetterAuth — Guideline

> Document de suivi de la migration `crates/web/` (Rust SSR) + auth custom → SvelteKit + BetterAuth + Drizzle.
> Branche : `dev/svelte`.
> **Mis à jour étape par étape — chaque case cochée est validée.**

Dernière mise à jour : 2026-06-10 — *Phase 1 testée (34+11 PASS) et commitée en blocs atomiques. Fixes de l'audit appliqués (S1-S4, P1-P5, compose, CI, env dev). Reprise → Phase 2 BetterAuth.*

---

## Décisions de stack (verrouillées)

| Composant | Choix | Raison |
|---|---|---|
| Framework | **SvelteKit** | Migration déjà entamée, SSR natif, full TypeScript |
| ORM | **Drizzle** | Déjà en place (`drizzle.config.ts`, schema, migrations) — pas de Prisma |
| Auth | **BetterAuth** | SvelteKit-native, TOTP intégré, rate limiting, adapter Drizzle officiel |
| DB | **PostgreSQL** | Inchangé |

**À retirer à terme** : `crates/web/` (Rust SSR), `auth.ts` / `session.ts` / `ratelimit.ts` custom.
**À conserver pour l'instant** : `crates/api/` (Rust API JWT pour SongSurf handoff).

---

## Phases

### Phase 1 — Compléter la migration des pages SvelteKit
**Objectif** : SvelteKit feature-complete par rapport à `crates/web/` avant de toucher à l'auth.

- [x] Inventaire des pages `crates/web/src/pages/` vs `frontend/src/routes/` *(2026-05-30)*
- [x] Lister les routes manquantes *(voir détail ci-dessous)*

#### Pages (`+page.svelte`)
- [x] `/japprends/audit` — vue du log d'audit admin *(2026-05-30)*
- [x] `/japprends/songsurf-logs` — viewer logs SongSurf *(2026-05-30)*
- [x] `/members/dashboard` — **redirect 301 vers `/home/friend`** (le Rust appelait la même fonction `pages::friend()`) *(2026-05-30)*
- ⏭️ Optionnel : `/japprends/endpoints` — peu prioritaire, skip
- ⏭️ Optionnel : `/dashboard` (decoy) — peu prioritaire, skip

> **Décision audit log 2026-05-30** : nouvelle table `web_audit_log` créée via `initDb()` (SvelteKit-only, Rust ne l'a pas). Helper `lib/server/audit.ts` `writeAudit()`. Les `console.info` des endpoints remove-password + wall-delete ont été remplacés par `writeAudit()`.

#### Endpoints WebAuthn (admin) — **SKIP, à faire en Phase 2 via BetterAuth**
- ⏭️ `GET /japprends/webauthn/register/start`
- ⏭️ `POST /japprends/webauthn/register/finish`
- ⏭️ `GET /japprends/webauthn/status`
- ⏭️ `POST /japprends/webauthn/remove`
- ⏭️ `GET /japprends/webauthn/credential/export`

> **Décision 2026-05-30** : on saute ces endpoints. BetterAuth passkey plugin les remplacera proprement en Phase 2. En attendant, `crates/web/` les sert toujours.

#### Endpoints Status manquants
- [x] `GET /status/all` *(2026-05-30)*
- [x] `POST /status/set-busy/:pseudo` *(2026-05-30)*
- [x] `POST /status/set-active/:pseudo` *(2026-05-30)*
- [x] `POST /status/set-inactive/:pseudo` *(2026-05-30)*

> Note 2026-05-30 : `/status` aussi mis à jour pour matcher le contrat Rust (`admin_ok/user_ok/api_ok/checked_at_epoch`, admin-only) au lieu du health check générique précédent. Helper `lib/server/api-health.ts` créé pour le check TCP vers `crates/api/`.

#### Endpoints Members
Après inspection : la plupart sont **déjà couverts** par les endpoints SvelteKit existants sous une autre forme (REST avec query params au lieu de paths). Décision : **on garde la forme SvelteKit**, les pages migrées appelleront ces URLs (les pages Rust SSR encore en place continuent à utiliser leurs handlers Rust).

| Rust | SvelteKit équivalent | Action |
|---|---|---|
| `POST /members/messages/send` | `POST /members/messages` `{to, body}` | ✅ couvert |
| `GET /members/messages/inbox` | `GET /members/messages?folder=inbox` | ✅ couvert |
| `GET /members/messages/sent` | `GET /members/messages?folder=sent` | ✅ couvert |
| `POST /members/messages/:id/read` | `PATCH /members/messages` `{id}` | ✅ couvert |
| `POST /members/donations/proof` | `POST /members/donations` | ✅ couvert + ajout validation method |
| `DELETE /members/avatar/:pseudo` | `DELETE /members/avatar` (self only) | ✅ couvert (self) — version `:pseudo` skippée (insécure côté Rust : pas d'auth) |
| `GET /members/donations/crypto-addresses` | — | ❌ à créer |
| `DELETE /members/account` | — | ❌ à créer |
| `DELETE /members/wall/:id` | `DELETE /members/wall?id=` | ⚠️ existe mais **bug** : pas d'ownership check |

- [x] `POST /members/donations` : ajout validation `method ∈ {crypto, pcs}` *(2026-05-30)*
- [x] `GET /members/donations/crypto-addresses` *(2026-05-30)*
- [x] `DELETE /members/account` — cascade messages + donations *(2026-05-30)*
- [x] `DELETE /members/wall` : fix ownership check *(2026-05-30)*

#### Endpoints Admin (`japprends`)
Idem Members : 3 sont **déjà couverts** sous une autre forme par les endpoints SvelteKit existants.

| Rust | SvelteKit équivalent | Action |
|---|---|---|
| `POST /japprends/set-password/:pseudo` | `POST /japprends/users/[pseudo]/password` | ✅ couvert |
| `POST /japprends/messages/mark-read` | `PATCH /japprends/messages` `{pseudo}` | ✅ couvert |
| `DELETE /japprends/messages/thread/:pseudo` | `DELETE /japprends/messages` `{pseudo}` | ✅ couvert |
| `GET /japprends/ping` | — | ✅ créé |
| `POST /japprends/auth-check` | — | ✅ créé |
| `GET /user/ping` | — | ✅ créé |
| `POST /japprends/remove-password/:pseudo` | `DELETE /japprends/users/[pseudo]/password` | ✅ créé (sous forme DELETE RESTful) |
| `DELETE /japprends/wall/:id` | `DELETE /japprends/wall/[id]` | ✅ créé (remplace l'ancien `?id=` query-based) |

- [x] `GET /japprends/ping` *(2026-05-30)*
- [x] `POST /japprends/auth-check` *(2026-05-30)*
- [x] `GET /user/ping` *(2026-05-30)*
- [x] `DELETE /japprends/users/[pseudo]/password` — remove password avec checks (existe + a un mdp) + audit console *(2026-05-30)*
- [x] `DELETE /japprends/wall/[id]` — admin delete any post + audit console *(2026-05-30)*

> Note 2026-05-30 : ancien `DELETE /japprends/wall?id=` retiré (était dupliqué avec le nouveau). `WallTab.svelte` mis à jour pour utiliser la nouvelle URL path-based. Audit fait via `console.info('[admin-audit] ...')` au lieu d'une table dédiée — match la nature éphémère du log Rust en mémoire.

#### Assets statiques
- [ ] **TODO différé** : `/static/hero/:filename` et `/static/tuto/:filename` sont dans `auth/static/`, pas dans `frontend/static/`. Aucune référence côté SvelteKit actuel — à traiter quand les pages Rust qui les consomment seront migrées (copier les assets dans `frontend/static/hero/` + `frontend/static/tuto/`, SvelteKit les servira automatiquement).

#### Migration & validation
- [x] Migrer pages manquantes (ordre suggéré : audit → songsurf-logs → members/dashboard) *(2026-05-30)*
- [x] Migrer endpoints manquants par groupe (status → members → admin) *(2026-05-30)*
- [x] Tests des blocs : 34 PASS / 0 FAIL (handoff + smoke par rôle admin/membre) *(2026-06-10)*
- [ ] Vérifier parité styles (CSS pages)
- [ ] Test navigateur : grille avatars profil + bouton refresh audit

> **Note 2026-06-10 — audit & fixes** : audit complet dans `docs/audit-migration-svelte-2026-06-10.md`.
> Appliqués : S1 (verifyPassword hash vide), S2 (rate limit login membre), S3 (refactor `avatar_id`,
> plus d'upload SVG — catalogue `$lib/avatars.ts`), S4 (secret unifié `AUTH_JWT_SECRET`, helper
> `$lib/server/songsurf.ts` — corrigeait un vrai 502), P1 (cascade+audit delete_user), P2 (pseudo
> lowercase au signup, lookups LOWER()), P4 (audit set_password), P5 (min 8 chars). Compose : healthcheck
> node, `REV0AUTH_API_UPSTREAM`, `ORIGIN`. CI `ci.yml` (check+vitest+cargo). `loadEnv` dans vite.config
> (le dev ne chargeait pas `.env` → les tests manuels n'avaient jamais pu tourner).
> **Page `GET /portal` non migrée** (seul le Rust la sert) — ajoutée à la checklist Phase 3.

### Phase 2 — Intégrer BetterAuth sur Drizzle
**Objectif** : remplacer l'auth custom par BetterAuth sans casser le schema partagé.

- [x] Installer `better-auth` + adapter Drizzle *(2026-06-10 — v1.6.16)*
- [x] Configurer BetterAuth (instance `lib/server/auth-v2.ts`, handler `/api/auth/[...all]`) *(2026-06-10)*

> **Note 2026-06-10 — choix de schéma** : tables dédiées `ba_*` (générées par `@better-auth/cli`,
> créées via `initDb()`) plutôt qu'un mapping direct sur `web_users` : le core BetterAuth exige
> email/id/timestamps incompatibles avec le shape `web_users` (pseudo PK, epochs). La migration des
> comptes se fera par script `web_users` → `ba_users` + `ba_accounts` (email synthétique
> `<pseudo>@local.invalid`, plugin `username` pour le login pseudo+mdp). `web_users` reste la table
> des données applicatives (rôle, accès, avatar…). Testé : sign-up, sign-in/username, get-session.

- [ ] Script de migration des comptes `web_users` → `ba_users`/`ba_accounts` (hashes Argon2 à porter — vérifier la compat `password` de ba_accounts ou forcer un re-set)
- [ ] Mapper le RBAC (`guest` / `member` / `mod` / `admin`) sur les rôles BetterAuth (additionalField `role` déjà en place)
- [ ] Migrer le flow login email/password
- [ ] Migrer le flow signup (avec approval queue existante)
- [ ] Migrer TOTP admin (`ADMIN_DASH_TOTP_SECRET` → BetterAuth TOTP plugin)
- [ ] Migrer WebAuthn / YubiKey (BetterAuth passkey plugin)
- [ ] Migrer rate limiting login
- [ ] Migrer cookies session (admin 8h, membre)
- [ ] Retirer `lib/server/auth.ts`, `session.ts`, `ratelimit.ts`
- [ ] Tests bout-en-bout des flows auth

### Phase 3 — Basculer Caddy vers SvelteKit ✅ (code fait 2026-06-12, deploy restant)
**Objectif** : SvelteKit seul derrière Caddy, `crates/web/` retiré du build.

- [x] Parité routes vérifiée (audit appelants) :
  - `/` home Rust = page login Svelte (déjà portée) ; `/portal` → redirect 301 `/` (info invitation intégrée au login) ; `/dashboard` decoy → 404 natif SvelteKit
  - assets `hero/`/`tuto/` : consommés uniquement par la page friend **Rust** — la version Svelte ne les utilise pas → abandonnés
  - login admin : réécrit en formulaire (pseudo+seed+mdp+2FA+challenge 🔒) — **YubiKey retirée** (WebAuthn vivait dans crates/web), reviendra via plugin passkey BetterAuth
  - supprimés : proxys webauthn, `/portal/login` (multi-étapes abandonné)
- [x] Service `web` retiré de `docker-compose.yml` + `Dockerfile.web` supprimé
- [x] Caddy : tout → frontend `:4173` (template + env example)
- [x] `DEPLOY.md`, `auth/CLAUDE.md`, `CLAUDE.md` racine mis à jour
- [ ] **Déploiement VPS + tests prod** (checklist dans `migration-tests-todo.md` — penser à `FRONTEND_UPSTREAM` dans le caddy env + TOTP admin en prod)

### Phase 4 — Retirer tout le Rust ✅ (2026-06-14)
**Objectif** : SvelteKit gère aussi l'API JWT pour SongSurf, plus aucun Rust.

- [x] JWT handoff déjà en SvelteKit (`lib/server/songsurf.ts`, HS256 `AUTH_JWT_SECRET`) : login membre, accès admin, logs admin
- [x] Retirer `crates/` (api + web), `Cargo.toml`, `Cargo.lock`, `Dockerfile.api`
- [x] Retirer le service `api` de `docker-compose.yml` + bloc API du Caddyfile + job `api` du CI + `web` du deploy.yml
- [x] Retirer `api-health.ts` + champ `api_ok` de `/status` (orphelin, aucun consommateur UI)
- [x] Nettoyer scripts (devtools/stop/launch/preflight/snapshot/gen_secret), Makefile, `.env.example`, docs (CLAUDE.md, README)
- [x] `npm run check` 0 erreur · vitest 26/26

> Note 2026-06-14 : le SongSurf Watcher valide déjà le JWT SvelteKit (même secret/algorithme que l'ancien Rust) — aucun changement côté NAS requis. Le code Rust reste consultable dans l'historique git.

---

## État actuel détaillé

### Frontend SvelteKit (`frontend/src/routes/`) — existant
```
/                       → redirect login/dashboard selon rôle
/portal                 → page publique
/auth/login             → login
/auth/logout
/auth/password-check
/home/friend            → zone membre
/japprends/*            → admin (login, dashboard)
/members/*              → membre
/signup
/status
/users
```

### Crates Rust restantes
- `crates/api/` : **conservé** (JWT pour SongSurf) — Phase 4
- `crates/web/` : **à retirer** progressivement — Phases 1 → 3

---

## Convention de mise à jour

- Cocher `[x]` une case quand l'étape est **testée et mergée** sur `dev/svelte`
- Ajouter une ligne `> Note YYYY-MM-DD : ...` sous une case si décision particulière
- Mettre à jour la date en haut à chaque modification
- Quand une phase est complète, ajouter un commit `docs(migration): phase N done`

### Handoff de tests

À chaque tâche réalisée, **réécrire** `docs/migration-tests-todo.md` avec :
- ce qui vient d'être fait (fichiers créés/modifiés)
- les pré-requis pour tester (serveurs à lancer, env)
- les tests précis à exécuter (curl + attendu)
- la validation type-check

Ce fichier sert à un autre Claude (ou à toi-même) pour valider la tâche sans relire toute la conversation.

---

## Point de reprise — prochaine session

**État au 2026-06-12 (fin session)** :

### Fait dans cette session (2026-06-12)
1. **Script de migration des comptes** : `frontend/scripts/migrate-web-users-to-ba.mjs`
   - `web_users` → `ba_users` + `ba_accounts` (provider `credential`), idempotent, `--dry-run`
   - username = pseudo lowercase, display_username = pseudo d'origine, email synthétique `@local.invalid`
   - hash vide (mdp retiré) → pas de credential ; hash non-Argon2 → copié + warning
   - Usage : `node --env-file=.env scripts/migrate-web-users-to-ba.mjs [--dry-run]`
2. **Argon2 branché dans BetterAuth** (`auth-v2.ts`) : `password.hash`/`password.verify` custom
   - verify : Argon2 si préfixe `$argon2`, sinon fallback scrypt BetterAuth ; hash corrompu → false (401, pas 500)
   - hash : les **nouveaux** mots de passe restent en Argon2 → compatibles crates/api et auth.ts pendant la coexistence
3. **Testé end-to-end en local** : login compte migré 200 + session, mauvais mdp 401, hash corrompu 401, sans credential 401, rate limit sign-in actif (429), sign-up → hash Argon2 en DB — voir `migration-tests-todo.md`

### Sessions précédentes (2026-06-10)
- Phase 1 validée (34+11 PASS) et commitée en 13 commits ; audit `docs/audit-migration-svelte-2026-06-10.md` traité ; CI `ci.yml` en place ; fondations BetterAuth (tables `ba_*`, plugin username, handler `/api/auth/[...all]`)

### À faire en premier au démarrage prochain
1. Lire `docs/migration-tests-todo.md` — tests restants (navigateur + checklist deploy VPS)
2. Au prochain deploy : nettoyer `SONGSURF_JWT_SECRET` des `.env`/`.secrets` (VPS + local), vérifier `ORIGIN` et le healthcheck frontend, **exécuter le script de migration sur la DB VPS** (dry-run d'abord)
3. Continuer **Phase 2** : mapper le RBAC (lecture `ba_users.role` dans les guards), puis migrer le flow login membre (`/auth/password-check` → BetterAuth sign-in) et la session serveur (`rev0auth_member_session` → `ba_sessions`)

### Phase 2 — étapes restantes
- [x] Fondations BetterAuth (tables `ba_*`, plugin username, handler)
- [x] Script migration comptes + hashes Argon2
- [x] Flow login membre sur BetterAuth (2026-06-12) : `hooks.server.ts` lit `ba_sessions`, `password-check` délègue à `signInUsername` + provisionnement paresseux, logout révoque
- [x] Synchronisation `web_users` → `ba_*` : helper `lib/server/ba-sync.ts` branché sur tous les chemins credentials/rôle/suppression (signup, mdp membre+onboarding, set/remove-password admin, create/delete user, role PUT, delete account membre)
- [x] RBAC : `locals.memberSession.role` exposé depuis `ba_users.role` (guards à densifier quand des routes mod/admin membres apparaîtront)
- [x] Cleanup code mort post-bascule (2026-06-12) : TDD dashboard + `web_test_runs`, endpoints parité sans appelant (`/users`, pings, `auth-check`, `/status/all`, `/status/set-*`), `session.ts` admin-only, deps npm mortes (bits-ui, happy-dom, adapter-auto)
- [ ] Passkeys via plugin BetterAuth (remplace le WebAuthn skippé en Phase 1)
- [ ] Auth admin : reportée — le dashboard admin sera externalisé (container dédié Tailscale-only), voir décision 2026-06-12
- [ ] UI manquantes côté Svelte à trancher/porter : suppression de compte membre (`/members/account` DELETE existe), donations crypto (`crypto-addresses` existe)

### ⚠️ Phase 3 — le switch Caddy est LA bascule effective
Le Caddyfile prod ne route vers SvelteKit que `/japprends/*` + `/_app/*`. Le flow membre BetterAuth (et tout le travail Phase 1/2 côté membres) ne sera **live qu'après extension du routage Caddy** vers les paths membres/login/signup. À faire avec le portage de `/portal` et `/`.

### Décisions verrouillées à ne pas remettre en cause
- Drizzle reste (pas Prisma)
- BetterAuth pour Phase 2
- Audit en DB (table `web_audit_log`), pas en mémoire
- `/members/dashboard` = redirect, pas page séparée
- Endpoints SvelteKit "REST" (query params) plutôt que paths-Rust quand existant : on garde le shape SvelteKit, les pages migrées appelleront ces URLs
- WebAuthn registration : skip Phase 1, à refaire via BetterAuth passkey plugin en Phase 2
- Assets `hero/`/`tuto/` : différés jusqu'à migration des pages qui les consomment
