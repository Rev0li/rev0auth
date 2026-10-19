# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

rev0auth is an authentication platform with RBAC. The migration to SvelteKit is
complete: the **SvelteKit frontend serves the whole site** (portal, members,
admin, signup) with BetterAuth member sessions. The Rust backend (`crates/`) has
been removed (Phase 4) — SvelteKit signs its own SongSurf JWTs.

- `frontend/` — SvelteKit (adapter-node, port 4173 in prod, 5173 in dev): all
  pages + API endpoints, Drizzle + PostgreSQL, BetterAuth (`ba_*` tables) for
  member sessions, custom admin sessions (`web_sessions`)

The old Rust crates (`crates/api`, `crates/web`) remain in git history for
reference; they are no longer in the working tree, build, or deployment.

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

# Database
export DATABASE_URL='postgres://postgres:postgres@localhost:5432/rev0auth'
# Schema = Drizzle migrations in frontend/drizzle/, applied at startup by
# migrateDb() (hooks.server.ts). PostgreSQL via `docker compose up -d postgres`.
npm run db:generate -- --name <change>  # new migration after editing db/*schema.ts
npx drizzle-kit generate --custom --name <x>  # hand-written data migration

# Admin console: admin role + one-time code to register a security key (15 min)
make admin-setup PSEUDO=<pseudo>       # docker compose exec frontend node scripts/admin-setup.mjs
node --env-file=.env scripts/admin-setup.mjs <pseudo>  # same, outside Docker
```

## Environment

Copy `.env.example` to `.env`. Required variables:
- `AUTH_JWT_SECRET` — min 32-byte secret for JWT signing (shared with SongSurf)
- `DATABASE_URL` — PostgreSQL connection string
- `PORT` — frontend port (4173 in prod compose, 5173 in dev)
- `ORIGIN` — public origin for the SvelteKit frontend (default https://rev0li.duckdns.org in compose);
  also the WebAuthn origin, its hostname is the RP ID of the admin security keys
- `SONGSURF_EVENTS_SECRET` — shared secret authenticating activity events pushed by the NAS to `POST /japprends/api/songsurf-events` (SvelteKit frontend). Must be byte-identical to `SongSurf/SongSurf/.secrets`. Events land in the `songsurf_events` table, displayed on `/japprends/songsurf-activity`.

## Architecture

### SongSurf JWT handoff

The Rust API used to mint the JWTs SongSurf validates. This now lives in
`frontend/src/lib/server/songsurf.ts` (`jose`, HS256, `AUTH_JWT_SECRET`):
- member login `/auth/password-check` → redirect `?token=` to SongSurf (8h)
- admin `/japprends/songsurf-access` → admin token (8h)
- admin `/japprends/songsurf-logs` → short-lived token (120s) for the logs proxy

### Frontend (`frontend/src/`)

```
hooks.server.ts            — resolves locals.memberSession (BetterAuth ba_sessions)
                             and locals.adminSession (admin role + security key
                             validated in that session)
lib/server/
  auth-v2.ts               — BetterAuth instance (ba_* tables, username plugin,
                             custom Argon2 password hash/verify)
  ba-sync.ts               — web_users → ba_* sync helpers (any code touching
                             password_hash/role/user deletion MUST use these)
  auth.ts                  — Argon2 helpers, random tokens
  admin-passkey.ts         — admin security keys (WebAuthn, @simplewebauthn),
                             setup codes, session elevation
  songsurf.ts              — SongSurf JWT (jose, AUTH_JWT_SECRET)
  audit.ts / ratelimit.ts / songsurf-events.ts
  db/                      — Drizzle schema (web_* tables) + auth-schema (ba_*),
                             migrateDb() applies frontend/drizzle/ at startup
routes/
  +page.svelte             — member login (POST /auth/password-check)
  signup/                  — invite-based signup
  home/friend/, members/*  — member zone
  japprends/*              — admin (login = security key, dashboard, audit,
                             songsurf pages, passkey/* endpoints)
  portal/+server.ts        — 301 → / (legacy watcher login URL)
scripts/
  admin-setup.mjs          — admin role + one-time setup code (creates the
                             account with a temporary password if missing)
  migrate-web-users-to-ba.mjs — bulk account migration (idempotent, --dry-run)
```

**Sessions**: one BetterAuth session (`better-auth.session_token`, 24h) for
everyone. Admin console = member with role `admin` + a security key (WebAuthn)
validated in that same session: `ba_sessions.admin_verified_at`, valid 8h, gone
with logout/expiry. First key (or lost key): one-time setup code from
`scripts/admin-setup.mjs`; extra keys from an already validated session
(dashboard › Sécurité). Not the BetterAuth passkey plugin: it signs in
passwordless and creates a second session, here the key is a second factor.

**Route groups**:
- Public: `/`, `/signup`, `/portal` (redirect)
- Admin role, key not yet validated: `/japprends/login`, `/japprends/passkey/{auth,register}-*`
- Admin-protected (role + validated key): every other `/japprends/*`
- Member-protected: `/home/friend`, `/members/*`

## Key Design Notes

- web_users stays the business source of truth (approved, active, access_*);
  ba_* holds BetterAuth identity. Join key: `ba_users.username = LOWER(pseudo)`,
  `ba_users.name` = exact pseudo.
- Passwords are hashed with Argon2 (custom hash/verify wired into BetterAuth in
  `auth-v2.ts`; scrypt fallback for legacy hashes).
- `/auth/password-check` lazy-provisions ba_* rows for accounts that only exist
  in web_users (robust to deploy ordering).
- BetterAuth is only called server-side (`auth.api.*`): there is no public
  `/api/auth/*` route and direct sign-up is disabled — accounts are created
  server-side only (invite signup, admin).
- Member identity = BetterAuth `user.username` (unique, set server-side), never
  `user.name`; `hooks.server.ts` reloads pseudo/role from web_users and only an
  active account gets a member session.
- Role hierarchy: `guest < member < mod < admin`.
- `make test` runs `npm run check` + `vitest`.
- Migrations: never edit an applied migration; `0000_baseline` is idempotent so
  databases created by the former `initDb()` converge without data loss.
