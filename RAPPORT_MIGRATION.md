# Rapport — État de l'architecture rev0auth

> Généré le 2026-05-25 — branche `dev/migration_svelte`

---

## Vue d'ensemble

| Composant | Techno | Port | Déployé ? |
|---|---|---|---|
| **API** | Rust / Axum | 8080 | ✅ prod |
| **Web (frontend actuel)** | Rust SSR (HTML inline) | 3000 | ✅ prod |
| **Frontend SvelteKit** | SvelteKit / adapter-node | 3001 (prévu) | ❌ pas encore |
| **DB** | PostgreSQL 16 | 5432 | ✅ prod |
| **Reverse proxy** | Caddy | 80/443 | ✅ prod |

---

## Ce qui tourne en production aujourd'hui

### `crates/api/` — rev0auth-api (port 8080)

API Axum pure, **jamais exposée publiquement** (interne uniquement). Routes :

| Méthode | Route | Rôle |
|---|---|---|
| `GET` | `/health` | Health check |
| `POST` | `/auth/signup` | Inscription (Argon2) |
| `POST` | `/auth/login` | Login JWT |
| `POST` | `/auth/refresh` | Rotation refresh token |
| `GET` | `/auth/me` | Profil via JWT |
| `GET` | `/admin/panel` | Panel admin API |

**Note :** le web crate ne passe pas par cette API — il interroge PostgreSQL directement.

---

### `crates/web/` — rev0auth-web (port 3000)

Frontend SSR en Rust pur. Pas de framework HTML. Toutes les pages sont des `String` construites en Rust. ~3 400 lignes dans `main.rs`, ~8 000 lignes dans `pages/`.

#### Pages rendues côté serveur

| Route | Description |
|---|---|
| `GET /` | Page de login (formulaire pseudo + mdp) |
| `GET /portal` | Page d'accueil publique (lien vers login) |
| `GET /signup` | Inscription via code d'invitation |
| `GET /japprends/login` | Login admin (WebAuthn YubiKey ou bootstrap) |
| `GET /home/friend` | Tableau de bord membre (chat mural, services, statut) |
| `GET /members/profile` | Profil membre (bio, avatar, mdp, messages, dons) |
| `GET /japprends/tdd` | Dashboard admin (utilisateurs, invitations, messages, dons, tests) |

#### API JSON (appelées en JS depuis les pages)

**Auth**
- `POST /auth/password-check` — vérifie mdp, crée session + JWT SongSurf
- `POST /auth/logout` — supprime session membre
- `POST /portal/login` — vérifie si pseudo existe (pré-check login)

**Membre**
- `GET /members/profile/data` — profil complet JSON
- `PUT /members/profile/data` — mise à jour bio/commentary/github/linkedin
- `PUT /members/password` — changement mdp (vérifie ancien)
- `PUT /members/status` — changer son statut (actif/occupé/inactif)
- `POST /members/access/request` — demande accès service (songsurf/jellyfin)
- `POST /members/avatar` — upload avatar (multipart)
- `GET /members/avatar/:pseudo` — servir l'image d'avatar
- `DELETE /members/avatar/:pseudo` — supprimer avatar
- `GET /members/messages/inbox` — messages reçus
- `GET /members/messages/sent` — messages envoyés
- `POST /members/messages/send` — envoyer un message
- `POST /members/messages/:id/read` — marquer lu
- `GET /members/donations` — liste dons du membre
- `POST /members/donations/proof` — upload justificatif don
- `GET /members/donations/crypto-addresses` — adresses crypto
- `GET /members/wall` — mur de discussion
- `POST /members/wall` — poster sur le mur
- `DELETE /members/wall/:id` — supprimer son post
- `DELETE /members/account` — supprimer son compte
- `GET /users` — liste membres actifs (publique pour membres connectés)

**Admin**
- `POST /japprends/login` — login admin (seed + mdp + TOTP)
- `POST /japprends/logout` — logout admin
- `GET /japprends/webauthn/auth/start` — démarrer auth YubiKey
- `POST /japprends/webauthn/auth/finish` — finir auth YubiKey
- `GET /japprends/webauthn/register/start` — enregistrer nouvelle YubiKey
- `POST /japprends/webauthn/register/finish`
- `GET /japprends/webauthn/status` — état credential WebAuthn
- `POST /japprends/webauthn/remove` — supprimer credential
- `GET /japprends/webauthn/credential/export`
- `GET /japprends/invites` — liste invitations
- `POST /japprends/invites` — créer invitation
- `DELETE /japprends/invites/:id` — révoquer invitation
- `GET /japprends/users` — liste utilisateurs
- `POST /japprends/users` — créer utilisateur
- `PUT /japprends/users/:pseudo` — modifier utilisateur
- `DELETE /japprends/users/:pseudo` — supprimer utilisateur
- `POST /japprends/set-password/:pseudo` — réinitialiser mdp membre
- `GET /japprends/messages` — tous les messages
- `POST /japprends/messages/reply` — répondre à un message
- `POST /japprends/messages/mark-read` — marquer thread lu
- `DELETE /japprends/messages/thread/:pseudo` — supprimer thread
- `GET /japprends/donations` — tous les dons
- `POST /japprends/donations/:id/review` — valider/rejeter don
- `DELETE /japprends/wall/:id` — supprimer post mural
- `GET /japprends/songsurf-logs` — logs SongSurf (proxy serveur)
- `GET /japprends/songsurf-access` — liste accès SongSurf
- `POST /status/set-busy/:pseudo` — admin force statut occupé
- `POST /status/set-active/:pseudo` — admin force statut actif
- `POST /status/set-inactive/:pseudo` — admin force statut inactif
- `GET /status` — statut DB
- `GET /status/all` — statut tous services

#### Librairies Rust utilisées
`axum`, `sqlx` (PostgreSQL), `argon2`, `jsonwebtoken`, `webauthn-rs`, `reqwest`, `totp-rs`, `tokio`, `tracing`

---

### Base de données — PostgreSQL 16

Tables gérées par les migrations `crates/api/migrations/` :

| Table | Rôle |
|---|---|
| `auth_users` | Utilisateurs API (JWT auth) |
| `auth_refresh_tokens` | Refresh tokens + CSRF |
| `auth_audit_logs` | Logs d'audit API |
| `web_users` | Membres du portail web |
| `web_messages` | Messages internes |
| `web_donations` | Dons déclarés |
| `web_wall_posts` | Mur de discussion |
| `web_invites` | Codes d'invitation (migration 0006) |

Tables créées au démarrage par SvelteKit (à venir) :

| Table | Rôle |
|---|---|
| `web_sessions` | Sessions admin + membre HttpOnly |
| `web_test_runs` | Historique des runs de tests |

---

## Ce qui est sur la branche `dev/migration_svelte` (pas encore déployé)

### `frontend/` — SvelteKit (adapter-node)

Migration en cours du Rust SSR vers SvelteKit 5 (runes mode). **39 fichiers modifiés/créés** depuis le dernier commit main.

#### Changements DB layer
- Supprimé : `better-sqlite3`, `bcryptjs` (SQLite local)
- Ajouté : `postgres` (driver), `argon2` (hash Argon2 compatible Rust), `jose` (SongSurf JWT HS256)
- `db/schema.ts` : réécrit pour mapper exactement les tables `web_*` de PostgreSQL
- `db/index.ts` : driver postgres-js, auto-création de `web_sessions` + `web_test_runs` au boot

#### Pages migrées

| Route SvelteKit | Équivalent Rust | État |
|---|---|---|
| `GET /` | `GET /` | ✅ login (pseudo + mdp, même UX) |
| `GET /signup?invite=CODE` | `GET /signup` | ✅ avec grille 5 avatars SVG |
| `GET /home/friend` | `GET /home/friend` | ✅ (rendu SSR + données DB) |
| `GET /members/profile` | `GET /members/profile` | ✅ |
| `GET /japprends/tdd` | `GET /japprends/tdd` | ✅ dashboard admin complet |
| `GET /japprends/login` | `GET /japprends/login` | ✅ WebAuthn auto-start + fallback mdp |

#### Routes API portées (couverture pages migrées)

**Auth**
- `POST /auth/password-check` — argon2 verify + cookie HttpOnly session + SongSurf JWT ✅
- `POST /auth/logout` ✅
- `POST /portal/login` → retourne 403 (inscription sur invitation uniquement) ✅

**Membre**
- `PUT /members/profile/data` ✅
- `PUT /members/password` ✅
- `PUT /members/status` ✅
- `POST /members/access/request` ✅
- `POST /members/avatar` (upload multipart) ✅
- `GET /members/avatar/[pseudo]` ✅
- `DELETE /members/avatar` ✅
- `GET /members/messages` (inbox + sent via `?folder=`) ✅
- `POST /members/messages` (envoi) ✅
- `PATCH /members/messages` (mark read) ✅
- `GET /members/donations` ✅
- `POST /members/donations` ✅
- `GET /members/wall` ✅
- `POST /members/wall` ✅
- `DELETE /members/wall` ✅
- `GET /users` ✅

**Inscription**
- `POST /signup` — valide invite + crée user + sauvegarde avatar SVG ✅

**Admin**
- `POST /japprends/login` (seed + mdp + TOTP + honeypot) ✅
- `POST /japprends/logout` ✅
- `GET /japprends/webauthn/auth/start` (proxy → Rust web) ✅
- `POST /japprends/webauthn/auth/finish` (proxy → Rust web, crée session admin SvelteKit) ✅
- `GET /japprends/users`, `POST`, `PUT /[pseudo]`, `DELETE /[pseudo]` ✅
- `PUT /japprends/users/[pseudo]/password` ✅
- `GET /japprends/signup-requests` (liste invites), `POST` (créer invite) ✅
- `POST /japprends/signup-requests/[id]/approve` (révoquer invite) ✅
- `POST /japprends/signup-requests/[id]/reject` ✅
- `GET /japprends/messages`, `POST /reply` ✅
- `GET /japprends/donations`, `POST /[id]/review` ✅
- `POST /japprends/tests/launch` ✅
- `GET /japprends/tests/history` ✅
- `GET /status` ✅

#### Amélioration par rapport au Rust
- Sessions membre en **cookie HttpOnly** (le Rust utilisait localStorage uniquement)
- SongSurf JWT généré **côté serveur** SvelteKit (le Rust le générait aussi côté serveur, mais via proxy reqwest)
- `web_sessions` en DB → sessions survivent aux redémarrages

#### Routes non portées (ne bloquent pas les pages migrées)
| Route Rust | Bloque quoi ? |
|---|---|
| `GET/POST/DELETE /japprends/webauthn/register/*` | Enregistrement nouvelle YubiKey |
| `GET /japprends/webauthn/status`, `/remove`, `/export` | Gestion YubiKey |
| `DELETE /japprends/messages/thread/:pseudo` | Admin — suppression thread |
| `DELETE /japprends/wall/:id` | Admin — suppression post mural |
| `GET /japprends/songsurf-logs`, `/songsurf-access` | Monitoring SongSurf |
| `POST /status/set-*/[:pseudo]` | Override statut admin |
| `GET /members/profile/data` (GET) | Non appelé par les pages |
| `DELETE /members/account` | Suppression compte membre |
| `POST /members/donations/proof` | Upload justificatif |
| `GET /members/donations/crypto-addresses` | Adresses crypto |

---

## Ce qu'il manque pour déployer

### Fichiers à créer

- [ ] `Dockerfile.frontend` — build SvelteKit (`npm ci` + `npm run build` + `node build/`)
- [ ] Service `frontend` dans `docker-compose.yml` (port 3001)
- [ ] Mise à jour `deploy.yml` — ajouter `frontend` au build
- [ ] Mise à jour `Caddyfile.template` — router vers SvelteKit (3001) au lieu de Rust (3000)

### Variables d'env à ajouter sur le VPS

```bash
# Dans .env ou .secrets — même valeur que AUTH_JWT_SECRET
SONGSURF_JWT_SECRET=<valeur de AUTH_JWT_SECRET>

# Rust web reste en parallèle pour les routes non portées + WebAuthn register
REV0AUTH_WEB_UPSTREAM=http://web:3000
```

### Ce qui ne change pas
- `DATABASE_URL` — identique
- `ADMIN_DASH_PASSWORD`, `ADMIN_DASH_PSEUDO`, `ADMIN_DASH_SEED`, `ADMIN_DASH_TOTP_SECRET` — identiques
- `SONGSURF_URL` — identique
- Les données PostgreSQL — aucune migration nécessaire

---

## Plan de déploiement recommandé

```
1. Créer Dockerfile.frontend
2. Ajouter service frontend dans docker-compose.yml (port 3001)
3. Caddy : frontend (3001) en principal, Rust web (3000) en fallback pour routes non portées
4. Ajouter SONGSURF_JWT_SECRET + REV0AUTH_WEB_UPSTREAM dans .env VPS
5. Merger dev/migration_svelte → main
6. Push → GitHub Action déploie automatiquement
7. Tester en prod
8. Quand stable : porter les routes manquantes, puis supprimer Rust web crate
```
