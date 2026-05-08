# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

rev0auth is a Rust authentication platform with RBAC, built on Axum + PostgreSQL. It has two crates:
- `crates/api/` (`rev0auth-api`) — pure auth API: JWT, CSRF, refresh tokens, RBAC, audit logging
- `crates/web/` (`rev0auth-web`) — server-side rendered member portal and admin dashboard

## Commands

```bash
# Build / run
make launch-all                        # Start API + Web + DB (docker-compose for DB)
make launch-api                        # API only (background)
make launch-web                        # Web only (background)
make stop-all                          # Stop all services
make status                            # Check running services

# Alternatively, run directly
~/.cargo/bin/cargo run -p rev0auth-api
~/.cargo/bin/cargo run -p rev0auth-web

# Tests
make test                              # security-audit.sh + cargo test
~/.cargo/bin/cargo test -p rev0auth-api
~/.cargo/bin/cargo test -p rev0auth-api -- auth::tests::my_test_name  # single test

# Database
export DATABASE_URL='postgres://postgres:postgres@localhost:5432/rev0auth'
~/.cargo/bin/cargo sqlx migrate run   # Run migrations from crates/api/migrations/

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
- `API_BIND_ADDR` / `WEB_BIND_ADDR` — override default ports (8080 / 3000)
- `REV0AUTH_API_UPSTREAM` — web → API proxy target (default: 127.0.0.1:8080)

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

### Web crate (`crates/web/src/`)

All rendering is pure Rust SSR — no JS framework, no templates. Each page and UI section is a Rust module that produces HTML strings.

```
pages/
  home.rs, portal.rs, admin_login.rs, dashboard.rs, friend.rs, profile.rs, tdd.rs
  dashboard_*_module.rs   — Admin dashboard tabs (users, donations, messages, queue, status, testing, theme)
  friend_*_module.rs      — Member zone sections (avatar, chat, status, services, onboarding)
  profile_*_module.rs     — Profile sections (edit, avatar, password, deletion, donations, messages)
  frontend_theme.rs       — Theme system (CSS variables, dark/light)
  *_page_styles.rs        — Per-page CSS
  *_page_assembly.rs      — Page layout composition
```

**Web state**: In-memory `AppState` (Arc<Mutex<...>>) stores users, sessions, messages, donations, avatars. No PostgreSQL connection from the web crate — all persistence is in-memory for now.

**Admin session**: Cookie-based (8h TTL, HttpOnly, SameSite=Lax). Admin login requires `ADMIN_DASH_PASSWORD` + optional TOTP.

**Web route groups**:
- Public: `/`, `/portal`, `/japprends/login`
- Admin-protected: `/dashboard`, `/japprends/*` (user management, signup approval, messages, donations, tests)
- Member-protected: `/home/friend`, `/members/*` (profile, messages, donations, avatar, account)

## Key Design Notes

- The web crate is intentionally standalone (in-memory) during the current development phase; it does not yet talk to the API crate over HTTP.
- CSRF tokens are stored in the `auth_refresh_tokens` table (added in migration 0003).
- Role hierarchy: `guest < member < mod < admin`. The `rbac.rs` extractor enforces minimum role per route.
- `make test` runs `scripts/security-audit.sh` before `cargo test` — the audit script checks for hardcoded secrets and other security issues.
- The TDD dashboard (`/japprends/tdd`) provides a live test runner UI that calls `POST /japprends/tests/launch`.
