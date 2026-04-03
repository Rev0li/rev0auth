# BookOfDev - Index de formation (phase transitoire)

Objectif: transformer une documentation actuellement dispersee en parcours de dev lisible, commit par commit.

## Etat actuel

Les documents existent, mais sont repartis dans plusieurs fichiers et themes.
Cette phase est volontaire: on garde les feuilles en place pendant la reconstruction, puis on organise proprement en fin de cycle.

## Regle de travail pendant la reconstruction

- 1 ticket = 1 commit
- chaque ticket met a jour au moins un document de suivi
- chaque etape doit etre relisible par un camarade sans contexte oral
- la branche sert de support TDD, pas de cage rigide
- l'implementation peut rester module-par-module, avec des pieces qui se collent proprement
- si un ticket demande du test avant code, on le respecte sans forcer un ordre artificiel

## Liens de base (source actuelle)

- `docs/roadmap-detailed.md`
- `docs/Next-Work.md`
- `docs/tickets-auth.md`
- `docs/dev-book-auth.md`
- `docs/nest-001-audit-backend.md`

## Parcours learning Git (branche reconstruction)

- `STEP-000` backend minimal health
- `STEP-001` signup route (validation + duplicate check in-memory)
- `STEP-002` tests signup (200 / 400 / 409)
- `STEP-003+` login/refresh puis hardening
- `STEP-00X` hardening, DB, ops

## Cible d'organisation finale

- `docs/learning/` -> histoire et commits pedagogiques
- `docs/tickets/` -> execution ticket par ticket
- `docs/operations/` -> runbook deploy/exploitation
- `docs/README.md` -> point d'entree unique

## Note

Ce fichier est la colonne vertebrale de la formation pendant la phase de refonte.
On accepte la dispersion temporaire, mais chaque etape rapproche vers une structure claire.
