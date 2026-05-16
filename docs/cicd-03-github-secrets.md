# Étape 3 — Secrets GitHub

> **Durée estimée** : 5 minutes  
> **Où** : Navigateur (GitHub) + machine de dev (pour copier la clé privée)

---

## Pourquoi des secrets GitHub ?

Le workflow GitHub Actions a besoin de se connecter au VPS.
Pour ça il lui faut : l'IP, le user, la clé SSH privée, et le port.

Ces valeurs ne doivent **jamais** apparaître dans le code — GitHub les chiffre et
les injecte uniquement au moment de l'exécution du workflow. Elles ne sont jamais
visibles dans les logs.

---

## Où les ajouter

`https://github.com/Rev0li/rev0auth`
→ **Settings → Secrets and variables → Actions → New repository secret**

---

## Les 4 secrets à créer

| Nom | Valeur | Comment l'obtenir |
|---|---|---|
| `VPS_SSH_KEY` | Clé privée entière | `cat ~/.ssh/github_deploy_auth` sur ta machine de dev |
| `VPS_HOST` | IP du VPS | ex: `94.23.107.22` |
| `VPS_USER` | User SSH | ex: `revovps` |
| `VPS_PORT` | Port SSH | ex: `4991` |

> ⚠️ **Piège fréquent** : oublier `VPS_PORT` si ton VPS n'est pas sur le port 22.
> Sans ça, le workflow GitHub Actions bloque en silence sur la connexion SSH.

### Copier la clé privée

```bash
cat ~/.ssh/github_deploy_auth
```

Copie **tout** le contenu, de `-----BEGIN OpenSSH PRIVATE KEY-----`
jusqu'à `-----END OpenSSH PRIVATE KEY-----` inclus.

---

## Vérification

Dans GitHub → **Settings → Secrets and variables → Actions**, tu dois voir les 4 secrets listés (les valeurs sont masquées, c'est normal).

---

## Étape suivante

➡️ [Étape 4 — Préparer le VPS](cicd-04-vps-git-setup.md)
