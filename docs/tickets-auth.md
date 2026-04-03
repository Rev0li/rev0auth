# Tickets - Module Auth (progressif)

## Ticket AUTH-001 - Signup

Statut: DONE

Objectif:
- Exposer `POST /auth/signup`
- Creer un utilisateur avec mot de passe hash Argon2id
- Retourner access token + refresh token

Definition of done:
- Validation email/password
- Rejet duplicate email (409)
- Test route signup OK

## Ticket AUTH-002 - Login

Statut: DONE

Objectif:
- Exposer `POST /auth/login`
- Verifier mot de passe Argon2id
- Retourner nouveaux tokens

Definition of done:
- Login invalide -> 401
- Login valide -> 200
- Test route login OK

## Ticket AUTH-003 - Refresh

Statut: DONE

Objectif:
- Exposer `POST /auth/refresh`
- Rotation refresh token (ancien invalide)
- Retourner nouveau access token + nouveau refresh token

Definition of done:
- Token expire/invalide -> 401
- Rotation effective
- Test chaine signup -> login -> refresh OK

## Ticket AUTH-004 - Documentation "livre"

Statut: DONE

Objectif:
- Documenter architecture, decisions, pipeline test
- Donner une lecture pedagogique du code Rust
 - Methode TDD (bibliotheque privee)

Definition of done:
- Guide de lecture par module
- Checklists de verification
Livrables:
- un listing de toutes les nouvelles fonctions/variables.
- tests individuels documentes.
- validation de la solidite du code via documentation.
- capacite a diagnostiquer un probleme X avec rapports associes.

## Ticket AUTH-005 - PostgreSQL + SQLx

Statut: DONE

Objectif:
- Migrer la persistence auth vers PostgreSQL
- Conserver les routes HTTP existantes
- Garder un mode memoire pour les tests rapides

Definition of done:
- Backend PostgreSQL actif si `DATABASE_URL` est defini
- Tables auth creees automatiquement au demarrage
- Test de chaine existant toujours vert
- Rapport de fonctions/variables/tests disponible

## Ticket AUTH-006 - Tests + RBAC

Statut: DONE

Objectif:
- Couvrir les modules auth avec des tests unitaires et handlers
- Ajouter l'extractor JWT et les guards RBAC
- Proteger des routes membres/admin et valider les acces

Definition of done:
- Suites tests password/jwt/store/handlers en place
- Middleware extractor JWT (`UserClaims`) en place
- RBAC guard (`Member`/`Admin`) en place
- Routes protegees (`/members/profile`, `/admin/users`) testees

## Ticket AUTH-007 - Hardening securite

Statut: DONE

Objectif:
- Migrer login/refresh vers cookies HttpOnly/Secure
- Ajouter protection CSRF (double-submit)
- Ajouter rate-limit progressif sur login
- Ajouter audit des evenements auth

Definition of done:
- Cookies access/refresh/csrf poses avec flags securises
- Endpoint `/csrf` + validation `X-CSRF-Token` sur mutations
- Lockout login progressif + tests de verrouillage/reset
- Endpoint admin `/admin/audit-logs` + tests audit

## Ticket AUTH-008-TESTDB - Postgres Test Container

Statut: DONE

Objectif:
- Mettre en place des tests d'integration bases sur un conteneur Postgres ephemere
- Initialiser le schema auth automatiquement
- Isoler les tests via reset DB avant/apres execution

Definition of done:
- Fixture `with_test_db()` disponible dans `crates/api/tests/integration_tests.rs`
- `test_creates_tables_if_not_exist()` vert
- `test_user_creation_persisted()` vert

## Ticket AUTH-008-FLOW-DB - Signup/Login/Refresh en DB

Statut: DONE

Objectif:
- Valider la chaine auth contre PostgreSQL (pas seulement en memoire)
- Couvrir persistence signup, lookup login, rotation refresh transactionnelle

Definition of done:
- `test_signup_saves_to_db()` vert (insert check)
- `test_login_finds_user_from_db()` vert (select + verification hash)
- `test_refresh_rotation_transactional()` vert (old token supprime + new token insere)

## Ticket AUTH-008-MIGRATIONS - SQL versionnees

Statut: DONE

Objectif:
- Introduire une structure de migrations SQL versionnees
- Appliquer uniquement les migrations non executees
- Tracer les versions appliquees dans `_migrations`

Definition of done:
- Struct `Migration { version, name, sql }` en place
- `run_pending_migrations(pool)` applique les pending migrations dans l'ordre
- Fichiers SQL `0002_audit_logs_table.sql` et `0003_indexes_optimization.sql` ajoutes
- Tests `test_migration_001_creates_users_table()`, `test_migrations_idempotent()`, `test_audit_logs_migration_adds_table()` verts

## Ticket AUTH-008-PERF - Baseline charge/stabilite

Statut: DONE

Objectif:
- Ajouter des tests de charge auth pragmatiques et reproductibles
- Verifier l'absence d'accumulation de sessions refresh sur longue sequence

Definition of done:
- `test_50_concurrent_logins()` vert
- `test_memory_leak_after_1000_refreshes()` vert
- Baseline documentee dans `docs/perf-baseline.md`

## Ticket AUTH-009-VPS-SETUP - Script infra idempotent

Statut: DONE

Objectif:
- Livrer un setup VPS scriptable par Makefile
- Industrialiser hardening de base + runtime Docker

Definition of done:
- `scripts/setup-vps.sh` cree et executable
- `docker-compose.yml` fourni pour `postgres` + `api`
- `.env.example` fourni pour brancher les secrets
- Documentation d'usage: `docs/auth-009-vps-setup.md`

## Ticket AUTH-009-DUCKDNS - Reverse proxy HTTPS

Statut: DONE

Objectif:
- Exposer l API sur un domaine DuckDNS public
- Mettre Caddy en reverse proxy avec TLS auto-renew (ACME)

Definition of done:
- Token DuckDNS stocke de facon securisee hors repository
- Caddyfile configure pour `api.revoli.duckdns.org -> 127.0.0.1:8080`
- Endpoint `/health` verifie en HTTPS public
- Runbook documente dans `docs/auth-009-duckdns.md`

## Ticket AUTH-009-HEALTH-CHECKS - Readiness + rolling restart

Statut: DONE

Objectif:
- Exposer des endpoints de health pour orchestration
- Permettre un healthcheck compose pour redemarrage fiable

Definition of done:
- `GET /health` retourne `{ service, status }`
- `GET /health/db` retourne `{ status, backend, latency_ms }`
- `docker-compose.yml` contient un healthcheck pour le service API
- Tests app health en place (`test_health_endpoint`, `test_health_db_endpoint_memory_backend`)

## Ticket AUTH-009-SECRETS - Startup secret policy

Statut: DONE

Objectif:
- Exiger `AUTH_JWT_SECRET` au demarrage production
- Garder `DATABASE_URL` optionnel avec fallback memoire

Definition of done:
- Startup echoue si `AUTH_JWT_SECRET` manquant (`test_startup_fails_without_jwt_secret`)
- Startup reussit sans `DATABASE_URL` (`test_database_url_optional`)
- Documentation dediee: `docs/auth-009-secrets.md`

## Ticket AUTH-010-WEB-PUBLIC

Statut: DONE

Objectif:
- Livrer le socle frontend public (landing + portail)
- Exposer les ecrans de base et points d'entree web

Definition of done:
- Route `GET /` disponible (connexion)
- Route `GET /portal` disponible (inscription)
- Route `GET /dashboard` disponible
- Build web compile (`cargo check -p rev0auth-web`)

## Ticket AUTH-011-MEMBERS-ZONE

Statut: DONE

Objectif:
- Exposer une zone membre dediee apres login
- Ajouter edition de profil membre
- Ajouter upload avatar

Definition of done:
- Route `GET /members/dashboard` disponible
- Route `GET /members/profile` disponible
- Route `GET /members/profile/data` disponible
- Route `PUT /members/profile/data` disponible
- Route `POST /members/avatar` disponible (multipart)
- Build web compile (`cargo check -p rev0auth-web`)