# Next Session

Date: 2026-04-21
Branch: feature/saas-dashboard-clean-1

## Done this session

- Created `CLAUDE.md` (codebase bible for Claude Code)
- Created `docs/connexion.md` (JWT integration guide for linked services — Songsurf, etc.)
- Audited all docs/ — plan ready, waiting for confirmation to execute

## Direction confirmed

Goal: transition rev0auth from learning project → stable open-source v1.
Pillars: **Simple, Secure, Modern.**

## Step 1 — Docs cleanup (next action, ready to execute)

Plan agreed, needs confirmation to run:

**Archive** (move to `docs/archive/`):
- `docs/learning/` (entire directory)
- `docs/roadmap-detailed.md`
- `docs/clean-fixpoints-saas-roadmap.md`
- `docs/polish-finalisation.md`
- `docs/clean-inventory-phase1.md`
- `docs/checklists-master.md`
- `docs/architecture-web-scan-2026-04-04.md`
- `docs/roadmaps/first_stable-roadmap.md`
- `docs/tickets-auth.md`
- `docs/notes/personal/`

**Keep:** all `auth-009-*`, `install-to-launch.md`, `public-project-handbook.md`, `cheatsheet-complet.md`, `operations/`, `archive/`

**Create:** `docs/BASE_DOC.md` — consolidated project bible (vision, architecture, stack, commands, env, deployment, security, routes)

**Move:** `feature_key_admin.md` → `docs/features/webauthn-admin.md` (private, not in public docs)

## Step 2 — Complete security test matrix

Remaining abuse cases in `crates/api/src/auth/handlers.rs` tests:
- Refresh token replay (deep)
- CSRF full mutation coverage
- RBAC escalation member → admin end-to-end

## Step 3 — Security hardening

- Uniform error responses (no info leak)
- Upload limits (size, MIME, extension) in web crate
- Admin action audit log — hardened

## Step 4 — V1 visual

- CSS token system + shared components
- Stabilize 3 core screens: home, dashboard, profile
- Minimum UI smoke tests

## Step 5 — WebAuthn / YubiKey admin

Spec in `feature_key_admin.md`. Use `webauthn-rs` crate.
Flow: password → temp JWT (`mfa_pending: true`) → YubiKey → full JWT (`mfa: true`).
Requires HTTPS + valid domain (already in place via Caddy).

## Key files to know

| File | Purpose |
|------|---------|
| `docs/connexion.md` | JWT integration for linked services |
| `feature_key_admin.md` | WebAuthn/YubiKey feature spec (private) |
| `crates/api/src/auth/jwt.rs` | Token issuance (HS256, claims structure) |
| `crates/api/src/auth/extractor.rs` | Bearer validation logic |
| `Makefile` | All dev/ops commands |
| `.env.example` | Required environment variables |
