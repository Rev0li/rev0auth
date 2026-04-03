# PERF Baseline - AUTH-008-PERF

Date: 2026-04-02

## Objectif

Valider un baseline de performance et de stabilite sur les chemins auth critiques:
- charge login concurrente (seuil pragmatique)
- rotation refresh repetitive sans accumulation de sessions

## Tests implementes

Fichier: `crates/api/tests/integration_tests.rs`

- `test_50_concurrent_logins()`
  - Mode: in-memory router (flow HTTP login complet)
  - Charge: 50 logins (10 workers x 5 tentatives)
  - Assertion: toutes les tentatives retournent HTTP 200
  - Resultat observe: `perf.login.50 elapsed_ms=19512`

- `test_memory_leak_after_1000_refreshes()`
  - Mode: PostgreSQL (testcontainer)
  - Charge: 1000 rotations refresh consecutives
  - Assertion: cardinalite sessions = 1 token restant pour l'utilisateur
  - Resultat observe: `perf.refresh.1000 elapsed_ms=5208 remaining_tokens=1`

## Commandes executees

```bash
cargo test -p rev0auth-api --test integration_tests test_50_concurrent_logins -- --test-threads=1 --nocapture
cargo test -p rev0auth-api --test integration_tests test_memory_leak_after_1000_refreshes -- --test-threads=1 --nocapture
```

## Conclusion

- Baseline validee pour un seuil login concurrent de 50.
- Rotation refresh x1000 stable sans fuite logique de sessions.
- Les scenarios servent de garde-fou regression, pas de benchmark micro-optimise.
