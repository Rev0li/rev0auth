# rev0auth — plateforme d'authentification

Plateforme d'identité, d'accès et de connexion de services externes, écrite en
**SvelteKit** (adapter-node) + **PostgreSQL** + **BetterAuth**. Le backend Rust
a été retiré : SvelteKit sert l'intégralité du site et signe lui-même les JWT
pour SongSurf.

## Features

- **Auth & RBAC** : login membre (BetterAuth), login admin (mot de passe + seed + TOTP), rôles `guest/member/mod/admin`
- **Espace membre** : profil, avatar composable (DiceBear), messages, mur, donations
- **Admin** (`/japprends`) : gestion des users, file d'attente d'invitations, audit, activité SongSurf
- **Portail public** : page de login, signup sur invitation
- **SongSurf** : handoff JWT (HS256, `AUTH_JWT_SECRET` partagé)
- **Sécurité** : CSRF (adapter-node `ORIGIN`), rate-limit login, Argon2, logs d'audit en DB

## Quick Start

### Local

```bash
cd frontend
cp .env.example .env          # puis remplir AUTH_JWT_SECRET, ADMIN_DASH_*, DATABASE_URL
npm install
npm run dev                   # http://localhost:5173
```

PostgreSQL local (Docker) : `docker compose up -d postgres` à la racine `auth/`.
Les tables (`web_*`, `ba_*`, `songsurf_events`) sont créées par `initDb()` au démarrage.

### Stack complète (Docker)

```bash
make gen-secret               # crée .env interactivement
make launch-all               # docker compose up -d --build (postgres + frontend)
# Vérif : http://localhost:4173/japprends/login
```

### Production

Voir [`../DEPLOY.md`](../DEPLOY.md) — deploy VPS via Docker Compose + Caddy
(`make preflight`, `./scripts/setup-vps.sh`, `./scripts/install-caddy-template.sh`).

## Architecture

- **frontend/** (`SvelteKit`) : tout le site — pages + endpoints API, Drizzle, BetterAuth
- **Database** : PostgreSQL
- **Infrastructure** : Caddy reverse proxy → `:4173`, DuckDNS, Tailscale (NAS privé)

→ Détails dans [`CLAUDE.md`](CLAUDE.md).

## Testing & Quality

```bash
make test                     # frontend : npm run check + vitest
# ou directement :
cd frontend && npm run check && npm test
```

## License & Status

Status : **production** (migration SvelteKit terminée, backend Rust retiré).
