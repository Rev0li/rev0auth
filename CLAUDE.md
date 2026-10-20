# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

rev0auth is an authentication platform with RBAC. The migration to SvelteKit is
complete: the **SvelteKit frontend serves the whole site** (portal, members,
admin, signup) with one BetterAuth session for everyone; the admin console
additionally requires a security key. The Rust backend (`crates/`) has been
removed (Phase 4). Services sit behind the portal gate (Caddy forward_auth), see `docs/connexion.md`.

- `frontend/` — SvelteKit (adapter-node, port 4173 in prod, 5173 in dev): all
  pages + API endpoints, Drizzle + PostgreSQL, BetterAuth (`ba_*` tables: the
  single user model and its sessions)

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
- `BETTER_AUTH_SECRET` — min 32-byte secret for BetterAuth sessions (signs the
  session cookie: changing it logs everyone out). Formerly `AUTH_JWT_SECRET`,
  renamed with the same value in prod (AUTH-09)
- `SONGSURF_URL` — public SongSurf address (https://songsurf.…), behind the portal gate
- `DATABASE_URL` — PostgreSQL connection string
- `PORT` — frontend port (4173 in prod compose, 5173 in dev)
- `ORIGIN` — public origin for the SvelteKit frontend (default https://rev0li.duckdns.org in compose);
  also the WebAuthn origin, its hostname is the RP ID of the admin security keys
- `SONGSURF_EVENTS_SECRET` — shared secret authenticating activity events pushed by the NAS to `POST /japprends/api/songsurf-events` (SvelteKit frontend). Must be byte-identical to `SongSurf/SongSurf/.secrets`. Events land in the `songsurf_events` table, displayed on `/japprends/songsurf-activity`.

## Architecture

### SongSurf behind the gate (AUTH-09)

No more JWT handoff: SongSurf sits behind the portal gate (Caddy forward_auth →
`/auth/verify?service=songsurf`), so the member and admin buttons are plain links
to `SONGSURF_URL`. The admin pages that read SongSurf's download log
(`/japprends/songsurf-logs`, `/japprends/songsurf-activity`) call it server-side
with the admin's own session cookie (`fetchSongsurfDlLogs` in `songsurf.ts`,
session cookie only, https on the portal domain only).

### Frontend (`frontend/src/`)

```
hooks.server.ts            — resolves locals.memberSession (BetterAuth ba_sessions)
                             and locals.adminSession (admin role + security key
                             validated in that session)
lib/server/
  auth-v2.ts               — BetterAuth instance (ba_* tables, username plugin,
                             custom Argon2 password hash/verify)
  users.ts                 — the single user model (ba_users): lookups, create,
                             update, password, sessions, delete, avatars
  auth.ts                  — Argon2 helpers, random tokens
  admin-passkey.ts         — admin security keys (WebAuthn, @simplewebauthn),
                             setup codes, session elevation
  songsurf.ts              — SongSurf address + admin calls through the gate
  forward-auth.ts          — service gate rules (access per service, safe
                             return URL) for /auth/verify
  audit.ts / ratelimit.ts / songsurf-events.ts
  db/                      — Drizzle schema (web_*, user_avatars, admin_passkeys)
                             + auth-schema (ba_*, incl. business fields),
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
```

**Sessions**: one BetterAuth session (`better-auth.session_token`, 24h) for
everyone. In https the cookie is set on the ORIGIN hostname as `Domain`, so it
reaches every subdomain (`songsurf.`, `jellyfin.`…) for forward_auth; Caddy
strips it (`sans_cookie_portail` in `infra/caddy/Caddyfile.template`) before
proxying to those services. In dev it stays host-only. Admin console = member with role `admin` + a security key (WebAuthn)
validated in that same session: `ba_sessions.admin_verified_at`, valid 8h, gone
with logout/expiry. First key (or lost key): one-time setup code from
`scripts/admin-setup.mjs`; extra keys from an already validated session
(dashboard › Sécurité). Not the BetterAuth passkey plugin: it signs in
passwordless and creates a second session, here the key is a second factor.

**Route groups**:
- Public: `/`, `/signup`, `/portal` (redirect)
- Service gate: `/auth/verify?service=songsurf|jellyfin`, called by Caddy
  `forward_auth` (snippet `porte_portail`) before each request to a service
  subdomain: 204 + `X-User-Id/Name/Role` if the member has access (admin: all),
  302 to `/?redirect=<url>` for a navigation without session, 401 for API
  calls, 403 without access (`lib/server/forward-auth.ts`)
- Admin role, key not yet validated: `/japprends/login`, `/japprends/passkey/{auth,register}-*`
- Admin-protected (role + validated key): every other `/japprends/*`
- Member-protected: `/home/friend`, `/members/*`

## Key Design Notes

- One user model: `ba_users` = BetterAuth identity + business fields (active,
  approved, bio, access_*/request_*, github/linkedin), declared as
  `additionalFields` (`input: false`) so `getSession` returns them. Pseudo:
  `username` = LOWER(pseudo) (login, unique), `name`/`displayUsername` = exact
  pseudo. Password = credential row in `ba_accounts`. Avatars in `user_avatars`
  (kept out of `ba_users` so sessions don't load image bytes). Go through
  `lib/server/users.ts`. The former `web_users` table was merged by migrations
  0004–0006.
- Passwords are hashed with Argon2 (custom hash/verify wired into BetterAuth in
  `auth-v2.ts`; scrypt fallback for legacy hashes).
- BetterAuth is only called server-side (`auth.api.*`): there is no public
  `/api/auth/*` route and direct sign-up is disabled — accounts are created
  server-side only (invite signup, admin).
- Member identity = BetterAuth `user.id` (`locals.memberSession.userId`); the
  pseudo shown is `user.name`, set server-side only (no route lets a client
  change it). Only an active account gets a member session.
- Password change (`/members/password`) goes through `auth.api.changePassword`
  and revokes the other sessions; an admin reset/removal revokes all of them.
- Role hierarchy: `guest < member < mod < admin`.
- `make test` runs `npm run check` + `vitest`.
- Access control is tested route × role in
  `lib/server/__tests__/routes-access.test.ts` (no DB: any DB/BetterAuth call
  means the guard was passed) and `hooks.test.ts` (session → locals). A new
  server route must be added to its `MATRIX`, or the coverage test fails.
- Migrations: never edit an applied migration; `0000_baseline` is idempotent so
  databases created by the former `initDb()` converge without data loss.
