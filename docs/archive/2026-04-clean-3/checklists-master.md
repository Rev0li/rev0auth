# Checklists Master (Learning -> SaaS v1)

Date: 2026-04-03

## 1) Backup / Snapshot Checklist

- [x] Script snapshot projet disponible (`scripts/project-snapshot.sh`)
- [x] Dossier snapshots local (`backups/`) cree automatiquement
- [ ] Snapshot lance avant chaque lot critique
- [ ] Snapshot tagge avant release interne

Commande rapide:

```bash
make snapshot
```

## 2) Learning Completion Checklist

- [x] Admin protege par session + login
- [x] Namespace admin dedie (`/japprends/*`)
- [x] Dashboard systeme endpoints en visuel OK/KO
- [x] Profil user complet modifiable
- [x] Workflow demandes d'acces services (GitHub/Jellyfin/Songsurf)
- [x] Toggle admin d'activation/revocation
- [x] Document unique public (`docs/public-project-handbook.md`)
- [ ] Journal admin des actions sensibles
- [ ] Export etat dashboard JSON

## 3) Tests / Validation Checklist

- [x] `cargo check -p rev0auth-web`
- [ ] `cargo test -p rev0auth-api`
- [ ] Smoke test login admin
- [ ] `make preflight`
- [ ] Smoke test demande acces member
- [ ] Smoke test toggle acces admin
- [ ] Smoke test profil complet admin (precedent/suivant)

Template validation rapide:

```bash
~/.cargo/bin/cargo check -p rev0auth-web
~/.cargo/bin/cargo test -p rev0auth-api
make preflight
curl http://localhost:8080/health
curl http://localhost:3000/status
```

## 4) Release Checklist (Interne)

- [ ] Working tree propre ou changements intentional only
- [ ] Snapshot cree (`make snapshot`)
- [ ] Docs index a jour
- [ ] Commit message clair + scope restreint
- [ ] Push branche + PR

## 5) SaaS v1 Scope Checklist

## Core product

- [ ] Entite workspace/tenant (simple)
- [ ] Entite connection par service
- [ ] Statuts: disconnected/pending/connected/blocked
- [ ] Event feed (request/approve/revoke)

## Admin cockpit

- [ ] Filtres users + tri
- [ ] Bulk approve/revoke simples
- [ ] Metrics panel 24h
- [ ] Journal actions sensibles

## Member UX

- [ ] Etat detaille de la demande
- [ ] Raison de blocage explicite
- [ ] Message retour admin visible

## Go-live v1

- [ ] Seed/demo data
- [ ] Runbook demo publique
- [ ] Checklist incident + rollback

## Fixpoints UI/UX a ne pas oublier

- [ ] Toujours montrer l'etat systeme (OK/KO) sans ambiguite
- [ ] Afficher feedback apres chaque action admin/user
- [ ] Garder les actions destructives avec confirmation explicite
- [ ] Garder la navigation rapide admin (user precedent/suivant)
