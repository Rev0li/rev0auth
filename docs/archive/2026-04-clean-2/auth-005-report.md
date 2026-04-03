# AUTH-005 Report - PostgreSQL + SQLx

> Note: ce rapport documente le jalon AUTH-005.
> Le flux auth courant a evolue ensuite (cookies + CSRF + audit), voir `docs/tickets-auth.md`.

## Objectif livre

Ticket AUTH-005 migre la persistence Auth vers PostgreSQL sans changer les routes HTTP.
Le mode memoire est conserve pour les tests rapides et le developpement local.

## Nouvelles fonctions

- AppState::new_in_memory(token_service: TokenService) -> Self
- AppState::from_env(token_service: TokenService) -> anyhow::Result<Self>
- AppState::create_user(email, password_hash, role) -> Result<User, &'static str>
- AppState::find_user_by_email(email) -> Option<User>
- AppState::issue_refresh_for_user(user) -> String
- AppState::rotate_refresh_token(current_token) -> Result<(User, String), &'static str>
- normalize_email(email) -> String
- generate_refresh_token() -> String
- role_to_str(role) -> &'static str
- str_to_role(role) -> Role
- epoch_to_utc(epoch) -> DateTime<Utc>
- is_unique_violation(err) -> bool
- initialize_auth_schema(pool) -> anyhow::Result<()>
- build_router() -> anyhow::Result<Router>
- build_router_in_memory() -> Router
- router_with_state(state) -> Router

## Nouvelles variables/structures

- AppState.backend: AuthBackend
- AppState.token_service: TokenService
- AuthBackend::Memory(MemoryStore)
- AuthBackend::Postgres(PgPool)
- MemoryStore.users_by_email
- MemoryStore.refresh_tokens
- DATABASE_URL (variable d'environnement)

## Test de chaine

Test existant valide toujours la chaine complete:
- signup -> login -> refresh -> ancien refresh refuse

Commande:

```bash
~/.cargo/bin/cargo test -p rev0auth-api
```

## Test individuel recommande

1. Mode memoire (sans DATABASE_URL)

```bash
unset DATABASE_URL
~/.cargo/bin/cargo test -p rev0auth-api
```

2. Mode PostgreSQL (avec DB locale)

```bash
export DATABASE_URL='postgres://postgres:postgres@localhost:5432/rev0auth'
~/.cargo/bin/cargo run -p rev0auth-api
```

Puis tester les routes:
- POST /auth/signup
- POST /auth/login
- POST /auth/refresh

## Solidite code

- Surface HTTP inchangée: routes et payloads conserves
- Rotation refresh token atomique cote PostgreSQL via transaction
- Contrainte d'unicite email enforcee en base
- Fallback memoire pour executer les tests sans dependance externe
