# Étape 2 — Autoriser la clé SSH sur le VPS

> **Durée estimée** : 10 minutes  
> **Où** : Ta machine de dev + VPS (SSH)

---

## Contexte

On a deux directions de connexion SSH à configurer :

```
GitHub Actions ──SSH──> VPS       (clé github_deploy_auth — étape 2A)
VPS            ──SSH──> GitHub    (clé github_rev0auth    — étape 2B)
```

---

## 2A — Autoriser GitHub Actions à se connecter au VPS

Depuis ta machine de dev :

```bash
cat ~/.ssh/github_deploy_auth.pub | ssh -p <PORT> revovps@<IP> "cat >> ~/.ssh/authorized_keys"
```

> ⚠️ **Piège fréquent** : si ton VPS utilise un port SSH non standard (ex: 4991),
> il faut le préciser avec `-p <PORT>` dans toutes les commandes ssh/scp/rsync.
> Sans ça, la connexion bloque indéfiniment sans message d'erreur.

### Vérification

```bash
ssh -i ~/.ssh/github_deploy_auth -p <PORT> revovps@<IP> "echo OK"
# → OK  (sans mot de passe)
```

---

## 2B — Autoriser le VPS à cloner depuis GitHub

Le VPS a besoin de sa propre clé SSH pour faire `git clone` / `git pull` sur GitHub.

### Générer une clé sur le VPS

```bash
ssh-keygen -t ed25519 -C "vps-rev0auth-deploy" -f ~/.ssh/github_rev0auth -N ""
```

### Afficher la clé publique à copier

```bash
cat ~/.ssh/github_rev0auth.pub
```

### Ajouter comme Deploy Key sur GitHub

`https://github.com/Rev0li/rev0auth` → **Settings → Deploy keys → Add deploy key**

| Champ | Valeur |
|---|---|
| Title | `vps-ovh` |
| Key | contenu de `github_rev0auth.pub` |
| Allow write access | ❌ Non (lecture seule suffit) |

### Configurer le VPS pour utiliser cette clé avec GitHub

```bash
echo "Host github.com
  IdentityFile ~/.ssh/github_rev0auth
  IdentitiesOnly yes" >> ~/.ssh/config
```

### Vérification

```bash
ssh -T git@github.com
# → Hi Rev0li! You've successfully authenticated...
```

---

## Récapitulatif des clés

| Clé | Créée où | Utilisée pour |
|---|---|---|
| `~/.ssh/github_deploy_auth` | Machine de dev | GitHub Actions → VPS |
| `~/.ssh/github_rev0auth` | VPS | VPS → GitHub (git pull) |

---

## Étape suivante

➡️ [Étape 3 — Secrets GitHub](cicd-03-github-secrets.md)
