# BookOfDev - Index de formation

Objectif: garder un parcours de dev lisible, commit par commit, tout en pointant vers les index de reference du projet.

## Etat actuel

La documentation est maintenant repartie par theme, mais l'acces reste simple via les index racines.
Ce fichier sert de guide de lecture pour la formation, pas de copie de la roadmap.

## Regle de travail pendant la reconstruction

- 1 ticket = 1 commit
- chaque ticket met a jour au moins un document de suivi
- chaque etape doit etre relisible par un camarade sans contexte oral
- la branche sert de support TDD, pas de cage rigide
- l'implementation peut rester module-par-module, avec des pieces qui se collent proprement
- si un ticket demande du test avant code, on le respecte sans forcer un ordre artificiel
- les commentaires sont ecrits en anglais et expliquent l'intention, la securite et l'architecture

## Liens de base (sources de reference)

- `docs/README.md`
- `docs/learning/README.md`
- `docs/roadmap-detailed.md`
- `docs/Next-Work.md`
- `docs/tickets-auth.md`
- `docs/dev-book-auth.md`
- `docs/nest-001-audit-backend.md`
- `docs/operations/README.md`
- `docs/tickets/README.md`

## Parcours learning Git (branche reconstruction)

- `STEP-000` backend minimal health
- `STEP-001` signup route (validation + duplicate check in-memory)
- `STEP-002` tests signup (200 / 400 / 409)
- `STEP-003/004` login route + tests
- `STEP-005/006` refresh route + tests
- `STEP-007` bearer guard + `/auth/me`
- `STEP-008` RBAC modularisation + tests
- `STEP-009` hardening deploy (VPS, DuckDNS, secrets, health checks)
- `STEP-010` web public (landing + portal + dashboard)
- `STEP-011` members zone (dashboard/profile/avatar)
- `STEP-012` media install-only sur infra deja disponible (prochaine etape)

## Branch graph (reading order)

```text
master
	└── NEST-001A/B stable backend snapshot

learning/devbook-story
	└── DOC-000 -> DOC-001 -> DOC-002 -> DOC-003

learning/static-step0
	└── STEP-000 commented health baseline

feature/step-001-signup-route
	└── STEP-001 commented signup route

feature/step-002-signup-tests
	└── STEP-002 commented signup tests

feature/step-003-login-pair
	└── STEP-003/004 commented login route + tests

feature/step-005-refresh-pair
	└── STEP-005/006 commented refresh route + tests

feature/step-007-auth-guard
	└── STEP-007 bearer auth + /auth/me

feature/step-008-rbac
	└── STEP-008 modular auth app + tests
	└── AUTH-009 infra hardening + docs
	└── AUTH-010/011 web public + members zone
```

The graph is intentionally linear for onboarding: one branch, one feature, one clear lesson.

## Cible d'organisation finale

- `docs/learning/` -> histoire et commits pedagogiques
- `docs/tickets/` -> execution ticket par ticket
- `docs/operations/` -> runbook deploy/exploitation
- `docs/README.md` -> point d'entree unique
- `docs/learning/BookOfDev.md` -> chemin de lecture learning

## Note

Ce fichier est la colonne vertebrale de la formation pendant la phase de refonte.
La dispersion existe encore par theme, mais les index racines doivent rendre la lecture et la modification rapides.

## Etat valide au 2026-04-03

- Commit `26a6e61`: AUTH-009 (infra, scripts, caddy, health, secrets)
- Commit `692e959`: AUTH-010/011 (web public + members zone)
- Commit `89b2850`: roadmap recadree (NEST-001 precondition, AUTH-012 install-only)
