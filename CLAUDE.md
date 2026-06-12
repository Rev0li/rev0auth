# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

rev0auth is an authentication platform with RBAC. Since Phase 3 of the SvelteKit
migration, the **SvelteKit frontend serves the whole site** (portal, members,
admin, signup) with BetterAuth member sessions:

- `frontend/` — SvelteKit (adapter-node, port 4173 in prod, 5173 in dev): all
  pages + API endpoints, Drizzle + PostgreSQL, BetterAuth (`ba_*` tables) for
  member sessions, custom admin sessions (`web_sessions`)
- `crates/api/` (`rev0auth-api`) — auth API: JWT, CSRF, refresh tokens, RBAC,
  audit logging. Scheduled for removal in Phase 4 (only `/health` is consumed)
- `crates/web/` (`rev0auth-web`) — legacy Rust SSR. **No longer deployed**
  (removed from docker-compose and Caddy in Phase 3); code kept for reference

Migration docs: `docs/migration-svelte-betterauth.md` (plan + reprise),
`docs/migration-tests-todo.md` (handoff tests, rewritten each task).

## Commands

```bash
# Frontend (the actual site)
cd frontend
npm run dev                            # Dev server (port 5173)
npm run check                          # svelte-check (expected: 0 errors)
npm test                               # vitest
node --env-file=.env scripts/migrate-web-users-to-ba.mjs --dry-run  # BetterAuth account migration

# API (Rust)
~/.cargo/bin/cargo run -p rev0auth-api
~/.cargo/bin/cargo test -p rev0auth-api
~/.cargo/bin/cargo test -p rev0auth-api -- auth::tests::my_test_name  # single test

# Database
export DATABASE_URL='postgres://postgres:postgres@localhost:5432/rev0auth'
~/.cargo/bin/cargo sqlx migrate run   # Run migrations from crates/api/migrations/
# SvelteKit-specific tables (web_sessions, web_audit_log, ba_*, songsurf_events)
# are created by initDb() at frontend startup.

# Admin 2FA
make admin-2fa-init                    # Initialize TOTP secret
make admin-otp                         # Generate current OTP
```

## Environment

Copy `.env.example` to `.env`. Required variables:
- `AUTH_JWT_SECRET` — min 32-byte secret for JWT signing
- `DATABASE_URL` — PostgreSQL connection string (API falls back to in-memory if unset)
- `ADMIN_DASH_PASSWORD`, `ADMIN_DASH_PSEUDO`, `ADMIN_DASH_SEED` — admin credentials
- `ADMIN_DASH_TOTP_SECRET` — optional Base32 TOTP secret for 2FA
- `API_BIND_ADDR` — override API port (default 8080) ; frontend port via `PORT` (4173 prod)
- `REV0AUTH_API_UPSTREAM` — frontend → API health-check target (default: 127.0.0.1:8080)
- `ORIGIN` — public origin for the SvelteKit frontend (default https://rev0li.duckdns.org in compose)
- `SONGSURF_EVENTS_SECRET` — shared secret authenticating activity events pushed by the NAS to `POST /japprends/api/songsurf-events` (SvelteKit frontend). Must be byte-identical to `SongSurf/SongSurf/.secrets`. Events land in the `songsurf_events` table (created by `initDb()`), displayed on `/japprends/songsurf-activity`.

## Architecture

### API crate (`crates/api/src/`)

```
app/
  handlers.rs    — Axum route handlers (signup, login, refresh, me, admin_panel)
  domain.rs      — Domain models and request/response types
  services.rs    — Business logic (auth service, token service)
  tests.rs       — Unit tests
auth/
  handlers.rs    — Auth-specific endpoints (CSRF, signup, login)
  jwt.rs         — Token generation and validation
  store.rs       — User and token persistence (PostgreSQL or in-memory)
  password.rs    — Argon2 password hashing
  cookies.rs     — CSRF and session cookie management
  rate_limit.rs  — Login rate limiting
  rbac.rs        — Role-based access control (guest/member/mod/admin)
  audit.rs       — Audit event logging
  extractor.rs   — Axum request extractors
  migrations.rs  — Schema migration helpers
```

**API routes**: `GET /health`, `POST /auth/signup`, `POST /auth/login`, `POST /auth/refresh`, `GET /auth/me`, `GET /admin/panel`

**Database schema**: `auth_users`, `auth_refresh_tokens` (with CSRF token column), `auth_audit_logs` — migrations in `crates/api/migrations/`.

### Frontend (`frontend/src/`)

```
hooks.server.ts            — resolves locals.adminSession (web_sessions) and
                             locals.memberSession (BetterAuth ba_sessions)
lib/server/
  auth-v2.ts               — BetterAuth instance (ba_* tables, username plugin,
                             custom Argon2 password hash/verify)
  ba-sync.ts               — web_users → ba_* sync helpers (any code touching
                             password_hash/role/user deletion MUST use these)
  auth.ts                  — Argon2 helpers + TOTP (admin login)
  session.ts               — custom admin sessions (web_sessions)
  songsurf.ts              — SongSurf JWT (jose, AUTH_JWT_SECRET)
  audit.ts / ratelimit.ts / api-health.ts / songsurf-events.ts
  db/                      — Drizzle schema (web_* tables) + auth-schema (ba_*)
routes/
  +page.svelte             — member login (POST /auth/password-check)
  signup/                  — invite-based signup
  home/friend/, members/*  — member zone
  japprends/*              — admin (login, dashboard, audit, songsurf pages, API endpoints)
  portal/+server.ts        — 301 → / (legacy watcher login URL)
  api/auth/[...all]/       — BetterAuth handler
scripts/
  migrate-web-users-to-ba.mjs — bulk account migration (idempotent, --dry-run)
```

**Sessions**: members = BetterAuth cookie `better-auth.session_token` (24h);
admin = custom cookie `rev0auth_admin_session` (8h, password + challenge +
honeypot + optional TOTP). Admin passkey (YubiKey) was removed with crates/web;
planned to return via the BetterAuth passkey plugin.

**Route groups**:
- Public: `/`, `/signup`, `/japprends/login`, `/portal` (redirect)
- Admin-protected: `/japprends/*`
- Member-protected: `/home/friend`, `/members/*`

## Key Design Notes

- web_users stays the business source of truth (approved, active, access_*);
  ba_* holds BetterAuth identity. Join key: `ba_users.username = LOWER(pseudo)`,
  `ba_users.name` = exact pseudo.
- New passwords are hashed with Argon2 (not scrypt) so hashes stay readable by
  crates/api during coexistence.
- `/auth/password-check` lazy-provisions ba_* rows for accounts that only exist
  in web_users (robust to deploy ordering).
- CSRF tokens (API crate) are stored in the `auth_refresh_tokens` table.
- Role hierarchy: `guest < member < mod < admin`.
- `make test` runs `scripts/security-audit.sh` before `cargo test`.
