# Tests à effectuer — handoff Claude

> **Ce fichier est réécrit à chaque nouvelle tâche par le Claude qui code.**
> Il décrit la **dernière tâche réalisée** et liste **les tests à exécuter** pour la valider.
> Branche : `dev/svelte` · Projet : `auth/frontend/`

---

## Dernière tâche réalisée

**Date** : 2026-06-12
**Tâche** : **Audit et suppression du code mort** (post-bascule BetterAuth + abandons actés du tour d'horizon).

### Supprimé
- **Dashboard TDD** (abandon acté) : `routes/japprends/tdd/`, `routes/japprends/tests/{launch,history}`, table `web_test_runs` (schema + initDb). Redirects/liens `tdd` → `/japprends/dashboard`.
- **Endpoints parité Rust sans aucun appelant** (état cible post-Phase 3) : `/users`, `/user/ping`, `/japprends/ping`, `/japprends/auth-check`, `/status/all`, `/status/set-{busy,active,inactive}/[pseudo]`.
- **`session.ts`** : simplifié admin-only (la branche `member` était morte depuis la bascule BetterAuth) — `createSession(pseudo)` / `getSession(token)` sans paramètre `kind`.
- **Deps npm mortes** : `bits-ui` (aucun import), `happy-dom` (vitest tourne en env node), `@sveltejs/adapter-auto` (le build utilise adapter-node).
- Faux positif corrigé : `lib/utils.ts` (`cn()`) est **vivant** — utilisé par `lib/components/ui/*` (login page) ; `clsx`/`tailwind-merge` conservés.

### Gardé volontairement (à connaître)
- `/status` — endpoint ops (checklist deploy, futur dashboard externe), admin-only.
- `/members/account` DELETE et `/members/donations/crypto-addresses` — endpoints sans UI Svelte pour l'instant (suppression de compte et donations crypto existent en Rust ; UI à porter ou feature à trancher).
- `/portal` + `/portal/login` — nécessaires Phase 3.
- `/api/auth/[...all]` — surface BetterAuth (client SDK futur).

### ⚠️ Découverte importante — routage prod
Le Caddyfile ne route vers SvelteKit que **`/japprends/*` et `/_app/*`**. Tout le reste (membres, login, signup, portal) est encore servi par le **Rust** en prod. La bascule BetterAuth du flow membre ne sera donc **effective qu'au switch Caddy (Phase 3)**. À ce moment-là, ajouter les paths SvelteKit dans le Caddyfile (ou tout basculer sauf les routes Rust restantes).

### État : testé en local ✅

| Test | Résultat |
|---|---|
| Login admin (challenge + createSession refactoré) → `/japprends/users` 200 | PASS |
| `GET /` admin → redirect `/japprends/dashboard` (plus tdd) | PASS |
| Login membre `password-check` | PASS |
| Routes supprimées → 404 (`/users`, `/user/ping`, `/japprends/ping`, `/japprends/tdd`, `/status/all`) | PASS |
| `/status` conservé → 200 admin / 401 sans session | PASS |
| `npm run check` | 0 erreur / 10 warnings |
| `npm test` (vitest) | 26/26 |

Compte de test local : `MigrTester` / `MigrTest123!`.

---

## Tests restants pour la prochaine session

### 1. Navigateur
- [ ] Login admin UI complète (page `/japprends/login` avec challenge) → dashboard, onglets Members/Invitations/Donations/Messages/Wall
- [ ] Flow membre UI : login → `/home/friend` → profil → changement mdp → logout
- [ ] Hérités : bouton "Rafraîchir" `/japprends/audit`, grilles d'avatars

### 2. Au prochain deploy VPS
```bash
node --env-file=.env scripts/migrate-web-users-to-ba.mjs --dry-run && node --env-file=.env scripts/migrate-web-users-to-ba.mjs
# DROP TABLE web_test_runs;  -- table morte, à nettoyer manuellement (initDb ne la recrée plus)
# checklist héritée : healthcheck, /status api_ok, ORIGIN, SONGSURF_JWT_SECRET
```

### 3. Validation type-check
```bash
cd /home/revoli/dev/rev0Univers/auth/frontend
npm run check   # attendu : 0 ERRORS / 10 WARNINGS
npm test        # attendu : 26/26
```

---

## Prochaine étape

Voir [`migration-svelte-betterauth.md`](migration-svelte-betterauth.md) : revue visuelle des pages membres (shadcn-svelte/MyCss), Phase 3 (porter `/portal` + `/` + switch Caddy), trancher UI suppression de compte & donations crypto.
