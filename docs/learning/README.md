# Learning - Workflow commits

Objectif: transformer le developpement en parcours de formation lisible par commit.

Voir aussi: `docs/learning/BookOfDev.md`

## Strategie Git validee

- Historique actuel conserve.
- Branche learning parallele pour raconter l'histoire de dev proprement.
- Granularite commit: balanced (1 commit = 1 etape/ticket court).

## Format commit conseille

- `STEP-000: bootstrap backend minimal`
- `STEP-001: signup route`
- `STEP-002: signup tests`
- `STEP-003: login route`
- `STEP-004: login tests`
- `STEP-005: refresh route`
- `STEP-006: refresh tests`
- etc.

## Regles

1. Chaque commit doit compiler.
2. Chaque commit doit avoir une intention unique.
3. Les tests associes doivent etre inclus dans la meme etape ou l'etape suivante immediate.
4. Message commit court + explicite.

## Prochaine mise en place

- Ajouter hooks `pre-commit` et `pre-push` apres reorganisation docs (decision utilisateur).

## Etat courant

- `feature/step-008-rbac` contient maintenant un decoupage plus modulaire de l'API dans `crates/api/src/app/`.
- Le commit cible doit rester aligne avec cette phase: routes dans `handlers`, logique partagee dans `services`, types et state dans `domain`, tests dans `tests`.
- Le prochain jalon doit conserver le lien avec `docs/README.md`, `docs/Next-Work.md` et `docs/roadmap-detailed.md` pour garder l'histoire lisible.
