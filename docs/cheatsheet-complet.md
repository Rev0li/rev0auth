# Cheatsheet Complet Projet rev0auth

Date: 2026-04-03

## Objectif

Avoir une reference rapide pour:
- debug un test qui plante
- retrouver les commandes utiles
- partager une documentation publique lisible

## Architecture rapide

- API Rust: `crates/api`
- Web Rust (dashboard/admin/members): `crates/web`
- Docs globales: `docs/`
- Branch active de travail: `feature/step-008-rbac`

## Ports et services

- API: `127.0.0.1:8080`
- Web: `127.0.0.1:3000`

Checks rapides:

```bash
curl http://localhost:8080/health
curl http://localhost:3000/status
curl http://localhost:3000/status/all
```

## Commandes dev essentielles

Lancer local:

```bash
make launch-all
```

Lancer separe:

```bash
make launch-api
make launch-web
```

Tests:

```bash
make test
~/.cargo/bin/cargo test -p rev0auth-api
~/.cargo/bin/cargo check -p rev0auth-web
```

## Variables critiques

Dashboard admin:

```bash
export ADMIN_DASH_PASSWORD='change-me'
```

Sans cette variable, le login admin ne fonctionne pas.

## URLs importantes

Public:
- `/`
- `/portal`

Admin:
- `/japprends/login`
- `/dashboard`
- `/japprends/tdd`
- `/japprends/endpoints`

Member:
- `/members/dashboard`
- `/members/profile`

## Si un test plante (procedure courte)

1. Verifier que API et Web tournent.
2. Lancer `cargo check` sur le crate cible.
3. Relancer les tests du crate cible seulement.
4. Verifier routes de sante (`/status`, `/status/all`).
5. Regarder les derniers changements Git localement.

Commandes utiles:

```bash
git status --short
git --no-pager diff
~/.cargo/bin/cargo check -p rev0auth-web
~/.cargo/bin/cargo test -p rev0auth-api
```

## Debug dashboard admin

Dans l'onglet Overview:
- bouton `Launch test now`
- historique des runs avec date

Dans l'onglet System:
- liste complete des endpoints
- badge visuel OK/KO par scope (pas de navigation)

Dans l'onglet Admin:
- stats live: users total, users actifs, demandes pending, dernier run tests

## Publication documentation (public)

Minimum a partager:
- `docs/README.md`
- `docs/operations/README.md`
- `docs/install-to-launch.md`
- `docs/caddy-duckdns-beginners.md`
- `docs/cheatsheet-complet.md`

Bonnes pratiques:
- masquer secrets/tokens avant publication
- garder les commandes reproductibles
- lier tous les nouveaux docs aux index

## Liens utiles

- `docs/README.md`
- `docs/operations/README.md`
- `docs/install-to-launch.md`
- `docs/auth-012-media-install.md`
- `docs/caddy-duckdns-beginners.md`
