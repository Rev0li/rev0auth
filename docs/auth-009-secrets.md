# AUTH-009-SECRETS

Date: 2026-04-03

## Objectif

Gerer `AUTH_JWT_SECRET` et `DATABASE_URL` de facon securisee, avec comportement explicite au demarrage.

## Regles appliquees

- `AUTH_JWT_SECRET` est requis pour le demarrage production (`build_router`).
- `DATABASE_URL` reste optionnel:
  - present -> backend `postgres`
  - absent -> backend `memory`

## Implementation

- Chargeur strict secret JWT:
  - `TokenService::from_env_required()`
- Router production:
  - `build_router()` utilise le chargeur strict
- Fallback developpement/tests conserve:
  - `TokenService::from_env()` (secret dev par defaut)

## Variables d'environnement

Template projet:

```env
AUTH_JWT_SECRET=replace-with-32-bytes-minimum-secret
DATABASE_URL=postgres://postgres:change-me-strong-password@postgres:5432/rev0auth
```

Bonnes pratiques:
- ne jamais commit `.env` avec vraies valeurs
- stocker les secrets dans Vault/SSM/K8s Secret selon l'environnement
- permissions strictes sur les fichiers de secrets (`600`)

## Tests

- `test_startup_fails_without_jwt_secret()`
- `test_database_url_optional()`

## Validation executee

```bash
cargo test -p rev0auth-api test_startup_fails_without_jwt_secret -- --test-threads=1
cargo test -p rev0auth-api test_database_url_optional -- --test-threads=1
cargo test -p rev0auth-api -- --test-threads=1
```

Resultat:
- 48 tests unitaires OK
- 10 tests integration OK

## Definition of done

- demarrage refuse si `AUTH_JWT_SECRET` absent
- fallback memoire si `DATABASE_URL` absent
- logs backend existants (`memory` vs `postgres`) conserves
- documentation dediee disponible
