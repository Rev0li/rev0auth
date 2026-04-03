# AUTH-008-FLOW-DB

## Objectif

Valider la chaine complete signup -> login -> refresh sur PostgreSQL via tests d'integration, en environnement isole (conteneur ephemere).

## Implementation

Fichier tests: `crates/api/tests/integration_tests.rs`

- Fixture `with_test_db()`
  - Lance un conteneur `postgres:16` avec `testcontainers`.
  - Construit un `DATABASE_URL` dynamique sur le port mappe.
  - Initialise `AppState::from_env(...)` pour creation automatique du schema.
  - Execute un reset de state DB avant et apres chaque test.
- Reset DB
  - `TRUNCATE TABLE auth_refresh_tokens, auth_users CASCADE`.

## Tests ajoutes

- `test_signup_saves_to_db()`
  - Cree un utilisateur avec hash Argon2.
  - Verifie l'insert en base avec `COUNT(*)` cible.
- `test_login_finds_user_from_db()`
  - Cree un utilisateur puis le recupere via `find_user_by_email` (case-insensitive).
  - Verifie email normalise + validite du hash mot de passe.
- `test_refresh_rotation_transactional()`
  - Emet un refresh token.
  - Effectue la rotation avec CSRF associe.
  - Verifie: ancien token absent, nouveau token present, cardinalite = 1 token pour l'utilisateur.

## Validation

Commandes executees:

```bash
cargo test -p rev0auth-api --test integration_tests -- --test-threads=1
cargo test -p rev0auth-api -- --test-threads=1
```

Resultats:

- Integration DB: 5/5 tests OK.
- Suite complete crate API: OK (unit + integration + doc tests).
