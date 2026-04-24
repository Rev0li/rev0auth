# Commit Plan - de 0 a maintenant

## Realite technique

Le depot Git local a ete initialise tardivement.
On ne peut pas reconstruire automatiquement l'historique exact passe commit par commit sans rejouer les etapes.

## Plan realiste et pedagogique (recommande)

On cree une histoire de formation fiable sur la branche `learning/devbook-story`.

### Etape L0 - socle
- commit docs/index/workflow
- objectif: point d'entree clair

### Etape L1 - backend minimal
- router + health
- modele auth minimal
- tests smoke

### Etape L2 - signup
- route signup
- hash password
- tests signup

### Etape L3 - login
- route login
- verification password
- tests login

### Etape L4 - refresh
- rotation refresh
- tests refresh

### Etape L5 - securite
- CSRF
- cookies
- rate limit
- tests associes

### Etape L6 - RBAC et audit
- extractor JWT
- guard roles
- audit events
- tests associes

### Etape L7 - DB et migrations
- postgres backend
- migrations
- tests integration

### Etape L8 - hardening deploy (DONE)
- AUTH-009-VPS-SETUP
- AUTH-009-DUCKDNS
- AUTH-009-SECRETS
- AUTH-009-HEALTH-CHECKS

### Etape L9 - web public + members zone (DONE)
- AUTH-010-WEB-PUBLIC
- AUTH-011-MEMBERS-ZONE

### Etape L10 - media install-only (NEXT)
- AUTH-012-MEDIA
- installation sur infra deja en place
- verification de branchement NAS/Tailscale
- preuve de verification (runbook + checks)

### Etape L11 - consolidation backend
- NEST-001A/B deja couverts
- suite NEST-001C..G

## Regle commit

- 1 commit = 1 intention
- code compilable a chaque commit
- tests minimaux inclus
- message explicite et court

## Note

Ce plan est une histoire d'apprentissage fiable, pas une simulation trompeuse de l'historique original.
