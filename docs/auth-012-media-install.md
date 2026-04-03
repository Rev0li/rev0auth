# AUTH-012 - Media install-only

Date: 2026-04-03

## Objectif

Installer et brancher la partie media sur une infra deja en place, sans reconstruire le socle reseau.

## Scope

- infra existante: conservee
- NAS: deja accessible via Tailscale
- DldeMedia: a installer et relier
- validation: a documenter et a verifier

## Hors scope

- reconfiguration de l'infra VPS
- refonte du reverse proxy
- changement du modele d'auth principal

## Procedure

1. Verifier que `AUTH-009` est present et que l'API/reverse proxy sont operationnels.
2. Verifier l'accessibilite Tailscale vers le NAS.
3. Installer la partie media sur la cible prevue.
4. Relier le media aux endpoints/ACL prevus.
5. Valider la lecture, l'acces prive et les erreurs de base.
6. Noter le resultat dans la documentation learning.

## Verifications attendues

- le service media est joignable selon le chemin prevu
- le NAS n'est pas expose publiquement
- les URL signees restent a TTL court
- l'acces est conforme aux roles attendus

## Livrable de fin

- une installation reproductible
- un controle de bout en bout minimal
- une trace claire dans `docs/learning/BookOfDev.md` et `docs/roadmap-detailed.md`
