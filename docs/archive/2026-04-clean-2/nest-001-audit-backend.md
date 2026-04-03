# NEST-001 - Audit socle backend (priorise)

## Scope

Audit cible sur la base backend avant tout reset frontend.
Perimetre:
- API et modules auth dans `crates/api/src`
- tests d'integration dans `crates/api/tests`
- structure de responsabilites, couplage, et risque de regression

Date: 03 avril 2026

---

## Snapshot rapide

- Volume backend: 3743 lignes sur le scope audite.
- Fichiers les plus lourds:
  - `crates/api/src/auth/handlers.rs` (1047)
  - `crates/api/src/auth/store.rs` (568)
  - `crates/api/tests/integration_tests.rs` (534)
  - `crates/api/src/app.rs` (395)

Conclusion: la base est solide et testee, mais 4 points concentrent la complexite et doivent etre decoupes en priorite.

---

## Findings (ordonnes par severite)

### 1. Critique - mapping d'erreur incoherent sur duplicate email en Postgres

- Emplacement: `crates/api/src/auth/store.rs`
- Observation:
  - en Postgres, `create_user` retourne `Err("email_error")` sur unique violation;
  - le handler signup mappe uniquement `"email_already_exists"` vers HTTP 409;
  - resultat probable: duplicate email en DB peut remonter en 500 au lieu de 409.

Impact:
- comportement incoherent entre backend memory et postgres;
- mauvaise DX/API contract cote client.

Action recommandee:
- normaliser le code d'erreur a `email_already_exists` dans `store.rs`.

---

### 2. Eleve - `handlers.rs` melange trop de responsabilites

- Emplacement: `crates/api/src/auth/handlers.rs`
- Observation:
  - auth flow, validation CSRF, cookie/session, audit, rate-limit, mapping erreurs, helpers et une grosse suite de tests sont regroupes dans un seul fichier.

Impact:
- lecture et maintenance plus lentes;
- risque de regression locale quand on touche une petite partie;
- difficultes a isoler les tests par famille.

Action recommandee:
- decouper en modules:
  - `handlers/signup.rs`
  - `handlers/login.rs`
  - `handlers/refresh.rs`
  - `handlers/csrf.rs`
  - `handlers/errors.rs`
- deplacer les tests handlers vers un dossier dedie par scenario.

---

### 3. Eleve - `store.rs` agrege persistence + conversion + health + audit

- Emplacement: `crates/api/src/auth/store.rs`
- Observation:
  - `AppState` porte memory/postgres, CRUD users, refresh rotation, audit, db health et fonctions utilitaires.

Impact:
- trop de raisons de modifier un meme fichier;
- frontiere metier/persistence peu nette;
- extension future plus couteuse.

Action recommandee:
- separer en sous-couches:
  - `store/users.rs`
  - `store/refresh.rs`
  - `store/audit.rs`
  - `store/health.rs`
  - `store/common.rs` (normalization/conversions)

---

### 4. Moyen - `integration_tests.rs` regroupe DB flow, migrations et perf

- Emplacement: `crates/api/tests/integration_tests.rs`
- Observation:
  - un seul fichier pour plusieurs natures de tests.

Impact:
- execution moins pilotable;
- parallelisation selective plus difficile;
- diagnostic d'echec moins direct.

Action recommandee:
- split par famille:
  - `tests/integration/db_flow.rs`
  - `tests/integration/migrations.rs`
  - `tests/integration/perf.rs`
- conserver les helpers partages dans `tests/integration/support/mod.rs`.

---

### 5. Moyen - duplication de helpers de test entre `app.rs` et `handlers.rs`

- Emplacements:
  - `crates/api/src/app.rs`
  - `crates/api/src/auth/handlers.rs`
- Observation:
  - fonctions utilitaires de bootstrap CSRF/cookies/post JSON presentes en plusieurs endroits.

Impact:
- duplication et divergence possible;
- maintenance de tests plus lourde.

Action recommandee:
- centraliser dans un module test support unique (unit/integration selon besoin).

---

## Priorites d'execution backend (sans toucher au reset frontend)

### P0 - securiser le contrat API

- NEST-001A: corriger le mapping duplicate email Postgres -> 409
- NEST-001B: ajouter test de non-regression duplicate signup avec backend Postgres

### P1 - decouper les points de friction

- NEST-001C: extraire `handlers.rs` en sous-modules
- NEST-001D: extraire `store.rs` en sous-modules metier/persistence

### P2 - accelerer la boucle qualite

- NEST-001E: scinder `integration_tests.rs` par famille
- NEST-001F: centraliser helpers de tests
- NEST-001G: preparer commandes de tests par groupe (unit/integration/perf)

---

## Etat et decision

- NEST-001: DONE (audit termine)
- NEST-002: DEFERRED (frontend reset complet planifie plus tard)

Cette decision est coherente avec l'objectif: renforcer d'abord la base backend pour ensuite accelerer librement sur le frontend.
