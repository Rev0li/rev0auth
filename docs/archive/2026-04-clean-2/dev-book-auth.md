# Livre de developpement - Module Auth Rust

## Pourquoi cette approche

On implemente une route, on teste la chaine, puis on passe a la suivante.
C'est une strategie de reduction de risque: chaque iteration est petite, verifiable, et pedagogique.

## Iteration 1 - Signup (`AUTH-001`)

But:
- route `POST /auth/signup`
- hash mot de passe Argon2id
- creation utilisateur en memoire

Verification:
- email invalide -> `400 invalid_email`
- password trop faible -> `400 weak_password`
- duplicate email -> `409 email_already_exists`

## Iteration 2 - Login (`AUTH-002`)

But:
- route `POST /auth/login`
- verification Argon2id
- emission access + refresh

Verification:
- credentials invalides -> `401 invalid_credentials`
- credentials valides -> `200`

## Iteration 3 - Refresh (`AUTH-003`)

But:
- route `POST /auth/refresh`
- rotation du refresh token

Verification:
- refresh token invalide/expire -> `401 invalid_refresh_token`
- refresh token valide -> `200` + nouveau refresh token
- ancien refresh token ne doit plus fonctionner

## Iteration 4 - Persistence PostgreSQL (`AUTH-005`)

But:
- brancher SQLx + PostgreSQL pour users et refresh tokens
- garder un fallback memoire pour execution locale rapide
- ne pas casser la surface HTTP existante

Verification:
- `DATABASE_URL` defini -> backend PostgreSQL utilise
- `DATABASE_URL` absent -> backend memoire utilise
- test de chaine signup/login/refresh toujours vert

## Guide de lecture du code

1. Router global:
- [crates/api/src/app.rs](crates/api/src/app.rs)

2. Modeles de donnees auth:
- [crates/api/src/auth/models.rs](crates/api/src/auth/models.rs)

3. Hash/verification mot de passe (Argon2id):
- [crates/api/src/auth/password.rs](crates/api/src/auth/password.rs)

4. Emission des tokens JWT:
- [crates/api/src/auth/jwt.rs](crates/api/src/auth/jwt.rs)

5. Store memoire/PostgreSQL + rotation refresh:
- [crates/api/src/auth/store.rs](crates/api/src/auth/store.rs)

6. Handlers HTTP + tests pipeline:
- [crates/api/src/auth/handlers.rs](crates/api/src/auth/handlers.rs)

## Commandes pipeline

```bash
~/.cargo/bin/cargo test -p rev0auth-api
~/.cargo/bin/cargo run -p rev0auth-api
```

## Exemples API

Note: le flux auth courant utilise cookies + CSRF.

Bootstrap CSRF:

```bash
curl -i http://localhost:8080/csrf
```

Signup:

```bash
curl -s http://localhost:8080/auth/signup \
  -H 'content-type: application/json' \
  -H 'Cookie: csrf_token=<TOKEN>' \
  -H 'X-CSRF-Token: <TOKEN>' \
  -d '{"email":"member@example.com","password":"my-strong-password-123"}'
```

Login:

```bash
curl -i http://localhost:8080/auth/login \
  -H 'content-type: application/json' \
  -H 'Cookie: csrf_token=<TOKEN>' \
  -H 'X-CSRF-Token: <TOKEN>' \
  -d '{"email":"member@example.com","password":"my-strong-password-123"}'
```

Refresh:

```bash
curl -i http://localhost:8080/auth/refresh \
  -H 'content-type: application/json' \
  -H 'Cookie: csrf_token=<TOKEN>; refresh_token=<TOKEN>' \
  -H 'X-CSRF-Token: <TOKEN>' \
  -d '{}'
```

## Etape suivante recommandee

- AUTH-008-MIGRATIONS: migrations versionnees + table `_migrations`.
- AUTH-008-PERF: tests charge login/refresh et baseline perfs.

## Rapport ticket

- [docs/auth-005-report.md](docs/auth-005-report.md)
