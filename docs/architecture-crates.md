# Architecture — crates Rust

Référence structurelle pour le crate `rev0auth-api`. Mis à jour : 2026-05-26.

---

## Vue d'ensemble

```
crates/api/
├── src/
│   ├── lib.rs              — exports: pub mod app; pub mod auth;
│   ├── main.rs             — point d'entrée, bind TCP, appelle app::build_router()
│   ├── app/                — PROTOTYPE (actuellement en production)
│   │   ├── domain.rs       — AppState in-memory, modèles, AppError, AppJson extractor
│   │   ├── handlers.rs     — routes: /health /auth/* /admin/panel
│   │   ├── services.rs     — helpers: normalize_email, issue_token, authenticated_user
│   │   └── tests.rs        — tests d'intégration du prototype
│   └── auth/               — IMPLEMENTATION CIBLE (actuellement en test seulement)
│       ├── mod.rs           — exports + build_router_in_memory() (#[cfg(test)])
│       ├── models.rs        — Role, User, RefreshSession, request/response types
│       ├── store.rs         — AppState avec backend dual (Memory | Postgres)
│       ├── jwt.rs           — TokenService: issue/verify access tokens HS256
│       ├── extractor.rs     — UserClaims: FromRequestParts, vérifie JWT sur chaque requête
│       ├── handlers.rs      — /csrf /auth/* /admin/panel /admin/audit-logs
│       ├── password.rs      — hash_password / verify_password (argon2 + OsRng)
│       ├── cookies.rs       — build_access/refresh/csrf_cookie, extract_cookie
│       ├── rbac.rs          — RequiredRole, RoleGuard extractor, require_member/admin middleware
│       ├── rate_limit.rs    — LoginRateLimiter: lockout progressif 15m → 1h → 24h
│       ├── audit.rs         — AuditEvent, AuditEventType, client_ip_from_headers
│       └── migrations.rs    — run_pending_migrations() via sqlx embed
└── migrations/
    ├── 0001_auth_schema.sql     — auth_users, auth_refresh_tokens
    ├── 0002_audit_logs_table.sql
    ├── 0003_indexes_optimization.sql — + colonne csrf_token sur refresh_tokens
    ├── 0004_web_state.sql
    ├── 0005_member_approved.sql
    └── 0006_invites.sql
```

---

## Module `app/` — prototype en production

**Statut** : actuellement servi par `main.rs`. Conçu comme scaffold, jamais remplacé.

### AppState (domain.rs)
```
users_by_email  : Arc<RwLock<HashMap<String, UserRecord>>>
refresh_tokens  : Arc<RwLock<HashMap<String, u64>>>   — token → user_id
access_tokens   : Arc<RwLock<HashMap<String, u64>>>   — token → user_id
next_user_id    : Arc<RwLock<u64>>                    — compteur séquentiel
```

Entièrement in-memory. Pas de PostgreSQL. IDs entiers séquentiels (pas UUID).

### Tokens (services.rs)
- Access token : format `access-{user_id}-{counter}` — **pas JWT**
- Refresh token : format `refresh-{user_id}-{counter}`
- Stockés en HashMap sans TTL (pas d'expiration)
- Rôle admin : hardcodé `if email == "admin@example.com"`

### Routes exposées
```
GET  /health
POST /auth/signup
POST /auth/login
POST /auth/refresh
GET  /auth/me
GET  /admin/panel
```

---

## Module `auth/` — implémentation cible

**Statut** : complet et testé, mais pas encore câblé à `main.rs`. Router disponible seulement via `build_router_in_memory()` dans `#[cfg(test)]`.

### AppState (store.rs)
```
backend         : AuthBackend::Memory(MemoryStore) | AuthBackend::Postgres(PgPool)
token_service   : TokenService
login_rate_limiter : LoginRateLimiter
```

Sélection du backend au démarrage : si `DATABASE_URL` est définie → Postgres, sinon in-memory.

### Flux d'authentification

```
Client                          API
  │── GET /csrf ──────────────► csrf() → génère token, pose cookie csrf_token
  │
  │── POST /auth/login ────────► validate_csrf_headers()
  │   body: {email, password}       └─ compare x-csrf-token header == cookie
  │   headers: x-csrf-token         password::verify_password (argon2)
  │   cookie: csrf_token=...        issue_access_token → JWT HS256 (15min)
  │                                 generate_refresh_token → 64 char alphanumeric
  │◄─ cookies: access_token (HttpOnly,Secure,900s)
  │            refresh_token (HttpOnly,Secure,7j)
  │            csrf_token (Secure, lisible JS)
  │   body: {user_id, email, role, csrf_token}
  │
  │── POST /auth/refresh ──────► validate_csrf_headers()
  │   headers: x-csrf-token         rotate_refresh_token() — DELETE old, INSERT new (transaction)
  │   cookie: csrf_token=...        vérifie csrf_token == stocké
  │            refresh_token=...    vérifie expires_at
  │◄─ nouveaux cookies (rotation complète)
```

### TokenService (jwt.rs)
- Algorithme : HS256
- Secret : `AUTH_JWT_SECRET` (requis en prod, fallback dev sinon)
- TTL access : `AUTH_ACCESS_TTL_SECS` (défaut 900s)
- TTL refresh : `AUTH_REFRESH_TTL_SECS` (défaut 604800s = 7 jours)
- Claims : `sub` (UUID), `email`, `role`, `token_type`, `iat`, `exp`

### UserClaims extractor (extractor.rs)
Lit `Authorization: Bearer <jwt>`, vérifie signature + expiration, injecte `UserClaims` dans le handler. Lu le secret depuis l'env à chaque requête.

### RBAC (rbac.rs)
```
RequiredRole::Member  → Role::Member | Role::Admin
RequiredRole::Admin   → Role::Admin seulement
```
Utilisation :
```rust
.route("/protected", get(handler).route_layer(from_fn(require_member)))
.route("/admin",     get(handler).route_layer(from_fn(require_admin)))
```

### Rate limiter (rate_limit.rs)
Par email normalisé. Paliers déclenchés tous les 5 échecs consécutifs :
```
lock_level 1 → 15 minutes
lock_level 2 → 1 heure
lock_level 3+ → 24 heures
```
Reset complet sur login réussi. **Pas de cleanup automatique** des entrées expirées.

### Routes exposées (auth/)
```
GET  /csrf
POST /auth/signup
POST /auth/login
POST /auth/refresh
GET  /admin/panel
GET  /admin/audit-logs
```

---

## Schéma PostgreSQL

```sql
auth_users (
  id            UUID PRIMARY KEY,
  email         TEXT UNIQUE NOT NULL,
  password_hash TEXT NOT NULL,
  role          TEXT NOT NULL,          -- "member" | "admin"
  created_at    TIMESTAMPTZ DEFAULT NOW()
)

auth_refresh_tokens (
  token       TEXT PRIMARY KEY,
  user_id     UUID REFERENCES auth_users(id) ON DELETE CASCADE,
  csrf_token  TEXT NOT NULL,            -- ajouté migration 0003
  expires_at  TIMESTAMPTZ NOT NULL,
  created_at  TIMESTAMPTZ DEFAULT NOW()
)

auth_audit_logs (
  id              BIGSERIAL PRIMARY KEY,
  timestamp_epoch BIGINT NOT NULL,
  user_id         UUID,
  event_type      TEXT,                 -- create_user | login | refresh | failed_auth
  ip              TEXT,
  created_at      TIMESTAMPTZ DEFAULT NOW()
)
```

---

## Dépendances clés (api/Cargo.toml)

| Crate | Usage |
|---|---|
| `axum` 0.7 | Framework HTTP async, extractors, routing |
| `tokio` | Runtime async, RwLock, TcpListener |
| `sqlx` 0.8 | Queries PostgreSQL compilées, migrations, pool |
| `jsonwebtoken` 9 | JWT HS256 encode/decode |
| `argon2` 0.5 | Hash + verify password (OsRng salt) |
| `uuid` 1 | IDs utilisateurs, v4 + serde |
| `rand` 0.8 | Génération tokens refresh et CSRF (Alphanumeric) |
| `chrono` 0.4 | DateTime<Utc> pour expires_at PostgreSQL |
| `serde` / `serde_json` | Sérialisation JSON |
| `tracing` | Logs structurés (target: rev0auth.auth / rev0auth.audit) |
| `anyhow` | Gestion d'erreurs startup |
