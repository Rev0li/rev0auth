# Audit Rust — rev0auth-api

Audit du crate `crates/api/`. Date : 2026-05-26.
Référence architecture : [`architecture-crates.md`](./architecture-crates.md).

---

## Résumé exécutif

La codebase contient **deux implémentations parallèles** du même service :

| Module | Statut prod | Sécurité | PostgreSQL |
|---|---|---|---|
| `app/` | **En production** (via `main.rs`) | Prototype | Non |
| `auth/` | Tests seulement | Complète | Oui |

Le module `auth/` — avec JWT, CSRF, rate limiting, audit logs, RBAC middleware — est complet et bien testé mais **n'est pas encore câblé à `main.rs`**. La prochaine étape prioritaire est de remplacer `app::build_router()` par `auth::build_router()` dans `main.rs`.

---

## Problèmes critiques

### 1. `main.rs` utilise le prototype, pas l'implémentation sécurisée

`main.rs:13` appelle `app::build_router()`. Ce router n'a ni CSRF, ni JWT, ni rate limiting, ni audit.

Ce qui est en production aujourd'hui :
- Tokens = strings opaques `access-{id}-{counter}` stockés en HashMap (pas JWT)
- Pas de CSRF
- Pas de rate limiting sur `/auth/login`
- Pas d'audit logs
- Admin déterminé par hardcode email : `admin@example.com` (`app/services.rs:12`)
- IDs utilisateurs = entiers séquentiels (pas UUID)
- Tokens sans TTL (HashMap infini)

**Action requise** : câbler `auth/` dans `main.rs` et supprimer `app/`.

---

### 2. Rôle admin hardcodé dans `app/services.rs`

```rust
// app/services.rs:12
pub(crate) fn role_for_email(email: &str) -> Role {
    if email == "admin@example.com" {
        Role::Admin
    } else {
        Role::Member
    }
}
```

N'importe qui peut s'inscrire avec `admin@example.com` et devenir admin.
Valide uniquement en dev, inacceptable en production.

---

### 3. `extractor.rs` lit le secret JWT depuis l'env à chaque requête

```rust
// auth/extractor.rs:41
let secret = std::env::var("AUTH_JWT_SECRET")
    .unwrap_or_else(|_| "dev-only-secret-change-me".to_string());
```

`std::env::var` est lente et non nécessaire à chaque requête. Le secret est déjà dans `TokenService` au démarrage. L'extractor devrait recevoir une `DecodingKey` depuis l'état partagé.

**Fix** : injecter la `DecodingKey` dans `AppState` ou lire le secret une fois au boot et le passer en paramètre à l'extractor.

---

### 4. Rate limiter sans cleanup mémoire (`rate_limit.rs`)

```rust
entries: Arc<RwLock<HashMap<String, LoginAttemptState>>>
```

Les entrées ne sont jamais supprimées sauf sur `record_success()`. Sur un serveur longue durée avec du spam, ce HashMap grandit indéfiniment.

**Fix simple** : ajouter un timestamp `last_seen` sur `LoginAttemptState` et nettoyer via une tâche `tokio::spawn` périodique (ex: toutes les heures, supprimer les entrées sans lock actif depuis > 24h).

---

### 5. Email validation trop permissive

```rust
// app/services.rs:8 et auth/handlers.rs:254
fn is_valid_email(email: &str) -> bool {
    email.contains('@') && email.len() >= 5
}
```

`fo@ba` ou `a@.b` passent cette validation. Pas de vérification du domaine, pas de RFC 5321.

**Fix** : crate [`email_address`](https://docs.rs/email_address/latest/email_address/) — 30 min de travail, zero dépendances lourdes.
```toml
email_address = "0.2"
```
```rust
use email_address::EmailAddress;
EmailAddress::is_valid(email)
```

---

### 6. `cookies.rs` — string formatting fragile

```rust
// auth/cookies.rs:34
format!("{}={}; HttpOnly; Secure; SameSite=Lax; Path=/; Max-Age=900", ...)
```

Une typo dans le format string passe à la compilation et casse silencieusement les cookies. `HeaderValue::from_str()` peut paniquer si le token contient des caractères invalides.

**Fix** : crate [`cookie`](https://docs.rs/cookie/latest/cookie/) (déjà dépendance transitive via axum).
```rust
use cookie::{Cookie, SameSite};
use time::Duration;

Cookie::build(("access_token", token))
    .http_only(true)
    .secure(true)
    .same_site(SameSite::Lax)
    .path("/")
    .max_age(Duration::seconds(900))
    .build()
```

---

## Ce qui est bien fait — ne pas toucher

### `auth/password.rs`
Utilisation correcte d'`argon2` avec `OsRng`. Rien à changer.
```rust
Argon2::default().hash_password(password.as_bytes(), &SaltString::generate(&mut OsRng))
```

### `auth/jwt.rs`
Wrapper propre autour de `jsonwebtoken`. Configuration TTL via env vars. Tests complets couvrant expiration, mauvais secret, format 3 segments.

### `auth/rbac.rs`
RBAC minimal via `FromRequestParts` Axum-natif. Hiérarchie admin ≥ member claire. Tests couvrent : member → /member ✓, member → /admin ✗, admin → /member ✓, admin → /admin ✓.

### `auth/store.rs` — rotation refresh tokens
La rotation utilise une transaction PostgreSQL : DELETE old + INSERT new dans la même tx, avec vérification `csrf_token` et `expires_at`. Replay du token précédent correctement rejeté. Tests end-to-end sur 2 générations.

### `auth/handlers.rs` — CSRF double-submit cookie
Implémentation correcte du pattern double-submit :
1. `GET /csrf` génère le token, le pose en cookie
2. Les mutations exigent `x-csrf-token` header == valeur du cookie `csrf_token`

### Tests — couverture sécurité
Le module `auth/handlers.rs` contient une matrice de tests sécurité exhaustive :
- Replay tokens (1 et 2 générations)
- CSRF : absent, mismatché, header seul, cookie seul
- RBAC : member → admin escalation, alg:none forgé
- Rate limit : 5 échecs → 429, reset sur succès
- Cookies : flags HttpOnly/Secure/SameSite vérifiés

---

## Dépendances — évaluation

| Crate | Verdict | Note |
|---|---|---|
| `argon2` | Garder | Correct, bien utilisé |
| `jsonwebtoken` | Garder | Bien utilisé |
| `sqlx` | Garder | Raw SQL, pas d'ORM — bon choix |
| `uuid` | Garder | v4 + serde |
| `rand` | Garder | Alphanumeric pour tokens 64 chars |
| `chrono` | Garder | Nécessaire pour sqlx DateTime<Utc> |
| `axum` | Garder | Framework principal |
| `tokio` | Garder | Runtime |
| `tracing` | Garder | Logs structurés en place |
| `anyhow` | Garder | Limité au startup, approprié |
| `cookie` | Ajouter | Remplacer string formatting dans cookies.rs |
| `email_address` | Ajouter | Remplacer validation `contains('@')` |

---

## Plan de migration — priorités

### Priorité 1 — Câbler `auth/` en production

Modifier `main.rs` pour utiliser `auth/` :

```rust
// main.rs — aujourd'hui
use rev0auth_api::app::build_router;

// main.rs — objectif
use rev0auth_api::auth::AppState;
use rev0auth_api::auth::jwt::TokenService;
// + construire le router auth avec AppState::from_env()
```

Implique de sortir `build_router_in_memory` du `#[cfg(test)]` et d'ajouter `build_router()` public dans `auth/mod.rs`.

**Bloquant pour la sécurité en production.**

### Priorité 2 — Supprimer `app/`

Une fois `auth/` en production, supprimer :
- `crates/api/src/app/domain.rs`
- `crates/api/src/app/handlers.rs`
- `crates/api/src/app/services.rs`
- `crates/api/src/app/tests.rs`
- `crates/api/src/app.rs`

Les tests de `app/tests.rs` couvrent des flows déjà testés dans `auth/handlers.rs` — pas de perte.

### Priorité 3 — Petits fixes (< 1 jour total)

1. **`extractor.rs`** — injecter `DecodingKey` dans AppState plutôt que lire l'env à chaque requête
2. **`rate_limit.rs`** — ajouter cleanup des entrées anciennes
3. **`cookies.rs`** — migrer vers le crate `cookie`
4. **Email validation** — ajouter le crate `email_address`

---

## Résumé des fichiers par état

| Fichier | État | Action |
|---|---|---|
| `auth/password.rs` | Excellent | Rien |
| `auth/jwt.rs` | Bon | Rien |
| `auth/rbac.rs` | Bon | Rien |
| `auth/store.rs` | Bon | Rien |
| `auth/audit.rs` | Bon | Rien |
| `auth/models.rs` | Bon | Rien |
| `auth/handlers.rs` | Bon | Exposer en prod (priorité 1) |
| `auth/extractor.rs` | Fonctionnel | Fix lecture env (priorité 3) |
| `auth/rate_limit.rs` | Fonctionnel | Ajouter cleanup (priorité 3) |
| `auth/cookies.rs` | Fragile | Migrer vers crate `cookie` (priorité 3) |
| `app/services.rs` | Dangereux | Supprimer après migration (priorité 2) |
| `app/domain.rs` | Prototype | Supprimer après migration (priorité 2) |
| `app/handlers.rs` | Prototype | Supprimer après migration (priorité 2) |
| `main.rs` | Bloquant | Câbler `auth/` (priorité 1) |
