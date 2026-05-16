# Étape 5 — Workflow GitHub Actions

> **Durée estimée** : 5 minutes  
> **Où** : Ta machine de dev

---

## Ce que fait le workflow

```
git push main
     ↓
GitHub Actions démarre un runner (ubuntu)
     ↓
SSH sur le VPS (port VPS_PORT)
     ↓
git pull origin main   ← met à jour le code
     ↓
docker compose up -d --no-deps --build api web
     ↑                  ↑
     pas de restart      rebuild uniquement api et web
     des dépendances     postgres n'est PAS touché
```

---

## Le fichier `.github/workflows/deploy.yml`

```yaml
name: Deploy to VPS

on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest

    steps:
      - name: Deploy via SSH
        uses: appleboy/ssh-action@v1
        with:
          host: ${{ secrets.VPS_HOST }}
          username: ${{ secrets.VPS_USER }}
          key: ${{ secrets.VPS_SSH_KEY }}
          port: ${{ secrets.VPS_PORT }}
          script: |
            cd ~/app/auth
            git pull origin main
            docker compose up -d --no-deps --build api web
```

### Décryptage ligne par ligne

| Ligne | Rôle |
|---|---|
| `on: push: branches: [main]` | Se déclenche uniquement sur push vers main |
| `runs-on: ubuntu-latest` | GitHub fournit une VM Ubuntu temporaire |
| `appleboy/ssh-action@v1` | Action open-source qui gère la connexion SSH |
| `${{ secrets.VPS_PORT }}` | Injecte le secret GitHub (jamais visible dans les logs) |
| `git pull origin main` | Met à jour le code sur le VPS |
| `--no-deps` | Ne redémarre pas postgres |
| `--build` | Recompile l'image Docker (prend en compte les changements Rust) |

---

## Les 4 secrets requis

| Secret GitHub | Rôle |
|---|---|
| `VPS_HOST` | IP du VPS |
| `VPS_USER` | User SSH |
| `VPS_SSH_KEY` | Clé privée `github_deploy_auth` |
| `VPS_PORT` | Port SSH (ex: 4991) |

> ⚠️ **Piège fréquent** : oublier `VPS_PORT`. Sans lui, la connexion SSH bloque
> en silence et le workflow timeout après plusieurs minutes.

---

## Suivre un déploiement

Sur GitHub → onglet **Actions** → clique sur le dernier workflow run.

Tu verras les logs en temps réel. Un déploiement Rust prend ~2-3 minutes
(compilation de l'image Docker).

---

## Ce qui est touché / pas touché

| Élément | Touché ? |
|---|---|
| Code Rust, templates, docker-compose | ✅ Oui (git pull) |
| `.env` | ❌ Non (gitignored) |
| `.secrets` | ❌ Non (gitignored) |
| Conteneur `postgres` | ❌ Non (`--no-deps`) |
| Volume `pgdata` (données) | ❌ Non |
