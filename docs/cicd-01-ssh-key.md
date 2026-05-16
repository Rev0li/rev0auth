# Étape 1 — Créer une clé SSH de déploiement

> **Durée estimée** : 5 minutes  
> **Où** : Ta machine de développement (pas le VPS)

---

## Pourquoi une clé SSH dédiée ?

Tu as déjà une clé SSH perso (`~/.ssh/id_ed25519` ou similaire) pour te connecter au VPS.
On ne va **pas** donner cette clé à GitHub — c'est ta clé personnelle.

On crée une clé séparée, uniquement pour le robot GitHub Actions. Comme ça :
- Si la clé est compromise, tu peux la révoquer sans toucher à ta clé perso
- Le robot n'a accès qu'à ce qu'on lui autorise explicitement

---

## La commande

```bash
ssh-keygen -t ed25519 -C "github-actions-deploy" -f ~/.ssh/github_deploy_auth -N ""
```

Décryptage :
- `-t ed25519` : algorithme moderne (plus sûr que RSA)
- `-C "github-actions-deploy"` : commentaire pour identifier la clé
- `-f ~/.ssh/github_deploy_auth` : nom du fichier (évite d'écraser ta clé perso)
- `-N ""` : pas de passphrase (le robot ne peut pas en saisir une)

---

## Ce que tu obtiens

```
~/.ssh/github_deploy_auth        ← clé PRIVÉE  (pour GitHub, jamais partagée autrement)
~/.ssh/github_deploy_auth.pub    ← clé PUBLIQUE (pour le VPS)
```

**Règle simple** : la `.pub` peut être copiée partout. La clé sans `.pub` ne quitte jamais ta machine, sauf vers GitHub Secrets (étape 3).

---

## Vérification

```bash
cat ~/.ssh/github_deploy_auth.pub
```

Tu dois voir une ligne qui commence par `ssh-ed25519 AAAA...` et se termine par `github-actions-deploy`.

---

## Résultat attendu

```
Generating public/private ed25519 key pair.
Your identification has been saved in /home/toi/.ssh/github_deploy_auth
Your public key has been saved in /home/toi/.ssh/github_deploy_auth.pub
```

---

## Étape suivante

➡️ [Étape 2 — Autoriser la clé sur le VPS](cicd-02-vps-authorized.md)
