# AUTH-009-VPS-SETUP

Date: 2026-04-02

## Objectif

Fournir un setup VPS reproductible et idempotent, pilotable ensuite par un Makefile commun qui execute les `setup.sh` de chaque projet.

## Livrables

- Script: `scripts/setup-vps.sh`
- Script orchestration: `scripts/devtools.sh`
- Launch all: `scripts/launch-all.sh`
- Launch individuel: `scripts/launch.sh`
- Stop services: `scripts/stop.sh`
- Compose API: `docker-compose.yml`
- Variables exemples: `.env.example`
- Makefile racine pour orchestration

## Ce que fait le script

Le script `scripts/setup-vps.sh` est idempotent et supporte `--dry-run`.

Checklist couverte par le script:
- UFW avec policy stricte + ouvertures `22`, `80`, `443`
- installation + activation `fail2ban`
- activation `unattended-upgrades`
- installation Docker Engine + Docker Compose plugin
- creation optionnelle d'un utilisateur non-root sudo (`--admin-user`)

## Usage

Execution directe:

```bash
sudo ./scripts/setup-vps.sh --admin-user deploy
```

Avec cle SSH admin:

```bash
sudo ./scripts/setup-vps.sh \
  --admin-user deploy \
  --admin-ssh-key "ssh-ed25519 AAAA..."
```

Mode simulation (recommande avant prod):

```bash
sudo ./scripts/setup-vps.sh --admin-user deploy --dry-run
```

## Integration future Makefile

Exemple cible Makefile:

```makefile
setup-vps:
	bash ./scripts/setup-vps.sh --admin-user deploy
```

Targets prets dans le projet:
- `make setup-vps ARGS="--admin-user deploy --dry-run"`
- `make launch-all`
- `make launch-api`
- `make launch-web`
- `make stop-all`
- `make status`

## Compose

`docker-compose.yml` de base:
- service `postgres:16` avec volume `pgdata`
- service `api` build depuis `crates/api`
- bind API sur `127.0.0.1:8080:8080`
- secrets injectes via variables (`POSTGRES_PASSWORD`, `AUTH_JWT_SECRET`, `DATABASE_URL`)

## Notes

- Le script cible un environnement Debian/Ubuntu (APT).
- Les secrets doivent venir d'un vault/manager et non etre commits.
- Etape suivante naturelle: AUTH-009-DUCKDNS (reverse proxy TLS + DNS).
