# connexion.md — Integration Guide for Linked Services

This document explains how a service linked to rev0auth (e.g. Songsurf) receives and validates its JWT.

---

## How it works

rev0auth issues signed JWT access tokens. Any linked service that shares the same `AUTH_JWT_SECRET` can independently validate these tokens — no round-trip to rev0auth is needed.

---

## Token format

**Algorithm:** HS256
**Transport:** `Authorization: Bearer <token>`

### Claims

```json
{
  "sub":        "550e8400-e29b-41d4-a716-446655440000",
  "email":      "user@example.com",
  "role":       "member",
  "token_type": "access",
  "iat":        1713700000,
  "exp":        1713700900
}
```

| Claim        | Type   | Description                          |
|--------------|--------|--------------------------------------|
| `sub`        | UUID   | User ID                              |
| `email`      | string | User email (lowercased)              |
| `role`       | string | `"member"` or `"admin"`              |
| `token_type` | string | Always `"access"` for access tokens  |
| `iat`        | u64    | Issued at (Unix epoch, seconds)      |
| `exp`        | u64    | Expires at (Unix epoch, seconds)     |

**Default TTL:** 15 minutes (configurable via `AUTH_ACCESS_TTL_SECS` on the rev0auth side).

---

## How the client gets the token

### After login (browser flow)

`POST /auth/login` sets an HttpOnly cookie named `access_token`. Since it is HttpOnly, JavaScript cannot read it. For a cross-domain service like Songsurf, the user must forward the token explicitly.

Two practical options:

**Option A — Cookie forwarding (same domain / reverse proxy)**
If Songsurf sits behind the same Caddy reverse proxy and shares the domain, the browser sends the `access_token` cookie automatically. Songsurf reads it from the `Cookie` header.

**Option B — Explicit Bearer (different domain)**
The client reads the token from the signup response (`POST /auth/signup` returns `access_token` in JSON) or from a dedicated token-exchange endpoint you add later. The client then sends:

```
Authorization: Bearer <access_token>
```

---

## What Songsurf must do to validate

1. Read `AUTH_JWT_SECRET` from its own environment (must match rev0auth).
2. Decode the JWT with HS256.
3. Validate `exp` (reject if expired).
4. Check `token_type == "access"` (reject refresh tokens).
5. Use `sub` (UUID) as the user identity, `role` for access control.

### Example — Rust (jsonwebtoken crate)

```rust
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct Claims {
    pub sub: String,
    pub email: String,
    pub role: String,
    pub token_type: String,
    pub exp: u64,
}

fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = std::env::var("AUTH_JWT_SECRET").expect("AUTH_JWT_SECRET required");

    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;
    validation.leeway = 0;

    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )?;

    if data.claims.token_type != "access" {
        return Err(jsonwebtoken::errors::ErrorKind::InvalidToken.into());
    }

    Ok(data.claims)
}
```

### Example — Node.js (jsonwebtoken package)

```js
import jwt from 'jsonwebtoken';

function validateToken(token) {
  const secret = process.env.AUTH_JWT_SECRET;
  const claims = jwt.verify(token, secret, { algorithms: ['HS256'] });

  if (claims.token_type !== 'access') {
    throw new Error('invalid token type');
  }

  return claims; // { sub, email, role, iat, exp }
}
```

### Example — Python (PyJWT)

```python
import jwt, os

def validate_token(token: str) -> dict:
    secret = os.environ["AUTH_JWT_SECRET"]
    claims = jwt.decode(token, secret, algorithms=["HS256"])

    if claims.get("token_type") != "access":
        raise ValueError("invalid token type")

    return claims  # { sub, email, role, iat, exp }
```

---

## Role-based access

Use the `role` claim to gate features:

| Role      | Access level                        |
|-----------|-------------------------------------|
| `member`  | Standard authenticated user         |
| `admin`   | Full access — reserved for you      |

```rust
if claims.role != "member" && claims.role != "admin" {
    return Err("forbidden");
}
```

---

## Error cases to handle

| Situation                   | Action                        |
|-----------------------------|-------------------------------|
| Missing `Authorization`     | `401 Unauthorized`            |
| Signature invalid           | `401 Unauthorized`            |
| Token expired (`exp` past)  | `401 Unauthorized` — client must refresh |
| `token_type != "access"`    | `401 Unauthorized`            |
| `role` insufficient         | `403 Forbidden`               |

---

## Shared secret setup

Both rev0auth and Songsurf must have the same value for `AUTH_JWT_SECRET`.

```env
# .env (rev0auth)
AUTH_JWT_SECRET=your-secret-minimum-32-chars

# .env (songsurf)
AUTH_JWT_SECRET=your-secret-minimum-32-chars
```

The secret must be at least 32 bytes. Never commit it. Rotate it by updating both services simultaneously (all existing tokens become invalid on rotation).

---

## Token refresh

Songsurf does not handle refresh itself. When a token is expired, redirect the user to rev0auth:

```
POST /auth/refresh
Cookie: refresh_token=...; csrf_token=...
X-CSRF-Token: ...
```

rev0auth rotates the session and issues a new `access_token` cookie. The user returns to Songsurf with a fresh token.
