# Tests à effectuer — handoff Claude

> **Ce fichier est réécrit à chaque nouvelle tâche par le Claude qui code.**
> Il décrit la **dernière tâche réalisée** et liste **les tests à exécuter** pour la valider.
> Branche : `dev/svelte` · Projet : `auth/frontend/`

---

## Dernière tâche réalisée

**Date** : 2026-06-10
**Tâche** : Validation Phase 1 + fixes de l'audit (`docs/audit-migration-svelte-2026-06-10.md`) + commits.

### État : tout est déjà testé et commité ✅

Contrairement aux handoffs précédents, **les tests ont été exécutés dans la session** :

| Suite | Résultat |
|---|---|
| 16 tests handoff Phase 1 (audit/songsurf-logs/dashboard) + smoke Status/Members/Admin | **34 PASS / 0 FAIL** (T9 skip, navigateur) |
| Tests fonctionnels des fixes S1/S2/P1/P2/P4/P5 | **11 PASS / 0 FAIL** |
| Refactor avatar_id (6 cas dont rejet SVG arbitraire et multipart) | **PASS** |
| vitest | 13/13 |
| svelte-check | 0 erreur / 22 warnings |
| cargo test -p rev0auth-api | 70/70 |

Scripts utilisés (réutilisables) : `/tmp/phase1-tests.sh`, `/tmp/fixes-tests.sh` — nécessitent le dev server + une session admin dans `/tmp/admin.jar`.

### Commits de la session (13)

Phase 1 en 6 blocs atomiques (status / members / audit-infra / admin / pages / docs), puis :
- `fix(security)` — S1 verifyPassword hash vide, S2 rate limit login membre, P1 cascade+audit delete_user, P2 pseudo lowercase, P4 audit set_password, P5 min 8 chars
- `refactor(avatar)` — catalogue `$lib/avatars.ts`, contrat `{avatar_id}`, plus d'upload SVG arbitraire (S3)
- `fix(songsurf)` — secret unifié `AUTH_JWT_SECRET` + helper `$lib/server/songsurf.ts` (S4) — **corrige le 502 réel de songsurf-logs**
- `fix(compose)` — healthcheck node (wget absent de node:22-slim), `REV0AUTH_API_UPSTREAM=api:8080`, `ORIGIN` par défaut
- `ci` — workflow check + vitest + cargo test sur `dev/svelte` et `main`
- `fix(dev)` — `loadEnv` dans vite.config : `.env` enfin chargé en `npm run dev`

### Changements de contrat à connaître

1. **`POST /members/avatar`** : JSON `{"avatar_id": "fox"}` (plus de multipart). IDs valides : fox, wolf, cat, eagle, dragon.
2. **`SONGSURF_JWT_SECRET` déprécié** : tout passe par `AUTH_JWT_SECRET` (fallback legacy encore lu). À nettoyer dans les `.env`/`.secrets` du VPS au prochain deploy.
3. **Signup** stocke les pseudos en lowercase (les comptes legacy casse mixte restent lisibles via les lookups `LOWER()`).

---

## Tests restants pour la prochaine session

### 1. Navigateur (non automatisables en curl)
- [ ] T9 handoff précédent : bouton "Rafraîchir" de `/japprends/audit`
- [ ] Grille d'avatars dans `/members/profile` : sélection → Sauvegarder → l'avatar s'affiche (vider le cache img si besoin)
- [ ] Grille signup toujours OK (catalogue partagé `$lib/avatars.ts`)

### 2. Après deploy VPS (checklist)
```bash
# le service frontend doit être healthy (avant : unhealthy permanent, wget absent)
ssh user@rev0li.duckdns.org "docker ps --format '{{.Names}} {{.Status}}'"
# /status doit répondre api_ok:true (avant : toujours false)
# vérifier que ORIGIN est défini dans ~/auth/.env (sinon défaut https://rev0li.duckdns.org)
# flow SongSurf complet : login membre avec access_songsurf → redirect token → NAS
```

### 3. Validation type-check
```bash
cd /home/revoli/dev/rev0Univers/auth/frontend
npm run check   # attendu : 0 ERRORS / 22 WARNINGS
npm test        # attendu : 13/13
```

---

## Prochaine étape : Phase 2 BetterAuth

Voir le point de reprise dans [`migration-svelte-betterauth.md`](migration-svelte-betterauth.md).
