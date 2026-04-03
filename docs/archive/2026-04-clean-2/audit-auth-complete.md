# Audit Complet - Module Auth (AUTH-001 a AUTH-005)

> Note: ce document est un snapshot historique au jalon AUTH-005.
> Pour l'etat courant (AUTH-006 a AUTH-008-FLOW-DB), voir `docs/tickets-auth.md` et `docs/auth-008-flow-db.md`.

## Resumé etat

- **Tests**: 2 tests passent (signup_login_refresh_pipeline_works, duplicate_signup_returns_conflict)
- **Backend**: Dual in-memory + PostgreSQL via SQLx
- **Routes HTTP**: POST /auth/signup, /auth/login, /auth/refresh
- **Sécurité**: Argon2id, JWT access/refresh tokens, rotation refresh atomique

---

## Structures de donnees (models.rs)

| Nom | Type | Champs | Testable |
|-----|------|--------|----------|
| `Role` | Enum | Member, Admin | ✅ |
| `User` | Struct | id (UUID), email, password_hash, role | ✅ |
| `SignupRequest` | Struct | email, password | ✅ |
| `LoginRequest` | Struct | email, password | ✅ |
| `RefreshRequest` | Struct | refresh_token | ✅ |
| `AuthResponse` | Struct | user_id, email, role, access_token, refresh_token | ✅ |
| `ErrorResponse` | Struct | error: &'static str | ✅ |
| `RefreshSession` | Struct | user_id, email, role, expires_at_epoch | ✅ |

---

## Fonctions - JWT (jwt.rs)

| Fonction | Signature | Statut |
|----------|-----------|--------|
| `TokenService::from_env()` | () -> Self | ✅ En utilisé |
| `TokenService::issue_access_token()` | (&self, &User) -> Result<String, String> | ✅ En test |
| `TokenService::refresh_ttl_secs()` | (&self) -> u64 | ✅ Basique |
| `now_epoch()` | () -> u64 | ✅ Utilitaire |

**Tests recommandes**:
- `test_token_service_from_env_dev()` - valider secret par defaut
- `test_issue_access_token_structure()` - valider format JWT
- `test_issue_access_token_ttl()` - valider expiration 15min
- `test_now_epoch_reasonable()` - timestamp > 1700000000

---

## Fonctions - Hash & Verification (password.rs)

| Fonction | Signature | Statut |
|----------|-----------|--------|
| `hash_password()` | (&str) -> Result<String, String> | ✅ En utilisé |
| `verify_password()` | (&str, &str) -> bool | ✅ En test |

**Tests recommandes**:
- `test_hash_password_produces_valid_argon2()` - validation format
- `test_verify_password_succeeds_with_correct()` - path OK
- `test_verify_password_fails_with_wrong()` - path KO
- `test_verify_password_rejects_corrupted_hash()` - robustesse

---

## Fonctions - State & Persistence (store.rs)

| Fonction | Signature | Statut |
|----------|-----------|--------|
| `AppState::new_in_memory()` | (TokenService) -> Self | ✅ En utilisé |
| `AppState::from_env()` | (TokenService) -> async Result<Self, _> | ✅ En utilisé |
| `AppState::create_user()` | ... -> async Result<User, &'static str> | ✅ En test |
| `AppState::find_user_by_email()` | (&self, &str) -> async Option<User> | ✅ En test |
| `AppState::issue_refresh_for_user()` | (&self, &User) -> async String | ✅ En test |
| `AppState::rotate_refresh_token()` | (&self, &str) -> async Result<(User, String), _> | ✅ En test |
| `normalize_email()` | (&str) -> String | ✅ Utilitaire |
| `generate_refresh_token()` | () -> String | ✅ Utilitaire |
| `role_to_str()` | (&Role) -> &'static str | ✅ Utilitaire |
| `str_to_role()` | (&str) -> Role | ✅ Utilitaire |
| `epoch_to_utc()` | (u64) -> DateTime<Utc> | ✅ Utilitaire |
| `is_unique_violation()` | (&sqlx::Error) -> bool | ✅ Utilitaire |
| `initialize_auth_schema()` | (&PgPool) -> async Result<()> | ✅ Automatique |

**Tests recommandes**:
- `test_create_user_memory()` - verifier sauvegarde memoire
- `test_create_user_postgres()` - verifier sauvegarde DB (needs DATABASE_URL)
- `test_create_user_duplicate_email()` - rejection 409
- `test_find_user_returns_none_for_missing()` - path KO
- `test_issue_refresh_token_generates_unique()` - 64 chars alphanum
- `test_rotate_refresh_deletes_old_issues_new()` - atomicite
- `test_normalize_email_lowercases_and_trims()` - @example.COM -> @example.com
- `test_is_unique_violation_detects_23505()` - detection code erreur PG

---

## Fonctions - Handlers HTTP (handlers.rs)

| Fonction | Route | Codes HTTP | Statut |
|----------|-------|--------------|--------|
| `signup()` | POST /auth/signup | 200, 400, 409, 500 | ✅ En test |
| `login()` | POST /auth/login | 200, 401, 500 | ✅ En test |
| `refresh()` | POST /auth/refresh | 200, 401, 500 | ✅ En test |
| `is_valid_email()` | - | - | ✅ Utilitaire |
| `err()` | - | - | ✅ Utilitaire |

**Tests integres (tests d'integration a etendre)**:
- `signup_login_refresh_pipeline_works()` - ✅ PASSE
- `duplicate_signup_returns_conflict()` - ✅ PASSE

**Tests unitaires recommandes**:
- `test_signup_invalid_email()` - 400
- `test_signup_weak_password()` - 400
- `test_login_missing_user()` - 401
- `test_login_wrong_password()` - 401
- `test_refresh_invalid_token()` - 401
- `test_refresh_token_reuse_rejected()` - 401

---

## Fonctions - App Router (app.rs)

| Fonction | Signature | Statut |
|----------|-----------|--------|
| `build_router()` | () -> async Result<Router> | ✅ En utilisé |
| `build_router_in_memory()` | () -> Router | ✅ En test |
| `router_with_state()` | (AppState) -> Router | ✅ Interne |

---

## Completude par domaine

### Modeles ✅
- User, Role, Request/Response types: COMPLETS
- Edge cases (email normalization, role enum): HANDLES

### Cryptographie ✅
- Hash Argon2id: ✅
- JWT tokens: ✅
- Refresh token rotation: ✅

### Persistance ✅ (Dual backend)
- Memory store: ✅
- PostgreSQL + SQLx: ✅
- Schema creation auto: ✅
- Constraints (unique email): ✅

### HTTP ✅
- Routes signup/login/refresh: ✅
- Codes HTTP corrects: ✅
- JSON payloads: ✅

### Tests ⚠️ (basique, peut s'etendre)
- Pipeline integration: ✅ (2 tests)
- Unitaires par fonction: ⚠️ (a ajouter)
- Database integration tests: ⚠️ (Docker container needed)

---

## Points de robustesse a valider

1. **Email normalization**: `test_normalize_email()` - @example.COM -> @example.com ✅
2. **Password strength**: min 12 chars ✅
3. **Token rotation**: ancien refresh token invalide apres rotation ✅
4. **DB Uniqueness**: constraint email unique en base ✅
5. **Timezone handling**: epoch <-> DateTime<Utc> conversion ✅
6. **Error codes SQL**: detection unique_violation (23505) ✅

---

## Variantes d'env

| Variable | Defaut | Usage |
|----------|--------|-------|
| `DATABASE_URL` | Absent = memoire | PostgreSQL connection |
| `AUTH_JWT_SECRET` | `dev-only-secret-change-me` | JWT encoding key |

---

## Documentation disponible

- [tickets-auth.md](tickets-auth.md) - progression par ticket
- [dev-book-auth.md](dev-book-auth.md) - guide lecture code
- [auth-005-report.md](auth-005-report.md) - detail Auth-005

---

## Prochaines etapes (etat courant)

1. AUTH-008-MIGRATIONS
2. AUTH-008-PERF
3. AUTH-009-VPS-SETUP
4. AUTH-009-SECRETS
5. AUTH-009-HEALTH-CHECKS
