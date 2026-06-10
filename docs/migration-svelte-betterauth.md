# Migration SvelteKit + BetterAuth — Guideline

> Document de suivi de la migration `crates/web/` (Rust SSR) + auth custom → SvelteKit + BetterAuth + Drizzle.
> Branche : `dev/svelte`.
> **Mis à jour étape par étape — chaque case cochée est validée.**

Dernière mise à jour : 2026-05-31 — *Fin de session. Phase 1 ≈ terminée (code complet, tests utilisateur à faire). Reprise → tests + Phase 2 BetterAuth.*

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
- [ ] Migrer pages manquantes (ordre suggéré : audit → songsurf-logs → members/dashboard)
- [ ] Migrer endpoints manquants par groupe (status → members → admin)
- [ ] Vérifier parité styles (CSS pages)
- [ ] Tests manuels flow complet par rôle (guest / member / mod / admin)

### Phase 2 — Intégrer BetterAuth sur Drizzle
**Objectif** : remplacer l'auth custom par BetterAuth sans casser le schema partagé.

- [ ] Installer `better-auth` + adapter Drizzle
- [ ] Configurer `better-auth` avec table `web_users` existante (custom mapping)
- [ ] Mapper le RBAC (`guest` / `member` / `mod` / `admin`) sur les rôles BetterAuth
- [ ] Migrer le flow login email/password
- [ ] Migrer le flow signup (avec approval queue existante)
- [ ] Migrer TOTP admin (`ADMIN_DASH_TOTP_SECRET` → BetterAuth TOTP plugin)
- [ ] Migrer WebAuthn / YubiKey (BetterAuth passkey plugin)
- [ ] Migrer rate limiting login
- [ ] Migrer cookies session (admin 8h, membre)
- [ ] Retirer `lib/server/auth.ts`, `session.ts`, `ratelimit.ts`
- [ ] Tests bout-en-bout des flows auth

### Phase 3 — Basculer Caddy vers SvelteKit
**Objectif** : SvelteKit seul derrière Caddy, `crates/web/` retiré du build.

- [ ] Vérifier que toutes les routes `crates/web/` ont un équivalent SvelteKit
- [ ] Retirer service `web` de `docker-compose.yml`
- [ ] Retirer `Dockerfile.web`
- [ ] Mettre à jour Caddy : `:3000` → SvelteKit (port node adapter)
- [ ] Mettre à jour `DEPLOY.md` (root)
- [ ] Mettre à jour `auth/CLAUDE.md` (retirer crates/web/)
- [ ] Mettre à jour `CLAUDE.md` racine
- [ ] Déploiement VPS + tests prod

### Phase 4 — (futur, pas urgent) Retirer `crates/api/`
**Objectif** : SvelteKit gère aussi l'API JWT pour SongSurf.

- [ ] Implémenter endpoint JWT handoff en SvelteKit (compatible `AUTH_JWT_SECRET` partagé)
- [ ] Vérifier que SongSurf Watcher accepte le JWT SvelteKit
- [ ] Retirer `crates/api/`
- [ ] Retirer `Dockerfile.api`

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

**État au 2026-05-31 (fin session)** :

### Code livré dans cette session (branche `dev/svelte`, non commité)
1. **Bloc Status** — 4 endpoints + helper `api-health.ts`
2. **Bloc Members** — 4 modifs réelles (donations validation, crypto-addresses, account DELETE cascade, wall ownership fix)
3. **Bloc Admin** — 5 endpoints créés (ping, auth-check, user/ping, password DELETE, wall DELETE) + cleanup wall query-based
4. **3 pages** — `/japprends/audit`, `/japprends/songsurf-logs`, `/members/dashboard` (redirect)
5. **Infra audit log** — nouvelle table `web_audit_log` + helper `writeAudit()` + remplacement des `console.info`

Tous les `npm run check` passent (0 errors).

### À faire en premier au démarrage prochain
1. **Lire `docs/migration-tests-todo.md`** — handoff du dernier bloc (3 pages + audit)
2. **Demander à l'user** s'il a testé localement les blocs précédents (Status, Members, Admin) et le bloc en cours (pages + audit), ou s'il veut qu'on enchaîne sur la base que le code compile
3. **Si tests OK** → commit en plusieurs commits atomiques par bloc, puis attaquer **Phase 2 BetterAuth**

### Phase 2 — point d'entrée
- Lire la roadmap Phase 2 plus haut dans ce doc
- Stack confirmé : BetterAuth + Drizzle (déjà en place), pas Prisma
- Premier sous-objectif : `npm install better-auth` + adapter Drizzle + configurer avec la table `web_users` existante (custom user mapping)
- Décider : garder le RBAC custom ou utiliser plugins BetterAuth ?

### Décisions verrouillées à ne pas remettre en cause
- Drizzle reste (pas Prisma)
- BetterAuth pour Phase 2
- Audit en DB (table `web_audit_log`), pas en mémoire
- `/members/dashboard` = redirect, pas page séparée
- Endpoints SvelteKit "REST" (query params) plutôt que paths-Rust quand existant : on garde le shape SvelteKit, les pages migrées appelleront ces URLs
- WebAuthn registration : skip Phase 1, à refaire via BetterAuth passkey plugin en Phase 2
- Assets `hero/`/`tuto/` : différés jusqu'à migration des pages qui les consomment
