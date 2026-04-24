# Roadmap - firs_stable vers V1

Date: 2026-04-06
Baseline stable: `firs_stable`

## Avancement - 2026-04-10

- Phase 1 (clean structurel): largement executee sur les pages web principales.
  - extraction des styles inline vers modules dedies
  - suppression des `style="..."` restants dans les templates et modules cibles
  - validation compile/test apres lots
- Phase 2 (best security tests): demarree et branchee au pipeline global.
  - `make test` est maintenant un all-test: audit securite + suite tests Rust
  - tests securite API ajoutes (payload malforme, champs manquants, bearer forge)
  - 2FA admin TOTP ajoute (optionnel, active via `ADMIN_DASH_TOTP_SECRET`)
- Ops/dev ergonomie:
  - chargement automatique de `.env` sur `make launch-all` et `make test`

Commits de reference:
- `e8d2ce8` - nettoyage styles inline web
- `57e9c58` - all-test securite + 2FA TOTP + integration `.env`

## Objectif

Partir d'une base stable, reduire la surface de code, renforcer la securite, puis livrer une V1 visuelle propre.

## Phase 0 - Baseline stable (immediat)

- Tagger le commit stable en `firs_stable`.
- Verrouiller la verification minimale:
  - `cargo check -p rev0auth-web`
  - `make test`
- Interdire les gros commits melanges.

## Phase 1 - Clean structurel (micro-refactor)

Objectif: fichiers plus petits, modules plus simples, moins de duplication.

- Extraire progressivement les blocs inline de:
  - `crates/web/src/pages/dashboard.rs`
  - `crates/web/src/pages/friend.rs`
  - `crates/web/src/pages/profile.rs`
- Centraliser les constantes frontend partagees:
  - keys localStorage
  - noms d'evenements
  - helpers UI communs
- Regle: 1 module = 1 responsabilite.
- Validation a chaque lot:
  - `cargo check -p rev0auth-web`
  - `make test`

## Phase 2 - Best security tests

Objectif: augmenter la confiance sur les chemins d'attaque.

- Ajouter tests negatifs et d'abus:
  - token forge / signature invalide
  - replay refresh token
  - bypass RBAC membre -> admin
  - CSRF absent/invalide sur mutations
  - payloads malformes / champs manquants
- Structurer les tests par groupe:
  - unit (rapides)
  - integration (db/http)
  - security (abuse cases)
- Ajouter commandes de run dediees dans `Makefile` ou scripts.

Etat actuel (2026-04-10):
- [x] commandes de run securite integrees au flux `make test`
- [x] premiers tests de securite API integres
- [ ] completer la matrice abuse cases (replay approfondi, CSRF mutation coverage complet, escalation scenario end-to-end)

## Phase 3 - Hardening securite

- Uniformiser les reponses d'erreur (pas de fuite sensible).
- Verifier limites upload (taille, mime, extension).
- Renforcer journalisation des actions sensibles.
- Ajouter checklist de verification securite pre-release.

## Phase 4 - V1 visuelle reelle

- Definir une direction visuelle unique (tokens + composants).
- Stabiliser 3 ecrans coeur:
  - home
  - dashboard
  - profile
- Supprimer le style duplique restant dans les pages shell.
- Ajouter smoke tests UI minimum (navigation + actions critiques).

## Definition of done V1

- Build et tests verts sur API + web.
- Dossiers docs ranges, index a jour.
- Fichiers pages coeur clairement modularises.
- Pack tests securite execute sans echec.
- Parcours utilisateur principal valide de bout en bout.

## Strategie de livraison

1. `firs_stable` (tag baseline)
2. `v1-clean-alpha` (clean structurel termine)
3. `v1-security-beta` (tests securite et hardening)
4. `v1.0.0` (visuel + stabilite finale)
