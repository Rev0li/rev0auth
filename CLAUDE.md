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
# All tables (web_*, ba_*, songsurf_events) are created by initDb() at frontend
# startup — no separate migration step. PostgreSQL via `docker compose up -d postgres`.

# Admin 2FA
make admin-2fa-init                    # Initialize TOTP secret
make admin-otp                         # Generate current OTP
```

## Environment

Copy `.env.example` to `.env`. Required variables:
- `AUTH_JWT_SECRET` — min 32-byte secret for JWT signing (shared with SongSurf)
- `DATABASE_URL` — PostgreSQL connection string
- `ADMIN_DASH_PASSWORD`, `ADMIN_DASH_PSEUDO`, `ADMIN_DASH_SEED` — admin credentials
- `ADMIN_DASH_TOTP_SECRET` — optional Base32 TOTP secret for 2FA
- `PORT` — frontend port (4173 in prod compose, 5173 in dev)
- `ORIGIN` — public origin for the SvelteKit frontend (default https://rev0li.duckdns.org in compose)
- `SONGSURF_EVENTS_SECRET` — shared secret authenticating activity events pushed by the NAS to `POST /japprends/api/songsurf-events` (SvelteKit frontend). Must be byte-identical to `SongSurf/SongSurf/.secrets`. Events land in the `songsurf_events` table (created by `initDb()`), displayed on `/japprends/songsurf-activity`.

## Architecture

### SongSurf JWT handoff

The Rust API used to mint the JWTs SongSurf validates. This now lives in
`frontend/src/lib/server/songsurf.ts` (`jose`, HS256, `AUTH_JWT_SECRET`):
- member login `/auth/password-check` → redirect `?token=` to SongSurf (8h)
- admin `/japprends/songsurf-access` → admin token (8h)
- admin `/japprends/songsurf-logs` → short-lived token (120s) for the logs proxy

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
  audit.ts / ratelimit.ts / songsurf-events.ts
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
honeypot + optional TOTP). Admin passkey (YubiKey) was removed with the Rust
crates; planned to return via the BetterAuth passkey plugin.

**Route groups**:
- Public: `/`, `/signup`, `/japprends/login`, `/portal` (redirect)
- Admin-protected: `/japprends/*`
- Member-protected: `/home/friend`, `/members/*`

## Key Design Notes

- web_users stays the business source of truth (approved, active, access_*);
  ba_* holds BetterAuth identity. Join key: `ba_users.username = LOWER(pseudo)`,
  `ba_users.name` = exact pseudo.
- Passwords are hashed with Argon2 (custom hash/verify wired into BetterAuth in
  `auth-v2.ts`; scrypt fallback for legacy hashes).
- `/auth/password-check` lazy-provisions ba_* rows for accounts that only exist
  in web_users (robust to deploy ordering).
- Role hierarchy: `guest < member < mod < admin`.
- `make test` runs `npm run check` + `vitest`.
