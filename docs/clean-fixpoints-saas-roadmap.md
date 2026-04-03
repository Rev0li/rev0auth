# Clean, Fixpoints, UI/UX, et Lancement SaaS Dashboard

Date: 2026-04-03

## Objectif

Transformer la base actuelle en version stable, propre, et prete pour la phase SaaS "Connection Dashboard".

## Fixpoints prioritaires (ce qui apporte le plus)

1. Securite et auth
- Centraliser les checks admin (session active, expiration, fallback login).
- Ajouter un journal admin minimal (qui a active/revoque quel acces, quand).
- Ajouter un mode read-only admin (optionnel) pour audit sans action destructive.

2. UI/UX admin
- Ajouter recherche + filtres dans la liste users (pseudo, acces, demandes en attente).
- Ajouter tri (dernier cree, plus actif, demandes en attente).
- Ajouter confirmations explicites pour les actions sensibles (delete, revoke access).

3. UI/UX member
- Afficher clairement "Pourquoi je suis bloque" pour chaque service.
- Ajouter timeline de demande d'acces (demande envoyee, lue, approuvee/refusee).
- Ajouter message de reponse admin visible cote user.

4. Observabilite
- Ajouter stats 24h (uptime, erreurs login, toggles effectues, demandes traitees).
- Ajouter p95 simple des checks API dans le dashboard.
- Ajouter export JSON de l'etat dashboard (debug support).

## CLEAN plan (sans casser le projet)

## Phase 1 - Inventaire

- Geler le code fonctionnel (tag/commit checkpoint).
- Identifier les fichiers non suivis/non relies.
- Marquer ce qui est:
  - a garder
  - a archiver
  - a supprimer

Sortie attendue:
- une liste claire des fichiers hors scope
- un plan de suppression par lot

## Phase 2 - Documentation canonique

- Garder un point d'entree unique: `docs/public-project-handbook.md`.
- Garder un guide debug rapide: `docs/cheatsheet-complet.md`.
- Marquer explicitement les docs historiques "archive" ou "active".

Sortie attendue:
- doc index sans ambiguite
- liens dashboard docs a jour

## Phase 3 - Hygiene code

- Verifier warnings et dead code (web/api).
- Uniformiser noms routes/messages UI.
- Ajouter checks de base en CI locale (check + test + format verify).

Sortie attendue:
- baseline stable et repetable

## Phase 4 - Stabilisation release Learning

Checklist release Learning:
1. auth admin OK
2. parcours member OK
3. demandes acces OK
4. toggles admin OK
5. docs publiques coherentes
6. smoke tests passes

## Lancement SaaS "Connection Dashboard" (scope v1)

## Vision v1

Un dashboard qui connecte:
- identite utilisateur
- autorisations par integration
- statut de connexion par service
- moderation admin simple

## Features v1

1. Workspace/tenant minimal
- notion de "projet" ou "espace"
- users rattaches a un espace

2. Connection cards
- GitHub
- Jellyfin
- Songsurf
- statut: disconnected / pending / connected / blocked

3. Rules engine simple
- regles d'acces par service
- validation manuelle admin
- prerequis service (ex: GitHub star claim)

4. Activity feed
- demandes, approvals, revokes
- horodatage + acteur

5. Metrics panel
- pending requests
- active users
- granted access ratio
- failed checks

## Roadmap execution proposee

Sprint A - Foundation
- modeles de donnees access requests + events
- endpoints CRUD basiques
- persistence (ou stockage transitoire propre si v1)

Sprint B - Admin cockpit
- liste users + filtres
- actions bulk simples
- event feed

Sprint C - Member experience
- statut detaille des demandes
- message admin retour
- UX de blocage explicatif

Sprint D - Hardening
- tests integration flux complets
- docs publiques finalisees
- package demo/share

## Questions decisionnelles (a trancher avant go)

1. SaaS v1 reste mono-tenant ou multi-tenant des le debut ?
2. On persiste en base tout de suite ou on stabilise encore en memoire + export backup ?
3. GitHub star verification: manuelle admin ou check API GitHub plus tard ?
4. Priorite mobile-first immediate ou desktop-first puis adaptation ?

## Actions immediates recommandees

1. Faire un commit "cleanup-inventory" avec la liste des fichiers a trier.
2. Tagger la baseline actuelle comme fin phase Learning.
3. Ouvrir le Sprint A SaaS avec 3 tickets:
- modele access-event
- endpoints connection-status
- vue dashboard connection v1
