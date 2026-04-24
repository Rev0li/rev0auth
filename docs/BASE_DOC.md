# BASE_DOC — rev0auth Project Bible

> Single source of truth for vision, architecture, stack, commands, environment, deployment, security, and routes.

---

## Vision

rev0auth is a private authentication platform built for a small community of friends.
It manages access to self-hosted services (Jellyfin, Songsurf, GitHub) with a clean admin dashboard and a member portal.

**Pillars: Simple · Secure · Modern**

- Simple: no framework bloat, pure Rust SSR, zero JS dependencies
- Secure: JWT + CSRF + RBAC + audit logs + 2FA (TOTP, WebAuthn planned)
- Modern: open-source v1 target, clean codebase, documented for contributors

---

## Architecture

Cargo workspace — two crates:

```
crates/
  api/    (rev0auth-api)  — Axum HTTP API: auth, tokens, RBAC, audit
  web/    (rev0auth-web)  — SSR member portal + admin dashboard (pure Rust)
docs/                     — Operations, features, archive
Makefile                  — All dev/ops commands
```

### API crate (`crates/api/src/`)

| Module | Role |
|--------|------|
| `app/handlers.rs` | Axum route handlers |
| `app/domain.rs` | Request/response types |
| `app/services.rs` | Business logic |
| `auth/jwt.rs` | Token generation and validation (HS256) |
| `auth/store.rs` | User and token persistence (PostgreSQL or in-memory) |
| `auth/password.rs` | Argon2 hashing |
| `auth/cookies.rs` | CSRF and session cookies |
| `auth/rate_limit.rs` | Login rate limiting |
| `auth/rbac.rs` | Role-based access control |
| `auth/audit.rs` | Audit event logging |
| `auth/extractor.rs` | Axum request extractors |

Database tables: `auth_users`, `auth_refresh_tokens` (with CSRF column), `auth_audit_logs`.
Migrations: `crates/api/migrations/`.

### Web crate (`crates/web/src/`)

Pure Rust SSR — no JS framework, no templates.
State: in-memory `AppState` (Arc<Mutex<...>>). No PostgreSQL from web crate yet.
Admin session: cookie-based, 8h TTL, HttpOnly, SameSite=Lax.

---

## Stack

| Layer | Technology |
|-------|-----------|
| Language | Rust (stable) |
| API framework | Axum |
| Password hashing | Argon2 |
| Tokens | JWT (HS256), short-lived + refresh rotation |
| Database | PostgreSQL (API crate) — fallback: in-memory |
| Frontend | Pure Rust SSR (web crate) |
| Reverse proxy | Caddy |
| Deployment | Docker / VPS |
| 2FA (current) | TOTP via `ADMIN_DASH_TOTP_SECRET` |
| 2FA (planned) | WebAuthn / YubiKey (see `docs/features/webauthn-admin.md`) |

---

## Environment Variables

Copy `.env.example` to `.env`.

| Variable | Required | Description |
|----------|----------|-------------|
| `AUTH_JWT_SECRET` | yes | Min 32-byte secret for JWT signing |
| `DATABASE_URL` | no | PostgreSQL URL — falls back to in-memory if unset |
| `ADMIN_DASH_PASSWORD` | yes | Admin login password |
| `ADMIN_DASH_PSEUDO` | no | Admin display name |
| `ADMIN_DASH_SEED` | no | Admin seed value |
| `ADMIN_DASH_TOTP_SECRET` | no | Base32 TOTP secret for 2FA |
| `API_BIND_ADDR` | no | Override API port (default: `0.0.0.0:8080`) |
| `WEB_BIND_ADDR` | no | Override web port (default: `0.0.0.0:3000`) |
| `REV0AUTH_API_UPSTREAM` | no | Web → API proxy target (default: `127.0.0.1:8080`) |

Port conflict tip — if `8080` is taken (e.g. by Songsurf):
```bash
export API_BIND_ADDR='0.0.0.0:18080'
export REV0AUTH_API_UPSTREAM='127.0.0.1:18080'
```

---

## Commands

```bash
# Start everything
make launch-all              # API + Web + DB (Docker for DB)
make launch-api              # API only (background)
make launch-web              # Web only (background)
make stop-all                # Stop all services
make status                  # Check running services

# Tests
make test                    # security-audit.sh + cargo test
~/.cargo/bin/cargo test -p rev0auth-api
~/.cargo/bin/cargo test -p rev0auth-api -- auth::tests::my_test  # single test

# Build checks
~/.cargo/bin/cargo check -p rev0auth-api
~/.cargo/bin/cargo check -p rev0auth-web

# Database
export DATABASE_URL='postgres://postgres:postgres@localhost:5432/rev0auth'
~/.cargo/bin/cargo sqlx migrate run

# Admin 2FA
make admin-2fa-init          # Initialize TOTP secret
make admin-otp               # Generate current OTP
```

---

## Routes

### API (`127.0.0.1:8080`)

| Method | Path | Description |
|--------|------|-------------|
| GET | `/health` | Health check |
| POST | `/auth/signup` | User registration |
| POST | `/auth/login` | Login — returns JWT + refresh token |
| POST | `/auth/refresh` | Refresh token rotation |
| GET | `/auth/me` | Current user info |
| GET | `/admin/panel` | Admin panel (requires admin role) |

### Web (`127.0.0.1:3000`)

Public:
- `/` — Home
- `/portal` — Member portal entry

Admin-protected:
- `/japprends/login` — Admin login
- `/dashboard` — Admin dashboard
- `/japprends/tdd` — Live test runner UI
- `/japprends/endpoints` — Endpoint status
- `/japprends/*` — User management, approvals, messages, donations

Member-protected:
- `/home/friend` — Member home
- `/members/dashboard` — Member dashboard
- `/members/profile` — Profile management
- `/members/*` — Messages, donations, avatar, account

Health / status:
- `/status` — Web status
- `/status/all` — Full stack status

---

## Security Model

- **Role hierarchy**: `guest < member < mod < admin`
- **RBAC**: enforced per-route via `auth/rbac.rs` extractor
- **CSRF**: token stored in `auth_refresh_tokens` table, validated on mutations
- **Passwords**: Argon2id
- **Tokens**: short-lived JWT + refresh rotation; CSRF column in refresh table
- **Rate limiting**: login endpoint
- **Audit log**: admin actions recorded in `auth_audit_logs`
- **Admin 2FA**: TOTP (optional); WebAuthn/YubiKey planned
- **Secrets**: env vars only — never in code or committed files
- **`make test`**: runs `scripts/security-audit.sh` before `cargo test` (checks for hardcoded secrets)

---

## Deployment Checklist

Before exposing publicly:

1. `cargo check -p rev0auth-api` passes
2. `cargo check -p rev0auth-web` passes
3. `make test` passes
4. Reverse proxy (Caddy) points to correct internal ports
5. All secrets set in target environment
6. Sensitive routes not directly exposed (only through reverse proxy)
7. Domain resolves to VPS, TLS active (port 80 redirect + 443)

Post-deploy verification:
- Home page accessible
- Admin dashboard accessible
- `GET /health` returns 200
- No 5xx on base routes

---

## Key Docs

| File | Purpose |
|------|---------|
| `docs/BASE_DOC.md` | This file — project bible |
| `docs/connexion.md` | JWT integration for linked services (Songsurf, Jellyfin) |
| `docs/install-to-launch.md` | Local → VPS deployment guide |
| `docs/public-project-handbook.md` | Public-facing project overview |
| `docs/cheatsheet-complet.md` | Quick reference — commands, URLs, debug |
| `docs/features/webauthn-admin.md` | WebAuthn/YubiKey feature spec (private) |
| `docs/operations/README.md` | Operations index |
| `docs/auth-009-*.md` | VPS, secrets, health checks, DuckDNS setup |
| `docs/caddy-duckdns-beginners.md` | Caddy + DuckDNS setup guide |
| `Makefile` | All dev/ops commands |
| `.env.example` | Required environment variables |
