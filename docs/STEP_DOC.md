# Pre-Launch Checklist — rev0auth

Ordered steps to run through before pointing a real domain at this server.
Each section must be fully green before moving to the next.

---

## 1. Tests & Security Audit

```bash
make test
# Runs scripts/security-audit.sh then cargo test for both crates.
# Zero failures required. Zero hardcoded secrets flagged.
```

- [ ] `cargo test -p rev0auth-api` — all API tests pass
- [ ] `cargo test -p rev0auth-web` — all web tests pass (TOTP + HTTP integration)
- [ ] `scripts/security-audit.sh` — no secrets in code, no TODO/FIXME left open
- [ ] No `unwrap()` calls on user-controlled input paths (grep check)

---

## 2. Environment Variables

Copy `.env.example` → `.env` on the VPS and fill every value from your secrets manager.

| Variable | Requirement |
|---|---|
| `AUTH_JWT_SECRET` | Min 32 bytes, random, never reused |
| `DATABASE_URL` | Postgres URL, same as used for migrations |
| `POSTGRES_PASSWORD` | Strong, matches `DATABASE_URL` |
| `ADMIN_DASH_PASSWORD` | Strong password, not the same as DB password |
| `ADMIN_DASH_PSEUDO` | Non-default value (not `admin`) |
| `ADMIN_DASH_SEED` | Non-default value (not `rev0auth-seed`) |
| `ADMIN_DASH_TOTP_SECRET` | Base32 TOTP secret — enroll in authenticator app **before** launch |
| `ADMIN_WEBAUTHN_CREDENTIAL` | Leave empty until you enroll YubiKey on first boot |
| `WEBAUTHN_RP_ID` | Your domain, e.g. `auth.example.com` |
| `WEBAUTHN_RP_ORIGIN` | `https://auth.example.com` (HTTPS required for WebAuthn) |
| `API_BIND_ADDR` | Default `0.0.0.0:8080` — keep internal, do not expose directly |
| `WEB_BIND_ADDR` | Default `0.0.0.0:3000` — keep internal, reverse-proxy only |
| `REV0AUTH_API_UPSTREAM` | `127.0.0.1:8080` (web crate → api crate proxy) |

- [ ] All required vars set and non-empty
- [ ] `.env` not committed to git (`git status` check)
- [ ] `.env` readable only by the service user (`chmod 600 .env`)

---

## 3. Database

```bash
export DATABASE_URL='...'
~/.cargo/bin/cargo sqlx migrate run
```

- [ ] Migrations applied cleanly (`0001`, `0002`, `0003` at minimum)
- [ ] `auth_users`, `auth_refresh_tokens`, `auth_audit_logs` tables exist
- [ ] DB accessible only from `127.0.0.1` (not bound to `0.0.0.0`)
- [ ] Postgres password matches `DATABASE_URL`

---

## 4. TLS / Reverse Proxy

WebAuthn **requires HTTPS** — the browser will refuse `navigator.credentials` on plain HTTP except for `localhost`.

```
Domain: auth.example.com
TLS:    Caddy (auto via Let's Encrypt) or nginx + certbot
Proxy:  :443 → 127.0.0.1:3000 (web)   :8443 or internal → 127.0.0.1:8080 (api)
```

- [ ] TLS certificate valid and auto-renewing
- [ ] `WEBAUTHN_RP_ORIGIN` matches the actual HTTPS origin exactly (no trailing slash)
- [ ] HTTP → HTTPS redirect in place
- [ ] API port not publicly accessible (firewall / ufw rule)
- [ ] HSTS header set by reverse proxy (`Strict-Transport-Security`)

**Caddy snippet:**
```
auth.example.com {
    reverse_proxy 127.0.0.1:3000
}
```

---

## 5. Firewall

```bash
ufw allow 22/tcp      # SSH
ufw allow 80/tcp      # HTTP (for ACME challenge)
ufw allow 443/tcp     # HTTPS
ufw deny 8080/tcp     # API — internal only
ufw deny 3000/tcp     # Web — internal only
ufw deny 5432/tcp     # Postgres — internal only
ufw enable
```

- [ ] Only 22, 80, 443 open externally
- [ ] 8080, 3000, 5432 blocked from internet

---

## 6. Systemd Services

Two units: `rev0auth-api.service` and `rev0auth-web.service`.

```ini
[Unit]
Description=rev0auth API
After=network.target postgresql.service

[Service]
User=rev0auth
EnvironmentFile=/opt/rev0auth/.env
ExecStart=/opt/rev0auth/rev0auth-api
Restart=on-failure
RestartSec=5

[Install]
WantedBy=multi-user.target
```

- [ ] Both services start and stay up (`systemctl status`)
- [ ] `Restart=on-failure` and `RestartSec=5` set
- [ ] Run as a dedicated non-root user
- [ ] Binary paths correct, binaries built in release mode (`cargo build --release`)

---

## 7. First-Boot Admin Setup (in order)

Run these steps once, immediately after services are up, before sharing any URL.

1. **Verify admin login** — navigate to `/japprends/login`, enter pseudo + seed + password
2. **Enroll TOTP** — scan the QR from `make admin-2fa-init` into your authenticator app, verify a code works
3. **Enroll YubiKey** — go to Dashboard → Security tab → click "Enregistrer ma clé YubiKey" → touch key
4. **Copy the credential JSON** shown in the textarea into `.env` as `ADMIN_WEBAUTHN_CREDENTIAL`
5. **Restart the web service** so the credential loads from env: `systemctl restart rev0auth-web`
6. **Test full login flow** — password + TOTP code + YubiKey touch → lands on dashboard
7. **Run the TDD suite** from the dashboard Testing tab — all cases should pass

- [ ] TOTP enrolled and working
- [ ] YubiKey enrolled and working
- [ ] `ADMIN_WEBAUTHN_CREDENTIAL` persisted in `.env`
- [ ] Full 3-factor login verified end-to-end

---

## 8. Smoke Tests (post-deploy)

```bash
# Health
curl -sf https://auth.example.com/status | jq .

# Public pages respond
curl -o /dev/null -sw "%{http_code}\n" https://auth.example.com/
curl -o /dev/null -sw "%{http_code}\n" https://auth.example.com/portal

# Admin login page loads
curl -o /dev/null -sw "%{http_code}\n" https://auth.example.com/japprends/login

# Protected route redirects (not 200 without session)
curl -o /dev/null -sw "%{http_code}\n" https://auth.example.com/dashboard

# API health (if port exposed internally)
curl -sf http://127.0.0.1:8080/health | jq .
```

- [ ] `/status` returns `{ admin_ok: true, user_ok: true }`
- [ ] `/dashboard` returns 302 (redirect to login) when unauthenticated
- [ ] API `/health` responds 200
- [ ] HTTPS cert valid (no browser warning)

---

## 9. Rate Limiting

The API crate has per-IP login rate limiting built in (5 failures → lockout).
Verify it is not bypassed by the reverse proxy forwarding the wrong IP.

- [ ] Proxy forwards `X-Forwarded-For` or `X-Real-IP` header
- [ ] Rate limit test: 5 wrong-password logins on the same account → 429 or locked response

---

## 10. Ongoing

- [ ] Log rotation configured (`/var/log/journal` or logrotate for file logs)
- [ ] Backup for Postgres (`pg_dump` cron or managed backup)
- [ ] Alert on service crash (systemd email notify or uptime monitor)
- [ ] Plan for JWT secret rotation (requires re-issue of all refresh tokens)
