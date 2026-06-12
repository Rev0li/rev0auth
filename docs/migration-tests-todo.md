# Tests à effectuer — handoff Claude

> **Ce fichier est réécrit à chaque nouvelle tâche par le Claude qui code.**
> Il décrit la **dernière tâche réalisée** et liste **les tests à exécuter** pour la valider.
> Branche : `dev/svelte` · Projet : `auth/frontend/`

---

## Dernière tâche réalisée

**Date** : 2026-06-12
**Tâche** : **Phase 3 — SvelteKit seul derrière Caddy**, crates/web retiré du déploiement.

### Changements
- **Caddyfile.template** : tout `WEB_DOMAIN` → frontend `:4173` (plus de matcher `/japprends/*`, plus de `WEB_UPSTREAM`). `caddy.env.example` mis à jour (`FRONTEND_UPSTREAM`).
- **docker-compose.yml** : service `web` supprimé, `Dockerfile.web` supprimé, `REV0AUTH_WEB_UPSTREAM` retiré.
- **`/portal`** : redirect 301 → `/` (l'info "invitation uniquement" est déjà sur la page de connexion). Le Watcher SongSurf qui redirige vers `/portal` continue de fonctionner.
- **Login admin réécrit** (`/japprends/login`) : formulaire pseudo + seed + mot de passe + code 2FA optionnel + challenge 3 icônes (choisir 🔒) + honeypot → `POST /japprends/login`. **Le mode YubiKey est retiré** (le WebAuthn vivait dans crates/web ; la passkey reviendra via le plugin BetterAuth).
- **Supprimé** : proxys `/japprends/webauthn/auth/{start,finish}`, `/portal/login` (login multi-étapes abandonné), ancien POST `/portal`.
- **Docs** : `DEPLOY.md`, `auth/CLAUDE.md`, `CLAUDE.md` racine mis à jour (architecture, checklists, dépannage).
- crates/web reste dans le repo (référence) mais n'est plus ni buildé ni déployé.

### État : testé en local ✅

| Test | Résultat |
|---|---|
| `/portal` → 301 `/` | PASS |
| Page login admin (SSR, champs Seed/MdP/2FA/challenge) | PASS |
| `POST /japprends/login` (challenge secure-lock) → session → `/japprends/users` 200 | PASS |
| Routes webauthn supprimées → 404 | PASS |
| Login membre (BetterAuth) inchangé | PASS |
| `npm run check` | 0 erreur / 10 warnings |
| `npm test` | 26/26 |

### ⚠️ À savoir avant le deploy
1. **L'admin se connecte désormais par mot de passe** (pseudo + seed + mdp + challenge 🔒 + TOTP si `ADMIN_DASH_TOTP_SECRET` est défini). **Vérifier que le TOTP est bien configuré en prod** pour garder un second facteur en attendant le plugin passkey BetterAuth.
2. Côté VPS, mettre à jour `/etc/caddy/rev0auth-caddy.env` : remplacer `WEB_UPSTREAM` par `FRONTEND_UPSTREAM=127.0.0.1:4173`, puis recharger Caddy.
3. `docker compose up -d --build` ne lancera plus le service `web` ; faire un `docker compose down` avant pour retirer l'ancien container.
4. Les sessions membres actives seront invalidées (re-login). Lancer la migration des comptes : `node --env-file=.env scripts/migrate-web-users-to-ba.mjs` (dry-run d'abord).

---

## Tests restants pour la prochaine session

### 1. Navigateur (local)
- [ ] Login admin via le nouveau formulaire (challenge 🔒) → dashboard, tous les onglets
- [ ] Flow membre complet : login → `/home/friend` → profil → mdp → logout
- [ ] Hérités : bouton "Rafraîchir" `/japprends/audit`, grilles d'avatars

### 2. Deploy VPS (checklist complète)
```bash
# 1. rsync (DEPLOY.md) puis sur le VPS :
cd ~/auth && docker compose down && docker compose up -d --build
docker compose ps        # postgres/api/frontend healthy (plus de web)
# 2. Caddy : FRONTEND_UPSTREAM=127.0.0.1:4173 dans rev0auth-caddy.env + reload
# 3. Migration comptes BetterAuth (dry-run d'abord)
# 4. Tests prod : login admin (challenge), login membre, flow SongSurf complet,
#    /status api_ok:true, https://rev0li.duckdns.org/portal → 301
# 5. Hérités : nettoyer SONGSURF_JWT_SECRET des .env/.secrets, vérifier ORIGIN
```

### 3. Validation type-check
```bash
cd /home/revoli/dev/rev0Univers/auth/frontend
npm run check   # attendu : 0 ERRORS / 10 WARNINGS
npm test        # attendu : 26/26
```

---

## Prochaine étape

Deploy VPS + tests prod, puis revue visuelle des pages membres (shadcn-svelte/MyCss). Ensuite : plugin passkey BetterAuth (restaurer la YubiKey admin), dashboard admin externe, Phase 4 (retrait crates/api).
