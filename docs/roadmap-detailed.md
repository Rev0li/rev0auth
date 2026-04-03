# Roadmap Detaillée - Jour par Jour (à partir d'aujourd'hui)

## Vue d'ensemble (4 semaines)

```
Jour 1-2:   Tests unitaires + RBAC middleware
Jour 3-4:   Hardening securite + cookies
Jour 5-6:   Tests d'integration DB
Jour 7-8:   Déploiement VPS + DuckDNS + reverse proxy
Jour 9-10:  Frontend publique (portfolio/CV)
Jour 11-12: Zone privée membres
Jour 13-14: Media streaming (prêt NAS)
```

## Docs vibe et suivi

- [FunFront](FunFront.md)
- [Outtime Documentation](Outtime.md)
- [Nest-Work - Consolidation de base](Nest-Work.md)

---

## Tickets precedents (deja realises)

- AUTH-001 - Signup: DONE
  - Route `POST /auth/signup`
  - Hash mot de passe Argon2id
  - Gestion duplicate email (409)

- AUTH-002 - Login: DONE
  - Route `POST /auth/login`
  - Verification mot de passe Argon2id
  - Emission access token + refresh token

- AUTH-003 - Refresh: DONE
  - Route `POST /auth/refresh`
  - Rotation refresh token
  - Ancien refresh token invalide apres rotation

- AUTH-004 - Documentation livre: DONE
  - Documentation pedagogique des modules auth
  - Guide de lecture et validation pipeline

- AUTH-005 - PostgreSQL + SQLx: DONE
  - Backend dual memoire/PostgreSQL via `DATABASE_URL`
  - Initialisation schema auth en base
  - Tests existants conserves en mode memoire

---

## JOUR 1 - Tests Unitaires Module Auth

**Durée**: 8 heures

### 08h00 - 09h30: Setup test framework & password tests (1.5h)

**Objectif**: Setup fixtures et tester password.rs

```bash
# Commandes a executer
~/.cargo/bin/cargo test --lib auth::password --
```

**Tests a écrire** (dans password.rs):
- `test_hash_password_produces_valid_argon2()` - format valide
- `test_hash_password_repeats_differently()` - salts diff
- `test_verify_password_with_correct_password()` - OK
- `test_verify_password_with_wrong_password()` - FAIL
- `test_verify_password_with_corrupted_hash()` - FAIL graceful

**Ticket**: AUTH-006-TEST-PWD: DONE

---

### 09h30 - 11h00: JWT token tests (1.5h)

**Objectif**: Tester jwt.rs complet

**Tests a écrire**:
- `test_token_service_from_env_uses_secret()` - load from env
- `test_issue_access_token_has_correct_claims()` - sub, email, role
- `test_issue_access_token_expiration()` - iat + 15min
- `test_issue_access_token_serializes_to_jwt()` - format: H.P.S

**Ticket**: AUTH-006-TEST-JWT: DONE

---

### 11h00 - 12h30: Store memory tests (1.5h)

**Objectif**: Tester toutes les fonctions store.rs (mode memoire)

**Tests a écrire**:
- `test_create_user_in_memory()` - save + get back
- `test_create_user_duplicate_email_rejected()` - 409
- `test_find_user_returns_none_if_missing()` - None
- `test_normalize_email_trims_and_lowercases()` - @Example.COM -> @example.com
- `test_issue_refresh_token_generates_unique()` - 64 alphanum
- `test_rotate_refresh_deletes_old_token()` - old invalid
- `test_rotate_refresh_issued_new_token()` - new valid

**Ticket**: AUTH-006-TEST-STORE-MEM: DONE

---

### 12h30 - 13h30: Lunch

---

### 13h30 - 15h00: Handler validation tests (1.5h)

**Objectif**: Tester edge cases signup/login/refresh

**Tests a écrire**:
- `test_signup_empty_email()` - 400
- `test_signup_weak_password_9_chars()` - 400
- `test_login_nonexistent_user()` - 401
- `test_login_wrong_password()` - 401
- `test_refresh_invalid_token_format()` - 401
- `test_refresh_expired_token()` - 401 (mock time)

**Usable helper**: fixture `create_test_app()` avec in-memory

**Ticket**: AUTH-006-TEST-HANDLERS  DONE

---

### 15h00 - 16h30: Test summary & report (1.5h)

**Deliverable**:
- Tous les tests passent
- Coverage report (ajuster si <80%)
- Documentation: docs/auth-006-tests.md

```bash
~/.cargo/bin/cargo test --lib auth:: -- --nocapture
~/.cargo/bin/cargo test --lib auth:: -- --test-threads=1
```

**Recap**: 12+ tests unitaires nouveaux, tous GREEN  

---   DONE

## JOUR 2 - Middleware RBAC + Middleware Extractor JWT

**Durée**: 8 heures

### 08h00 - 10h00: JWT Extractor middleware (2h)

**Objectif**: Middleware qui decode JWT + extrait claims

**Code a écrire** (nouveau fichier `crates/api/src/auth/extractor.rs`):
- `struct UserClaims { id, email, role }`
- `impl FromRequestParts for UserClaims` (Axum extractor)
- Decode JWT from Authorization header
- Return 401 si invalid/expired

```rust
// Exemple utilisation route
async fn my_protected_route(
    UserClaims { id, email, role }: UserClaims
) -> Json<...> { ... }
```

**Tests**:
- `test_extract_valid_jwt()` - SUCCESS
- `test_extract_missing_header()` - 401
- `test_extract_invalid_signature()` - 401
- `test_extract_expired_token()` - 401

**Ticket**: AUTH-006-RBAC-EXTRACTOR   DONE

---

### 10h00 - 12h00: RBAC Role middleware (2h)

**Objectif**: Middleware qui check role

**Code a écrire** (nouveau fichier `crates/api/src/auth/rbac.rs`):
- `enum RequiredRole { Member, Admin }`
- `struct RoleGuard(RequiredRole)`
- Implement `FromRequestParts` pour role check
- Return 403 si role insufficient

```rust
// Exemple utilisation
async fn admin_only_route(
    RoleGuard(RequiredRole::Admin): RoleGuard,
    claims: UserClaims
) -> Json<...> { ... }
```

**Tests**:
- `test_member_can_access_member_route()` - OK
- `test_member_cannot_access_admin_route()` - 403
- `test_admin_can_access_everything()` - OK

**Ticket**: AUTH-006-RBAC-GUARD   DONE

---

### 12h00 - 13h00: Lunch

---

### 13h00 - 15h00: Protected routes + tests (2h)

**Objectif**: Ajouter exemple routes protegees avec RBAC

**Routes a ajouter** (dans app.rs):
- `GET /members/profile` (requires Member)
- `GET /admin/users` (requires Admin)

**Tests**:
- Full pipeline: signup -> login -> access /members/profile avec token
- Full pipeline: login as non-admin -> GET /admin/users -> 403

**Ticket**: AUTH-006-RBAC-ROUTES    DONE

---

### 15h00 - 16h30: Documentation RBAC (1.5h)

**Deliverable**: docs/auth-006-rbac.md
- Exemple intégration
- Diagramme flux JWT
- RBAC matrix

---   DONE

## JOUR 3 - Hardening Securité (Cookies, CSRF, Rate-limit)

**Durée**: 8 heures

### 08h00 - 10h00: Cookies Secure (2h)

**Objectif**: Remplacer bearer token par cookies HttpOnly+Secure

Code a écrire (nouveau fichier `crates/api/src/auth/cookies.rs`):
- `extract_cookie(cookie_name)` -> Option<String>
- Set-Cookie response avec flags: HttpOnly, Secure, SameSite=Lax
- Path=/; Max-Age=900 for access_token

Routes impactees:
- `/auth/login` - set cookies au lieu de JSON tokens
- `/auth/refresh` - set new cookie

**Tests**:
- `test_login_sets_secure_cookie()` - HttpOnly=true
- `test_refresh_rotates_cookie()` - new Max-Age
- `test_csrf_token_generated()` - CSRF token pour POST

**Ticket**: AUTH-007-COOKIES   DONE

---

### 10h00 - 12h00: CSRF Protection (2h)

**Objectif**: Token CSRF pour toute mutation (signup, login, etc)

Code a écrire (dans handlers.rs):
- Validator CSRF: extracte token de header `X-CSRF-Token`
- Store refresh tokens avec CSRF token associe
- Verify CSRF sur POST /auth/signup, /auth/login

**Exemple flow**:
1. GET /csrf -> { csrf_token: "abc123..." }
2. POST /auth/signup + header X-CSRF-Token: abc123

**Tests**:
- `test_signup_without_csrf_token()` - 403
- `test_signup_with_invalid_csrf()` - 403
- `test_signup_with_valid_csrf()` - 200

**Ticket**: AUTH-007-CSRF    DONE

---

### 12h00 - 13h00: Lunch

---

### 13h00 - 15h00: Rate Limit Login (2h)

**Objectif**: Limiter tentatives login bruteforce

Dependencies a ajouter:
- tower-governor (rate limiter)

Code a écrire (nouveau middleware):
- Track echecs login par email: 5 echecs max / 15min
- Lockout progressif: 15min, 1h, 24h
- Return 429 Too Many Requests si lockout

**Audit log**:
- Log chaque tentative login (success + failure)

**Tests**:
- `test_5_failed_logins_locks_for_15min()` - 429
- `test_successful_login_resets_counter()` - OK

**Ticket**: AUTH-007-RATE-LIMIT   DONE

---

### 15h00 - 16h30: Audit Logs (1.5h)

**Objectif**: Tracer auth events (signup, login, refresh, failed_login)

Code a écrire (nouveau module `auth/audit.rs`):
- Struct `AuditEvent { timestamp, user_id, event_type, ip }`
- Log dalam store (pour now: just tracing::info!)
- Query `/admin/audit-logs` protected

**Tests**:
- `test_signup_creates_audit_event()` - audit.create_user
- `test_failed_login_logged()` - audit.failed_auth

**Ticket**: AUTH-007-AUDIT   DONE

---

## JOUR 4 - Tests d'Integration Database

**Durée**: 8 heures

### 08h00 - 10h00: Docker Postgres Test Container (2h)

**Objectif**: Setup testcontainers pour tests DB

Dependencies:
- testcontainers = "0.16"
- tokio-postgres ou sqlx

Code a écrire (tests/integration_tests.rs):
- Fixture `with_test_db()` qui lance container Postgres
- Auto cleanup apres test
- Reset DB state

**Tests**:
- `test_creates_tables_if_not_exist()` - schema init
- `test_user_creation_persisted()` - save -> select -> found

**Ticket**: AUTH-008-TESTDB   DONE

---

### 10h00 - 12h00: Full auth flow with DB (2h)

**Objectif**: Chaîne complète signup -> login -> refresh en DB

Tests a écrire (dans tests/):
- `test_signup_saves_to_db()` - insert check
- `test_login_finds_user_from_db()` - select check
- `test_refresh_rotation_transactional()` - delete old + insert new

**Ticket**: AUTH-008-FLOW-DB    DONE

---

### 12h00 - 13h00: Lunch

---

### 13h00 - 15h00: Migrations & Schema versioning (2h)

**Objectif**: Structurer migrations SQL proprement

File structure:
```
migrations/
├── 0001_auth_schema.sql (initial)
├── 0002_audit_logs_table.sql (new)
└── 0003_indexes_optimization.sql (new)
```

Code:
- Define struct `Migration { version, name, sql }`
- Implement `run_pending_migrations(pool)`
- Track executed migrations dans table `_migrations`

**Tests**:
- `test_migration_001_creates_users_table()` - schema check
- `test_migrations_idempotent()` - run twice = same result
- `test_audit_logs_migration_adds_table()` - via schema versions

**Ticket**: AUTH-008-MIGRATIONS    DONE

---

### 15h00 - 16h30: Perf tests & cleanup (1.5h)

**Objectif**: Verifier perfs et stabilité

Tests:
- `test_50_concurrent_logins()` - benchmark pragmatique
- `test_memory_leak_after_1000_refreshes()` - steady state

Deliverable:
- Benchmark report: docs/perf-baseline.md

**Ticket**: AUTH-008-PERF   DONE

---

## JOUR 5 - Deployment VPS Skeleton

**Durée**: 8 heures

### 08h00 - 10h00: VPS Setup (Hardening + Docker) (2h)

**Deliverable**: scripts/setup-vps.sh

Checklist:
- [ ] UFW firewall (22, 80, 443 only)
- [ ] Fail2ban (protect SSH)
- [ ] Auto updates enabled
- [ ] Docker + Docker Compose installed
- [ ] Non-root user with sudoers

**Infrastructure as Code** (docker-compose.yml for API):
```yaml
services:
  postgres:
    image: postgres:16
    environment:
      POSTGRES_PASSWORD: <from vault>
  api:
    build: crates/api
    depends_on: [postgres]
    environment:
      DATABASE_URL: postgres://...
      AUTH_JWT_SECRET: <from vault>
    ports: ["127.0.0.1:8080:8080"]
```

**Ticket**: AUTH-009-VPS-SETUP   DONE

---

### 10h00 - 12h00: DuckDNS + Reverse Proxy (Caddy) (2h)

**Objective**: Public HTTPS domain pointing to API

Caddy config (caddyfile):
```caddy
api.revoli.duckdns.org {
  reverse_proxy localhost:8080
}
```

Setup:
- [x] DuckDNS token stored securely
- [x] Caddy auto-renew HTTPS (ACME)
- [x] health check endpoint `/health`

**Ticket**: AUTH-009-DUCKDNS   DONE

---

### 12h00 - 13h00: Lunch

---

### 13h00 - 15h00: Secrets Management (2h)

**Objective**: Handle AUTH_JWT_SECRET, DATABASE_URL securely

Options:
1. `.env` file (dev only, never commit)
2. K8s Secrets (if using k8s)
3. Hashicorp Vault (recommended for prod)
4. AWS SSM Parameter Store (if on AWS)

For now, setup `.env.example` template:
```
AUTH_JWT_SECRET=<generate 32 bytes>
DATABASE_URL=postgres://postgres:PASSWORD@db:5432/rev0auth
```

Validation:
- Startup fails if critical env vars missing
- Logging shows which backend (memory vs postgres)

**Tests**:
- `test_startup_fails_without_jwt_secret()` (mock behavior)
- `test_database_url_optional()` - graceful fallback

Setup:
- [x] startup bloque sans `AUTH_JWT_SECRET`
- [x] fallback memory sans `DATABASE_URL`
- [x] template `.env.example` maintenu

**Ticket**: AUTH-009-SECRETS DONE

---

### 15h00 - 16h30: Health checks & Rolling restart (1.5h)

**Objective**: Deployable without downtime

Endpoints:
- `GET /health` -> { status: ok }
- `GET /health/db` -> { status: ok, latency_ms: 5 }

Docker Compose healthcheck:
```yaml
healthcheck:
  test: ["CMD", "curl", "-f", "http://localhost:8080/health"]
  interval: 30s
  timeout: 10s
  retries: 3
```

Setup:
- [x] endpoint `GET /health` en place
- [x] endpoint `GET /health/db` en place
- [x] docker compose healthcheck API en place

**Ticket**: AUTH-009-HEALTH-CHECKS   DONE

---

## JOUR 6-7 - Frontend Public (Portfolio + CV)

**Durée**: 16 heures (2 jours)

### JOUR 6 - Setup Web Framework

**Objective**: Frontend public pour portfolio + contact

Stack proposé: Rust + Askama templates ou Leptos (SSR)

Routes:
- `GET /` -> landing page
- `GET /portfolio` -> list projects
- `GET /cv` -> PDF download
- `POST /contact` -> contact form (anti-spam)

**Deliverable**: crates/web avec routes

**Ticket**: AUTH-010-WEB-PUBLIC

---

### JOUR 7 - Member Zone

**Objective**: Zone privée accessible après login

Routes:
- `GET /members/dashboard` (requires Member role)
- `GET /members/profile` - edit profile
- `GET /members/profile/data` - load profile data
- `PUT /members/profile/data` - save profile data
- `POST /members/avatar` - upload avatar

**Ticket**: AUTH-011-MEMBERS-ZONE

---

## JOUR 8 - Consolidation Technique (ticket par ticket)

**Durée**: 8 heures

**Objectif**: consolider le socle avant les tests finaux, en decoupant le projet en pieces plus petites et plus modulables.

### 08h00 - 09h00: Audit des gros blocs (1h)

**Ticket**: NEST-001-AUDIT-SOCLE

**Status**: DONE

**Rapport**: `docs/nest-001-audit-backend.md`

**Objectif**: identifier ce qui est trop gros, trop melange, ou trop dur a reprendre.

**Actions**:
- lister les fichiers HTML, CSS, JS et backend les plus charges;
- repérer les duplications et frontieres floues;
- noter les modules a extraire en premier;
- marquer les dependances qui empechent le decoupage.

**Livrable**:
- mini inventaire de consolidation avec priorites.

---

### 09h00 - 10h30: Refactor HTML (1h30)

**Ticket**: NEST-002-HTML-MODULARITY

**Status**: DEFERRED (frontend reset complet plus tard)

**Objectif**: rendre le HTML plus simple, plus lisible et plus reutilisable.

**Actions**:
- extraire les sections repetitives en blocs clairs;
- reduire les templates monolithiques;
- isoler les zones communes;
- garder une structure par page avec une seule intention principale.

**Tests/Validation**:
- verifier que les pages gardent le meme rendu;
- confirmer que les sections importantes restent accessibles et previsibles.

---

### 10h30 - 12h00: Refactor CSS (1h30)

**Ticket**: NEST-003-CSS-LAYERS

**Objectif**: reorganiser les styles par couches au lieu d'accumuler les regles.

**Actions**:
- separer tokens, layout, composants et etats;
- supprimer les doublons et overrides fragiles;
- clarifier les noms de classes;
- isoler le responsive si besoin.

**Livrable**:
- structure CSS plus petite, plus stable, plus previsible.

---

### 13h00 - 14h30: Refactor JavaScript (1h30)

**Ticket**: NEST-004-JS-MODULES

**Objectif**: decouper le JavaScript en petites pieces orientees action.

**Actions**:
- sortir l'initialisation du reste de la logique;
- separer event listeners, state et services;
- reduire les effets de bord;
- nommer les fonctions par intention, pas par hasard.

**Cible**:
- un point d'entree clair;
- des modules petits et remplaçables;
- moins de gros fichiers utilitaires fourre-tout.

---

### 14h30 - 15h30: Consolidation backend + tests (1h)

**Tickets**: NEST-005-BACKEND-SPLIT, NEST-006-TEST-RESET

**Objectif**: aligner le backend et les tests sur la nouvelle modularite.

**Actions backend**:
- reduire les routes trop chargees;
- isoler la logique metier dans des services plus petits;
- garder la validation proche des entrees;
- harmoniser les erreurs.

**Actions tests**:
- regrouper les tests par famille (`unit`, `integration`, `perf`);
- executer les tests rapides en parallelisation plus forte;
- isoler les tests lourds et la charge;
- eviter de melanger les cas de nature differente.

**Validation**:
- verifier que le refactor ne casse pas les comportements existants.

---

### 15h30 - 16h30: Doc et checklist finale consolidation (1h)

**Ticket**: NEST-007-DOC-CHECK

**Objectif**: laisser une trace claire de la consolidation pour la suite.

**Actions**:
- resumer les fichiers touches;
- noter les decisions de coupe et de modularisation;
- pointer les prochains refactors si besoin;
- lier ce plan a `docs/Nest-Work.md`.

**Livrable**:
- rapport court de consolidation + checklist de reprise.

---

## JOUR 8-9 - Media Streaming (Ready NAS)

**Objective**: Intégration DldeMedia + NAS via Tailscale

Cette partie necessite:
- Metadata DB pour videos
- URL signing (court TTL, 30min)
- Acces NAS via Tailscale (privé)

**Ticket**: AUTH-012-MEDIA

---

## JOUR 10 - Tests e2e + Stress

**Durée**: 8 heures

### Full pipeline test

```bash
# Setup
1. Clean DB
2. Start docker-compose
3. Wait for health checks

# Scenario 1: Public access
GET / -> 200
GET /portfolio -> 200
POST /contact { name, email, msg } -> 200 (rate limited)

# Scenario 2: Auth flow
POST /auth/signup -> 200 + set cookies
GET /members/dashboard (avec auth cookie) -> 200
POST /auth/refresh -> 200 + new cookie

# Scenario 3: Admin
POST /admin/users -> 403 (member not admin)
CREATE ADMIN user via SQL
GET /admin/users (admin token) -> 200

# Scenario 4: Stress
100 concurrent signups
1000 concurrent logins
Refresh rotations in parallel

# Cleanup
Docker stop
```

---

## Resume Timeline

| Jour | Focus | Heures | Status |
|------|-------|--------|--------|
| 1 | Auth tests unitaires | 8 | ✅ DONE |
| 2 | RBAC + JWT Extractor | 8 | ✅ DONE |
| 3 | Securité (cookies, CSRF, rate-limit) | 8 | ✅ DONE |
| 4 | Tests DB integration | 8 | ✅ DONE |
| 5 | VPS + Docker + DuckDNS | 8 | ✅ DONE |
| 6 | Frontend public (portfolio) | 8 | ✅ DONE |
| 7 | Zone privée membres | 8 | ✅ DONE |
| 8-9 | Media streaming (NAS) | 16 | 📋 TODO |
| 10 | E2E + Stress tests | 8 | 📋 TODO |

**Total**: 80 heures (10 jours) pour une plateforme complète, sécurisée, déployable sur VPS.

---

## Priorités par besoin

### MVP Min (Jour 1-3) = 24h
- Auth complet testée
- RBAC + routes protegees
- Cookies sécurisés

### Deployable (Jour 1-5) = 40h
- + VPS ready + DuckDNS + reverse proxy

### Produit Final (Jour 1-10) = 80h
- + Frontend public + zone privée + media
