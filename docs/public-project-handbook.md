# Public Project Handbook - rev0auth

Date: 2026-04-03

## Pourquoi ce document unique

Ce handbook sert de point d'entree unique pour:
- comprendre le projet rapidement
- lancer et verifier la stack
- diagnostiquer un incident ou un test KO
- partager la documentation publiquement

## TL;DR

- Stack principale: Rust API + Rust Web dashboard
- API locale: `http://127.0.0.1:8080`
- Web locale: `http://127.0.0.1:3000`
- Admin login: `/japprends/login`
- Dashboard admin: `/dashboard`
- Espace membre: `/members/dashboard`

## Architecture courte

- `crates/api`: logique API auth
- `crates/web`: pages web admin + member + monitoring
- `docs/`: documentation operations, learning, roadmap

## Demarrage rapide local

Prerequis:
- Rust toolchain installee
- variables d'environnement minimales

Commandes:

```bash
export ADMIN_DASH_PASSWORD='change-me'
make launch-all
```

Checks immediats:

```bash
curl http://localhost:8080/health
curl http://localhost:3000/status
curl http://localhost:3000/status/all
```

## Comptes et acces services

Principe securite:
- un user membre est cree sans acces externe par defaut
- les acces externes sont actives manuellement par l'admin

Services geres:
- GitHub
- Jellyfin
- Songsurf

Workflow GitHub:
1. User cree son compte.
2. User confirme qu'il a mis une star sur le projet.
3. User envoie une demande d'acces GitHub (avec username GitHub).
4. Admin active l'acces GitHub depuis dashboard user/admin.

Workflow Jellyfin / Songsurf:
1. User clique "demander acces".
2. Admin voit la demande et active le toggle.

## Dashboard admin (lecture rapide)

Overview:
- etats admin/user/api
- historique tests dashboard
- bouton "Launch test now"

Admin:
- validation demandes signup
- gestion users
- toggles d'acces GitHub/Jellyfin/Songsurf
- mini stats live (users, actifs, pending, dernier test)

System:
- verification de chaine (maillons critiques)
- liste endpoints avec badge OK/KO (non cliquable)

## Si un test plante

Procedure courte:
1. verifier que API + Web tournent
2. relancer check crate cible
3. relancer tests crate cible
4. verifier endpoints sante
5. inspecter diff git local

Commandes:

```bash
~/.cargo/bin/cargo check -p rev0auth-web
~/.cargo/bin/cargo test -p rev0auth-api
git status --short
git --no-pager diff
```

## Troubleshooting express

Symptome: login admin KO
- verifier `ADMIN_DASH_PASSWORD`
- verifier route `/japprends/login`

Symptome: endpoint DOWN dans dashboard
- verifier API sur 8080
- verifier endpoint `/status` puis `/status/all`

Symptome: user n'ouvre pas un service externe
- verifier que toggle service est ON cote admin
- verifier demande d'acces cote user
- pour GitHub: confirmer star + username fourni

## Publication publique docs

Pack minimal recommande:
- `docs/public-project-handbook.md`
- `docs/README.md`
- `docs/operations/README.md`
- `docs/install-to-launch.md`
- `docs/caddy-duckdns-beginners.md`

Avant publication:
1. retirer tokens/secrets et exemples sensibles
2. verifier cohérence des liens
3. verifier commandes copy/paste

## Liens internes complementaires

- `docs/cheatsheet-complet.md`
- `docs/polish-finalisation.md`
- `docs/install-to-launch.md`
- `docs/caddy-duckdns-beginners.md`
- `docs/auth-012-media-install.md`
