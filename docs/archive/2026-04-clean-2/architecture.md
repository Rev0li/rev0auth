# Architecture proposee (VPS + NAS + Tailscale + DuckDNS)

## Objectif

Construire une plateforme Rust avec:
- espace prive pour membres par rang
- service video partage (DldeMedia)
- espace public portfolio/CV/contact

## Topologie reseau recommandee

1. Internet -> DuckDNS -> VPS (443)
2. VPS reverse proxy -> services Rust internes (ports prives)
3. VPS -> NAS via interface Tailscale (100.x.y.z)
4. NAS inaccessible depuis Internet public

## Composants

- Reverse proxy TLS: Caddy (certificats auto) ou Nginx + ACME
- `rev0auth-web` (port 3000): pages publiques + portail membres
- `rev0auth-api` (port 8080): auth, RBAC, profil, media permissions
- PostgreSQL: users, roles, sessions, audit
- Redis (optionnel): rate limit, cache sessions

## Domaines DuckDNS

- `revoli.duckdns.org` -> front public
- `api-revoli.duckdns.org` -> API
- `media-revoli.duckdns.org` -> proxy media (si necessaire)

## Auth/Roles (RBAC)

- `public`: pages portfolio/CV/contact
- `member`: acces contenu prive
- `premium` (optionnel): contenu media exclusif
- `admin`: moderation + gestion utilisateurs + logs

## Bonnes pratiques de securite

- MFA pour comptes admin
- hash mot de passe Argon2id
- cookies `HttpOnly`, `Secure`, `SameSite=Lax`
- CSRF protection pour formulaires sensibles
- rate limiting login + lockout progressif
- policy de backup DB + tests restauration
