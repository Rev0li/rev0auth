# CI/CD — Roadmap de déploiement automatique

> **Objectif** : Quand tu fais `git push` sur `main`, le code se déploie automatiquement
> sur le VPS. La base de données et les fichiers secrets ne sont jamais touchés.

---

## Vue d'ensemble

```
[Ta machine]          [GitHub]              [VPS OVH]
    │                    │                      │
    │── git push main ──>│                      │
    │                    │── SSH + git pull ────>│
    │                    │                      │── docker compose up
    │                    │                      │   (postgres reste ON)
    │                    │                      │── api  ← rebuild
    │                    │                      │── web  ← rebuild
```

### Ce qui est touché lors d'un déploiement
| Élément | Touché ? | Pourquoi |
|---|---|---|
| Code (Rust, templates) | ✅ Oui | C'est le but |
| `docker-compose.yml` | ✅ Oui | Suivi par git |
| `.env` | ❌ Non | Gitignored, reste sur le VPS |
| `.secrets` | ❌ Non | Gitignored, reste sur le VPS |
| `pgdata` (volume Docker) | ❌ Non | Volume Docker indépendant |
| Postgres (conteneur) | ❌ Non | `--no-deps` dans la commande |

---

## Les 5 étapes

| # | Étape | Où ça se passe | Doc |
|---|---|---|---|
| 1 | Créer une clé SSH de déploiement | Ta machine de dev | [cicd-01-ssh-key.md](cicd-01-ssh-key.md) |
| 2 | Autoriser la clé sur le VPS | VPS (SSH) | [cicd-02-vps-authorized.md](cicd-02-vps-authorized.md) |
| 3 | Ajouter les secrets dans GitHub | GitHub (navigateur) | [cicd-03-github-secrets.md](cicd-03-github-secrets.md) |
| 4 | Préparer le VPS (passer à git) | VPS (SSH) | [cicd-04-vps-git-setup.md](cicd-04-vps-git-setup.md) |
| 5 | Créer le workflow GitHub Actions | Ta machine de dev | [cicd-05-workflow.md](cicd-05-workflow.md) |

---

## Vocabulaire de base

- **CI/CD** : *Continuous Integration / Continuous Deployment* — automatiser le déploiement
- **GitHub Actions** : robot de GitHub qui exécute des scripts quand tu pousses du code
- **Clé SSH de déploiement** : clé SSH dédiée uniquement à ce robot (pas ta clé perso)
- **GitHub Secret** : variable chiffrée stockée sur GitHub, jamais visible dans les logs
- `--no-deps` : option Docker qui dit "redémarre ce service, mais pas ses dépendances"

---

## Prérequis

- [ ] Repo GitHub existant : `git@github.com:Rev0li/auth-selfhost-rust.git` ✅
- [ ] VPS accessible en SSH : `revovps@94.23.107.22` ✅
- [ ] `.env` et `.secrets` déjà en place sur le VPS ✅
- [ ] Docker Compose fonctionnel sur le VPS ✅
