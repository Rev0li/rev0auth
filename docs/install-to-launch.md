# Install to Launch

Date: 2026-04-03

## Objectif

Passer d'une installation locale ou VPS a un lancement lisible, avec reverse proxy pret avant de publier le service.

## Ce que ce document couvre

- preparation du reverse proxy
- verification des variables et des ports
- lancement local avant deploiement
- checks a faire avant exposition publique

## Ce que ce document ne couvre pas

- refonte du model auth
- changement de l'architecture reseau
- installation de la brique media elle-meme

## Preparation du reverse proxy

Si tu prepares Caddy ou un autre reverse proxy avant de deployer:

1. Verifier que le domaine pointe vers le VPS.
2. Preparer le template de config reverse proxy.
3. Valider les ports exposes en entree:
   - 80 pour le challenge et la redirection
   - 443 pour le TLS
4. Verifier que les services internes ne sont pas exposes publiquement.
5. Tester la configuration en mode simulation si possible.

Commandes utiles:

```bash
make caddy-install ARGS='--dry-run'
```

## Lancement local avant deploiement

Avant de toucher au VPS, valider le socle localement:

```bash
make launch-all
curl http://localhost:8080/health
```

Puis verifier:

- `http://localhost:3000/` pour le frontend
- `http://localhost:3000/dashboard` pour le dashboard op
- `http://localhost:3000/admin/tdd` pour la page TDD

## Check-list avant deploy

1. `cargo check -p rev0auth-web` passe.
2. `cargo check -p rev0auth-api` passe.
3. `make test` passe.
4. Le reverse proxy pointe vers les bons ports internes.
5. Les secrets sont defines dans l'environnement cible.
6. Les routes sensibles ne sont pas ouvertes directement au public.

## Lancement sur VPS

1. Installer ou valider le reverse proxy.
2. Copier les variables d'environnement attendues.
3. Lancer les services avec les scripts du projet.
4. Tester le healthcheck et les pages publiques.
5. Confirmer que le domaine sert bien le frontend et l'API attendue.

## Verification minimale apres lancement

- page d'accueil accessible
- dashboard accessible
- healthcheck API OK
- aucun 5xx sur les routes de base

## Liens utiles

- [Operations Index](operations/README.md)
- [Caddy + DuckDNS (pour les nuls)](caddy-duckdns-beginners.md)
- [AUTH-009 VPS setup](auth-009-vps-setup.md)
- [AUTH-012 Media install-only](auth-012-media-install.md)