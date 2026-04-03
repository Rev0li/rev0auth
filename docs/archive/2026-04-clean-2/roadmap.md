# Roadmap MVP

## Phase 1 - Foundation

- setup VPS hardening (ufw, fail2ban, updates auto)
- setup DuckDNS + reverse proxy TLS
- deploy workspace Rust (API + Web)
- healthchecks + logs centralises

## Phase 2 - Auth

- inscription/connexion/deconnexion
- hash Argon2id + verification email
- sessions (access + refresh tokens)
- RBAC member/admin

## Phase 3 - Zone privee

- dashboard membre
- contenus prives par role
- audit log actions sensibles

## Phase 4 - Media

- index metadata video
- ACL media selon role
- streaming via URL signees (court TTL)

## Phase 5 - Public Pro

- portfolio
- page CV
- formulaire contact (anti-spam)
