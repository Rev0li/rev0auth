# rev0auth - SaaS Platform

Plateforme SaaS Rust pour gestion d'identité, d'accès et connexion de services externes.

## Features

- **Auth & RBAC**: Signup/login sécurisés, tokens JWT rotatifs, roles granulaires
- **Connection Dashboard**: Vue unifiée des services connectés (GitHub, Jellyfin, Songsurf)
- **Admin Control**: Gestion users, activation d'accès, audit logs
- **Private Zone**: Espace membre personnalisé avec profil, avatar, messages
- **Portfolio Public**: Pages publiques (home, profil, CV)
- **Security First**: CSRF, rate-limit, validation stricte, logs d'audit

## Quick Start

### Local

```bash
# Setup variables
export AUTH_JWT_SECRET='replace-with-32-bytes-minimum-secret'
export ADMIN_DASH_PASSWORD='change-me'

# Launch
make launch-all

# Verify
curl http://localhost:8080/health
```

API: `http://127.0.0.1:8080`
Web: `http://127.0.0.1:3000`

### Production

```bash
export AUTH_JWT_SECRET='your-production-secret'
export DATABASE_URL='postgres://...'
make preflight
./scripts/setup-vps.sh
./scripts/install-caddy-template.sh
```

Voir [docs/install-to-launch.md](docs/install-to-launch.md) pour le déploiement complet.

## Architecture

- **API** (`crates/api`): Auth, RBAC, user management, audit
- **Web** (`crates/web`): Admin dashboard, member zone, public pages
- **Database**: PostgreSQL (avec fallback memoire pour dev)
- **Infrastructure**: Caddy reverse proxy, DuckDNS, Tailscale (pour NAS privé)

## Services

Intégration gérée:
- **GitHub** (star verification, username linking)
- **Jellyfin** (accès media)
- **Songsurf** (music service)

Admin contrôle activation par user.

## Documentation

- [Roadmap](docs/roadmaps/first_stable-roadmap.md)
- [Handbook & Troubleshooting](docs/public-project-handbook.md)
- [Operations Guide](docs/operations/README.md)
- [Learning](docs/learning/README.md)

## Testing & Quality

```bash
make test              # All tests
cargo check -p rev0auth-web
cargo check -p rev0auth-api
```

## License & Status

Status: **v1-alpha** (firs_stable baseline)
Target: v1.0.0 (clean + security hardened + UI polish)