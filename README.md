# rev0auth

Plateforme Rust pour:
- zone privee membres (roles/rangs)
- diffusion/partage video (DldeMedia)
- zone publique portfolio + CV + contact pro

## Architecture cible

- `crates/api`: API auth + RBAC + gestion membres
- `crates/web`: frontend public/prive (SSR initial)
- VPS: reverse proxy HTTPS (Caddy/Nginx)
- DNS: DuckDNS
- NAS: acces prive via Tailscale uniquement

## Demarrage local

```bash
export AUTH_JWT_SECRET='replace-with-32-bytes-minimum-secret'
~/.cargo/bin/cargo run -p rev0auth-api
~/.cargo/bin/cargo run -p rev0auth-web
```

API healthcheck:

```bash
curl http://localhost:8080/health
```

Preflight avant deploy:

```bash
export ADMIN_DASH_PASSWORD='change-me'
make preflight
```

## Vision securite

- JWT court + refresh tokens rotatifs
- RBAC (guest/member/mod/admin)
- secrets via variables d'environnement
- NAS non expose publiquement (Tailscale tailnet)
- logs d'audit sur connexions et actions sensibles

## Module Auth implemente

Routes disponibles:
- POST /auth/signup
- POST /auth/login
- POST /auth/refresh

Documentation active:
- docs/public-project-handbook.md
- docs/checklists-master.md
- docs/tickets-auth.md

## AUTH-005 - Backend PostgreSQL

Le backend auth utilise PostgreSQL automatiquement si `DATABASE_URL` est defini.
Sinon, il reste en mode memoire (pratique pour tests locaux).

Exemple:

```bash
export AUTH_JWT_SECRET='replace-with-32-bytes-minimum-secret'
export DATABASE_URL='postgres://postgres:postgres@localhost:5432/rev0auth'
~/.cargo/bin/cargo run -p rev0auth-api
```

Schema SQL versionne:
- crates/api/migrations/0001_auth_schema.sql

Rapport ticket AUTH-005 (archive):
- docs/archive/2026-04-clean-2/auth-005-report.md
## Audit complet & Roadmap future

Pour voir TOUT le detail des fonctions/variables/tests:
- [docs/archive/2026-04-clean-2/audit-auth-complete.md](docs/archive/2026-04-clean-2/audit-auth-complete.md)

Pour la timeline jour par jour (10 jours) vers produit final:
- [docs/roadmap-detailed.md](docs/roadmap-detailed.md)