# GitHub + Git Cheatsheet (pratique)

Objectif: aller vite, rester propre, et eviter les pertes de travail.

## 1) Flux quotidien (simple et fiable)

```bash
# partir de ta branche
git status
git pull --rebase origin <ta-branche>

# travailler
git add -A
git commit -m "feat(web): ..."
git push -u origin HEAD
```

Notes:
- `git push -u origin HEAD` cree le tracking de la branche en une commande.
- `--rebase` garde un historique plus lisible que des merges locaux repetes.

## 2) Branches utiles

```bash
# creer / changer
git switch -c feature/nom-court
git switch feature/nom-court

# voir tracking local <-> origin
git branch -vv

# push toutes les branches locales
git push --all origin
```

## 3) Tags (moins connu mais tres utile)

Utilise les tags pour marquer une release stable (ex: V1).

```bash
# tag annote (recommande)
git tag -a v1.0.0 -m "V1 stable"

# pousser un tag
git push origin v1.0.0

# pousser tous les tags
git push --tags origin

# lister / voir
git tag -n
git show v1.0.0
```

Bonnes pratiques:
- prefere les tags annotes (`-a`) aux tags lightweight.
- nommage coherent: `v1.0.0`, `v1.1.0`, `v1.1.1`.

## 4) Fonctions "moins connues" qui sauvent du temps

### a) `git worktree` (2 branches ouvertes en meme temps)

```bash
git worktree add ../rev0auth-hotfix hotfix/login-redirect
```

Tu peux coder une feature et un hotfix en parallele, sans stash permanent.

### b) `git reflog` (recuperer un commit "perdu")

```bash
git reflog
git checkout <hash>
```

Tres utile apres un rebase ou reset rate.

### c) `git commit --fixup` + autosquash

```bash
git commit --fixup <hash-commit-a-corriger>
git rebase -i --autosquash origin/<ta-branche>
```

Permet de nettoyer l'historique avant merge.

### d) `git range-diff` (comparer 2 series de commits)

```bash
git range-diff origin/main...HEAD~5 origin/main...HEAD
```

Ideal pour verifier ce qui a change apres rebase.

### e) `git rerere` (memorise la resolution de conflits)

```bash
git config --global rerere.enabled true
```

Quand le meme conflit revient, Git peut reappliquer ta resolution.

## 5) Stash propre

```bash
# stash nomme
git stash push -m "wip-chat-ui"

# stash d'un fichier seulement
git stash push -m "wip-dashboard" -- crates/web/src/pages/dashboard.rs

git stash list
git stash pop
```

## 6) Pull Requests GitHub (efficace)

Checklist PR courte:
- titre clair (`feat(...)`, `fix(...)`)
- description: probleme -> solution -> impact
- screenshots si UI
- test local minimal (`cargo check -p rev0auth-web`)

Keywords utiles dans la PR:
- `Closes #123`
- `Fixes #123`

Fonctions GitHub utiles:
- Draft PR pour WIP
- Auto-merge apres checks
- Squash merge pour garder un historique lisible
- Labels (`frontend`, `auth`, `ops`)
- Milestones pour grouper la roadmap (ex: `V1`)

## 7) Releases GitHub (a faire pour V1)

1. Tagger la version (`v1.0.0`).
2. Push tag.
3. GitHub -> Releases -> "Draft a new release".
4. Selectionner le tag, notes de release, publier.

Astuce:
- active "Generate release notes" pour un premier draft automatique.

## 8) Raccourcis GitHub UI

- Appuie sur `?` dans GitHub pour voir les raccourcis clavier.
- Recherche puissante:
  - `is:pr is:open author:@me`
  - `is:issue is:open label:bug`
  - `repo:Rev0li/auth-selfhost-rust is:pr review-requested:@me`

## 9) Recuperation rapide (panique mode)

```bash
# commit disparu / HEAD bouge
git reflog

# annuler un commit sans re-ecrire l'historique distant
git revert <hash>

# supprimer une branche locale deja mergee
git branch -d feature/old
```

## 10) Regles d'or

- commit petit, message clair, push regulier.
- evite `git push --force` sauf cas maitrise.
- prefere `--force-with-lease` si force push obligatoire.
- tag chaque jalon important (beta, rc, v1).