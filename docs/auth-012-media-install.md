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

## Training launch local

Tu peux deja lancer et valider la base localement avant la partie media:

1. Lancer l'API et le web en local.
	- `make launch-all`
	- ou `make launch-api` puis `make launch-web`
2. Verifier que les services repondent.
	- `curl http://localhost:8080/health`
	- ouvrir la page web locale et verifier les routes visibles
3. Lancer les tests de base.
	- `make test`
	- si besoin, `~/.cargo/bin/cargo test -p rev0auth-api`

Limite du local:
- tu valides la base applicative et les routes de support
- tu ne peux pas valider le vrai flux NAS/Tailscale sans la cible media

## Verifications attendues

- le service media est joignable selon le chemin prevu
- le NAS n'est pas expose publiquement
- les URL signees restent a TTL court
- l'acces est conforme aux roles attendus

## Livrable de fin

- une installation reproductible
- un controle de bout en bout minimal
- une trace claire dans `docs/learning/BookOfDev.md` et `docs/roadmap-detailed.md`
