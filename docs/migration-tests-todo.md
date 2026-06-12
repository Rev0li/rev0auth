# Tests à effectuer — handoff Claude

> **Ce fichier est réécrit à chaque nouvelle tâche par le Claude qui code.**
> Il décrit la **dernière tâche réalisée** et liste **les tests à exécuter** pour la valider.
> Branche : `dev/svelte` · Projet : `auth/frontend/`

---

## Dernière tâche réalisée

**Date** : 2026-06-12
**Tâche** : Phase 2 — **bascule du flow membre sur BetterAuth**. Les sessions membres sont désormais des `ba_sessions` (cookie `better-auth.session_token`), plus le cookie custom `rev0auth_member_session`.

### Fichiers touchés
- `src/hooks.server.ts` — `locals.memberSession` résolu via `auth.api.getSession` (BetterAuth) ; admin inchangé (custom)
- `src/app.d.ts` — `memberSession: { pseudo, role } | null`
- `src/lib/server/ba-sync.ts` — **nouveau** : `ensureBaUser`, `setBaPassword`, `removeBaPassword`, `deleteBaUser`, `syncBaRole` (synchro web_users → ba_* pendant la coexistence)
- `routes/auth/password-check` — même contrat JSON, mais délègue à `auth.api.signInUsername` + **provisionnement paresseux** (compte web_users absent/désynchronisé de ba_* → auto-créé si mdp valide)
- `routes/auth/logout` — révoque la session BetterAuth + nettoyage cookie legacy
- Points de synchro ba_* ajoutés : `members/password` (PUT + onboarding), `members/account` (DELETE), `signup`, `japprends/users` (POST), `japprends/users/[pseudo]` (PUT role, DELETE), `japprends/users/[pseudo]/password` (POST/DELETE)
- `lib/server/session.ts` — `MEMBER_COOKIE_OPTS` supprimé (mort), `MEMBER_COOKIE` marqué legacy (nettoyage uniquement)

### État : testé en local ✅ (15 cas, dev server + curl)

| Test | Résultat |
|---|---|
| Login `password-check` → ok:true + cookie `better-auth.session_token` | PASS |
| Contrat JSON inchangé (ok/message/pseudo/songsurf_url) — page login non modifiée | PASS |
| `GET /` connecté → redirect `/home/friend` (hooks résout la session BetterAuth) | PASS |
| PUT `/members/profile/data` authentifié / sans cookie | 200 / 401 |
| Changement mdp → hash identique dans `web_users` ET `ba_accounts`, login nouveau mdp OK | PASS |
| Provisionnement paresseux : compte web_users sans lignes ba_* → login crée ba_users+credential | PASS |
| Logout → session révoquée, endpoint membre ensuite | 401 |
| Compte `active=false` rejeté au login | PASS |
| Signup (invitation) → lignes ba_* créées, login immédiat | PASS |
| `npm run check` | 0 erreur / 22 warnings |
| `npm test` (vitest) | 26/26 |

Compte de test local : `MigrTester` / `MigrTest123!`.

### À savoir

1. **Les sessions membres actives au moment du deploy seront invalidées** (cookie custom plus reconnu) → les membres devront se reconnecter une fois. TTL 24h inchangé.
2. Le **provisionnement paresseux** dans `password-check` rend la migration robuste : même si le script `migrate-web-users-to-ba.mjs` n'a pas tourné (ou si un mdp a été changé côté Rust), le login répare ba_* à la volée. Le script reste utile pour migrer en masse au deploy.
3. **Toute modification de credential/rôle/suppression passe par `ba-sync.ts`** — si tu ajoutes un endpoint qui touche `web_users.password_hash`, `role` ou supprime un user, synchronise ba_* pareil.
4. L'**auth admin reste custom** (`web_sessions`, cookie `rev0auth_admin_session`) — décision reportée (dashboard admin sera externalisé).

---

## Tests restants pour la prochaine session

### 1. Navigateur (non automatisables en curl)
- [ ] Flow login complet UI : login → `/home/friend` → profil → changement mdp → logout
- [ ] Flow SongSurf : login d'un compte `access_songsurf=true` → `songsurf_url` → redirect NAS
- [ ] Hérités : bouton "Rafraîchir" `/japprends/audit`, grilles d'avatars profil + signup

### 2. Au prochain deploy VPS
```bash
# 1. Migration en masse (le lazy-provisioning couvre les retardataires)
node --env-file=.env scripts/migrate-web-users-to-ba.mjs --dry-run
node --env-file=.env scripts/migrate-web-users-to-ba.mjs
# 2. Login d'un compte réel via le site
# 3. Checklist héritée : healthcheck frontend, /status api_ok:true, ORIGIN,
#    nettoyer SONGSURF_JWT_SECRET des .env/.secrets
```

### 3. Validation type-check
```bash
cd /home/revoli/dev/rev0Univers/auth/frontend
npm run check   # attendu : 0 ERRORS / 22 WARNINGS
npm test        # attendu : 26/26
```

---

## Prochaine étape

Voir [`migration-svelte-betterauth.md`](migration-svelte-betterauth.md) : audit du code mort côté user (web_sessions membres, colonnes avatar legacy…), puis revue visuelle des pages membres, puis dashboard admin externe.
