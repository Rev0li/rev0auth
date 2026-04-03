# AUTH-009-HEALTH-CHECKS

Date: 2026-04-03

## Objectif

Rendre le service deployable sans downtime via des probes de disponibilite explicites.

## Endpoints

- `GET /health`
  - Reponse: `{ "service": "rev0auth-api", "status": "ok" }`
- `GET /health/db`
  - Reponse: `{ "status": "ok|down", "backend": "memory|postgres", "latency_ms": <u64> }`

## Implementation

- Route `/health`: liveness globale API
- Route `/health/db`: readiness backend
  - backend memory -> `status=ok`, `latency_ms=0`
  - backend postgres -> `SELECT 1` + mesure latence

## Docker Compose

Le service API est maintenant monitore par un healthcheck:

```yaml
healthcheck:
  test: ["CMD-SHELL", "curl -f http://localhost:8080/health || wget -qO- http://localhost:8080/health >/dev/null"]
  interval: 30s
  timeout: 10s
  retries: 3
```

## Tests

- `app::tests::test_health_endpoint`
- `app::tests::test_health_db_endpoint_memory_backend`

## Validation executee

Commande:

```bash
cargo test -p rev0auth-api -- --test-threads=1
```

Resultat:
- suite complete OK (unit + integration)
- endpoints health verifies via tests app
