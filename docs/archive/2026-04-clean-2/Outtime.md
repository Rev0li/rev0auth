# Outtime Documentation

Journal intime du kiff de code.

## 2026-04-01

### Session matinee

- AUTH-006-TEST-PWD complete
- AUTH-006-TEST-JWT complete
- AUTH-006-TEST-STORE-MEM complete
- Pipeline API verte: 18/18 tests

### Session frontend fun

- Landing moderne en HTML/CSS
- Dashboard admin/user ajoute
- Monitoring live via endpoint /status

### Ce qui a ete satisfaisant

- Boucle courte: coder -> tester -> valider -> documenter
- Visibilite immediate des progres
- Pas de dette cachee sur les tests du coeur auth

### Frictions notees

- Gestion des raw strings HTML dans Rust
- Besoin de garder des assertions de test robustes sans surcharger les structs

### Lecons du jour

1. Une roadmap horaire aide a rester focus.
2. Un ticket = une preuve (test ou demo visible).
3. Le fun frontend booste la motivation pour les parties infra plus lourdes.

### Prochaine vibe

- Ajouter RBAC visuel dans le dashboard
- Differencier clairement les etats admin vs user
- Garder le niveau de qualite actuel sur chaque tranche de dev
