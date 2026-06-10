# Audit — Migration Rust → SvelteKit (`auth/`)

> Date : 2026-06-10 · Branche : `dev/svelte` · Auditeur : Claude Code
> Périmètre : état de la migration Phase 1, sécurité, qualité du code, tests & déploiement.
> Référence : [`migration-svelte-betterauth.md`](migration-svelte-betterauth.md) · [`migration-tests-todo.md`](migration-tests-todo.md)

---

## Résumé exécutif

La **Phase 1 est effectivement complète au niveau code** : toutes les routes prévues existent, `svelte-check` passe (0 erreur, 22 warnings), les 13 tests unitaires vitest passent. Le code livré le 31/05 est de bonne qualité générale (guards systématiques, requêtes Drizzle paramétrées, validation des entrées sur les endpoints sensibles).

**Mais rien n'est commité depuis le 31/05** (~25 fichiers), **les 16 tests manuels du handoff n'ont pas été exécutés**, et l'audit révèle :

- **2 régressions de parité** vs Rust (cascade delete admin, casse du pseudo au portal/login)
- **4 points de sécurité** à corriger avant la bascule Phase 3 (dont 1 crash 500 sur login après remove-password)
- **2 bugs de config Docker Compose** qui rendront le monitoring et `/status` faux en prod
- **1 incohérence de secrets JWT** (`SONGSURF_JWT_SECRET` vs `AUTH_JWT_SECRET`) — piège lors d'une rotation

Aucun de ces points ne bloque le démarrage de la Phase 2 (BetterAuth), mais les fixes sécurité S1–S4 et les commits devraient passer en premier.

---

## 1. État de la migration (Phase 1)

### 1.1 Parité des routes — vérifiée fichier par fichier

Inventaire croisé : 74 routes Rust (`crates/web/src/main.rs`) vs `frontend/src/routes/`.

**✅ Migré et conforme** (guards inclus — `/user/ping` et `/status/*` sont bien admin-only des deux côtés) :
status (×5), members (account, avatar, donations + crypto-addresses, messages, password + onboarding, profile, status, wall, access/request, dashboard-redirect), japprends (audit, auth-check, donations + review, invites, login/logout, messages + reply, ping, set/remove-password, songsurf-access, songsurf-logs, tdd, users CRUD, wall), auth (logout, password-check), signup, portal/login, users, user/ping.

**⏭️ Différé volontairement (documenté, Rust les sert encore)** :
- WebAuthn register/status/remove/export → Phase 2 (plugin passkey BetterAuth)
- `/japprends/endpoints`, `/dashboard` (decoy) → skip assumé
- `/static/hero/:filename`, `/static/tuto/:filename` → à copier dans `frontend/static/` quand les pages consommatrices migreront

**⚠️ Écart non documenté** :
- **Page `GET /portal` absente côté SvelteKit.** `routes/portal/+server.ts` ne contient qu'un `POST` (refus signup public). La doc de migration liste `/portal → page publique` comme existante — c'est inexact : seul le Rust la sert. À ajouter à la checklist Phase 3 (vérification "toutes les routes ont un équivalent").

**➕ Routes SvelteKit sans équivalent Rust** (fonctionnalités nouvelles, OK) : `japprends/signup-requests/*`, `japprends/tests/history`, `japprends/dashboard` (page), `members/password/onboarding`.

### 1.2 Statut process

| Item | État |
|---|---|
| Code Phase 1 | ✅ complet |
| `npm run check` | ✅ 0 erreur / 22 warnings (conforme handoff) |
| `npm test` (vitest) | ✅ 13/13 |
| Tests manuels (16 du handoff) | ❌ **non exécutés** |
| Commits | ❌ **~25 fichiers non commités depuis le 31/05** |
| Phase 2 BetterAuth | non démarrée |

---

## 2. Sécurité

### Corrections recommandées avant Phase 3 (bascule prod)

**S1 — Crash 500 au login si mot de passe retiré** · `auth/password-check/+server.ts`, `members/password/+server.ts`
`DELETE /japprends/users/[pseudo]/password` met `passwordHash = ''`. Or `argon2.verify('', …)` **lève une exception** non catchée → 500 au lieu d'un refus propre. Fix : court-circuiter si `!user.passwordHash` (ou try/catch autour de `verifyPassword`).

**S2 — Pas de rate limiting sur le login membre** · `auth/password-check/+server.ts`
Le login admin est protégé (5 essais / 15 min) mais le login membre ne l'est pas — brute-force possible sur les mots de passe membres. Le Rust a la même lacune, mais `lib/server/ratelimit.ts` existe déjà : l'appliquer coûte 4 lignes.

**S3 — Avatars SVG servis inline → XSS stocké potentiel** · `members/avatar/+server.ts` + `[pseudo]/+server.ts`
L'upload accepte `image/svg+xml` (contenu arbitraire) et le GET le sert avec ce Content-Type sans en-tête de protection. Un membre peut uploader un SVG avec script ; un admin qui ouvre l'URL exécute le script sur l'origine (les cookies sont HttpOnly, mais les actions admin restent jouables). Même comportement côté Rust — bug commun, pas une régression. Fix minimal : retirer `image/svg+xml` de l'allowlist d'**upload** (les SVG d'avatars du signup sont générés côté serveur, donc sûrs) ou servir avec `Content-Security-Policy: sandbox`.

**S4 — Deux noms d'env pour le même secret JWT** · `songsurf-logs` utilise `SONGSURF_JWT_SECRET`, `songsurf-access` et le Rust utilisent `AUTH_JWT_SECRET`. Le `.env.example` documente "usually same value" — c'est exactement le scénario où une rotation de `AUTH_JWT_SECRET` (procédure du CLAUDE.md racine) oublie `SONGSURF_JWT_SECRET` et casse silencieusement les logs SongSurf. Recommandation : un seul nom (`AUTH_JWT_SECRET`) partout, fallback temporaire sur l'ancien.

### Points à surveiller (non bloquants)

- **JWT 8h dans l'URL** (`?token=` vers SongSurf, dans `songsurf-access` et `password-check`) : fuite possible via logs/historique. Design hérité de l'archi (handoff Tailscale), mais un TTL court (ex. 5 min, SongSurf pose ensuite son propre cookie) réduirait l'exposition. À traiter avec SongSurf, pas unilatéralement.
- **Rate limiter basé sur `x-forwarded-for`** (`ratelimit.ts`) : spoofable si le port Node est joignable sans passer par Caddy. OK tant que compose bind sur `127.0.0.1:4173` (c'est le cas) et que Caddy contrôle XFF.
- **`portal/login` énumère les comptes** (états `missing`/`inactive` distincts) et révèle `totpEnabled` pour le pseudo admin. Assumé dans le code ("safe for private app") — à réévaluer si l'app s'ouvre.
- **Comparaison des credentials admin en `!==`** (non constant-time) : faible impact réel (rate limit + honeypot + TOTP), parité avec le Rust.
- **Sessions jamais purgées** : `web_sessions` accumule les sessions expirées (filtrées en lecture mais jamais supprimées). Ajouter un `DELETE WHERE expires_at < now` périodique ou au boot — sera de toute façon remplacé par BetterAuth en Phase 2.
- **`japprends/users/[pseudo]` PUT** : passe le JSON brut à `.set()` après avoir retiré `passwordHash`/`pseudo`/`avatarBytes`. Admin-only donc acceptable, mais un body vide ou une clé inconnue → erreur Drizzle 500. Une allowlist de champs serait plus robuste.

---

## 3. Parité fonctionnelle — régressions vs Rust

**P1 — `DELETE /japprends/users/[pseudo]` ne cascade pas** (régression)
Le Rust supprime aussi messages + donations et écrit un audit `delete_user`. Le SvelteKit supprime uniquement sessions + user : messages/donations orphelins, et pas de trace d'audit.

**P2 — Casse du pseudo incohérente** (régression sur `portal/login`)
- Rust : `LOWER(pseudo) = LOWER($1)` partout.
- SvelteKit : mélange de `sql LOWER()` (password-check, signup, set-password…) et de `eq(users.pseudo, key)` avec `key` lowercasé (portal/login, avatar GET, donations, password PUT, profile…).
- Le `signup` insère le pseudo **sans le normaliser** (`Alice` reste `Alice`), alors que `japprends/users` POST lowercase. Conséquence concrète : un compte créé `Alice` via signup → `portal/login` répond "Compte introuvable".
- Fix recommandé : **normaliser en lowercase à l'insertion** (signup) + uniformiser les lookups.

**P3 — `POST /members/avatar` n'enregistre pas `avatarFilename`/`avatarSizeBytes`** (le Rust les set) — toute page qui affiche ces champs verra des valeurs nulles.

**P4 — `POST /japprends/users/[pseudo]/password` (set-password)** : pas d'audit (le Rust écrit `set_password`), et répond `ok: true` même si le pseudo n'existe pas (le Rust vérifie `rows_affected`).

**P5 — `PUT /members/password`** : pas de longueur minimale sur le nouveau mot de passe, alors que `password/onboarding` impose 8 caractères. Harmoniser.

**Bugs communs aux deux stacks** (pas des régressions, à corriger à l'occasion) :
- la suppression de compte (self et admin) laisse les `web_wall_posts` orphelins ;
- `status/all` renvoie des valeurs codées en dur (`sprint: 'AUTH-006'`, `tests_api_total: 18`) copiées du Rust.

---

## 4. Qualité du code

Globalement propre et homogène. Points d'amélioration :

1. **`requireAdmin()` dupliqué** dans 3+ fichiers, et le pattern `if (!locals.adminSession) throw error(401)` répété dans ~20 autres → centraliser dans `$lib/server/guards.ts` (`requireAdmin(locals)`, `requireMember(locals)`). Réduit le risque d'oubli sur une future route.
2. **`status/set-busy|active|inactive`** : 3 fichiers identiques à une chaîne près → factoriser (helper partagé ou route `[action]` avec allowlist).
3. **Casse pseudo** : cf. P2 — c'est autant un problème de cohérence de code que de parité.
4. **22 warnings svelte-check** (`state_referenced_locally` principalement) : connu et assumé dans le handoff, à traiter en une passe globale.
5. **`initDb()` en top-level await** dans `hooks.server.ts` : crash au boot si Postgres n'est pas prêt. OK en compose (healthcheck `depends_on`), mais en dev local le message d'erreur est cryptique. Un retry court serait plus confortable.
6. **`japprends/tests/launch`** : cwd fallback codé en dur `/home/revoli/dev/rev0auth` (ancien chemin du projet) et dépend de `cargo` — **inutilisable dans le container** node:22-slim. La feature TDD-dashboard ne fonctionnera qu'en dev local avec `CARGO_MANIFEST_DIR` correctement défini. À documenter ou à désactiver en prod.

---

## 5. Tests & déploiement

### Tests
- **Unitaires** : 13 tests sur `auth.ts` / `ratelimit.ts` / `session.ts` — passent. Aucun test d'endpoint/intégration : toute la Phase 1 (≈30 routes) repose sur les tests manuels.
- **Manuels** : les 16 tests de `migration-tests-todo.md` (audit log, songsurf-logs, redirect dashboard) **ne sont pas exécutés** — c'est le préalable au commit selon le process défini dans le doc de migration. Les blocs précédents (Status, Members, Admin) n'ont pas non plus de trace de validation.

### CI/CD
- `deploy.yml` (push sur `main`) : **aucun check ni test avant déploiement** — ni `npm run check`, ni `vitest`, ni `cargo test`. Recommandé : un job CI sur PR/push (check + vitest + cargo test) en amont du deploy.
- Rien ne tourne sur `dev/svelte` : le travail actuel n'est protégé par aucune CI.

### Docker Compose — 2 bugs probables en prod
1. **Healthcheck frontend cassé** : `wget` n'est pas installé dans l'image runtime `node:22-slim` (le `Dockerfile.web` Rust l'installe explicitement, pas `Dockerfile.frontend`). Le healthcheck échouera toujours → container `unhealthy` permanent. Vérifier avec `docker ps` sur le VPS ; fix : installer wget, ou healthcheck en `node -e "fetch(...)"`.
2. **`REV0AUTH_API_UPSTREAM` non défini pour le service frontend** → défaut `127.0.0.1:8080` *dans le container* → `checkApiUp()` échouera toujours → `/status` répondra `api_ok: false` en prod. Fix : `REV0AUTH_API_UPSTREAM: api:8080` dans le service frontend (comme le service web).

### Config
- **`ORIGIN`** : présent dans `.env.example`, absent du `.env` local. Requis par adapter-node (protection CSRF des form actions + URLs correctes). Vérifier qu'il est bien dans le `.env` du VPS.
- **Doc** : le CLAUDE.md racine annonce Caddy `:5173 → frontend` ; le compose expose `4173`. Harmoniser à la Phase 3.

---

## 6. Plan d'action recommandé (ordre)

1. **Exécuter les 16 tests manuels** de `migration-tests-todo.md` (+ smoke test des blocs Status/Members/Admin).
2. **Commiter par blocs atomiques** (status / members / admin / pages+audit / infra) — 10 jours de travail non commité, c'est le risque n°1.
3. **Fixes sécurité rapides** (1 session) : S1 (crash argon2 hash vide), S2 (rate limit password-check), S3 (retirer SVG de l'upload), P1 (cascade + audit delete_user), P2 (lowercase à l'insertion signup), P4 (audit set_password).
4. **Fixes compose** : wget/healthcheck + `REV0AUTH_API_UPSTREAM` ; vérifier `ORIGIN` sur le VPS.
5. **Unifier le secret JWT** (S4) — idéalement avant la prochaine rotation.
6. **Démarrer la Phase 2 BetterAuth** — qui remplacera de toute façon `auth.ts`/`session.ts`/`ratelimit.ts` ; ne pas sur-investir dans ces fichiers au-delà des fixes ci-dessus.
7. Ajouter un **workflow CI** (check + vitest + cargo test) avant la Phase 3.

---

*Audit réalisé en lecture seule — aucun code modifié.*
