# Tests à effectuer — handoff Claude

> **Ce fichier est réécrit à chaque nouvelle tâche par le Claude qui code.**
> Il décrit la **dernière tâche réalisée** et liste **les tests à exécuter** pour la valider.
> Branche : `dev/svelte` · Projet : `auth/frontend/`

---

## Dernière tâche réalisée

**Date** : 2026-05-30
**Tâche** : Migration des **3 pages manquantes** de Phase 1 + mise en place audit log persistant.

### Changements majeurs
1. **Nouvelle table `web_audit_log`** créée via `initDb()` + helper `lib/server/audit.ts` `writeAudit()` (Postgres)
2. Remplacement des `console.info('[admin-audit] ...')` des tâches précédentes par `writeAudit()` dans `remove-password` et `wall-delete`
3. **3 pages** : audit log viewer, SongSurf logs viewer, redirect members/dashboard

### Fichiers créés
- `frontend/src/lib/server/audit.ts` — helper `writeAudit(action, actorPseudo, target?, detail?)`
- `frontend/src/routes/japprends/audit/data/+server.ts` — `GET` admin-only, retourne entries triées DESC en JSON (limit param, défaut 200, max 1000). *Endpoint dédié à part de la page pour éviter le conflit `+page.svelte`/`+server.ts` au même chemin.*
- `frontend/src/routes/japprends/audit/+page.server.ts` — load 200 entries
- `frontend/src/routes/japprends/audit/+page.svelte` — table audit (Date / Action / Acteur / Cible / Détail) + filtre + bouton refresh
- `frontend/src/routes/japprends/songsurf-logs/+server.ts` — proxy JWT vers SongSurf `/api/admin/dl-logs` (JWT HS256, 120s TTL, cookie `access_token`)
- `frontend/src/routes/japprends/songsurf-logs/+page.server.ts` — guard admin
- `frontend/src/routes/japprends/songsurf-logs/+page.svelte` — form pseudo/limit + affichage JSON
- `frontend/src/routes/members/dashboard/+page.server.ts` — `redirect(301, '/home/friend')`

### Fichiers modifiés
- `frontend/src/lib/server/db/index.ts` — ajout `CREATE TABLE IF NOT EXISTS web_audit_log` + index DESC
- `frontend/src/lib/server/db/schema.ts` — ajout `auditLog` pgTable + type `AuditEntry`
- `frontend/src/routes/japprends/users/[pseudo]/password/+server.ts` — `console.info` → `await writeAudit('remove_password', ...)`
- `frontend/src/routes/japprends/wall/[id]/+server.ts` — `console.info` → `await writeAudit('wall_delete', ...)`

### Décisions
- **Audit DB** plutôt qu'in-memory : persistant, queryable, propre. Réservé à des actions admin sensibles (pas tous les GET).
- **`/members/dashboard` = redirect** : le Rust appelait littéralement la même fonction de rendu que `/home/friend`. Pas de duplication de page.
- **Hero/tuto static** : différé jusqu'à migration des pages qui les consomment (aucune dans le SvelteKit actuel).

---

## Pré-requis pour tester

```bash
# 1. DB
cd /home/revoli/dev/rev0Univers/auth
make launch-all

# 2. SvelteKit dev — important : la table web_audit_log est créée au boot
cd frontend
npm run dev
# → vérifier dans les logs au démarrage que pas d'erreur SQL
```

**Vérif initiale DB** :
```sql
\d web_audit_log
-- Doit montrer : id (bigint), action (text), actor_pseudo (text), target (text), detail (text), created_at_epoch (bigint)
\d+ web_audit_log
-- Doit montrer l'index web_audit_log_created_idx
```

**Cookies** : session admin (`rev0auth_admin_session`) requise pour tous les tests.

**Variables env optionnelles pour songsurf-logs** :
```bash
# .env
SONGSURF_JWT_SECRET=...      # même secret que SongSurf (AUTH_JWT_SECRET partagé)
SONGSURF_URL=http://localhost:8000  # ou URL Tailscale du NAS
```

---

## Tests à exécuter

### A. Audit log — création de table et helper

#### Test 1 — Table créée au boot
**Vérif** : après `npm run dev`, `SELECT count(*) FROM web_audit_log;` ne doit pas échouer (table existe).

#### Test 2 — Helper écrit en DB (via remove-password)
```bash
# Setup : user alice avec mot de passe (cf section setup tâche précédente)
curl -X DELETE -H "Cookie: rev0auth_admin_session=<TOKEN>" \
  http://localhost:5173/japprends/users/alice/password
```
**Vérif DB** :
```sql
SELECT action, actor_pseudo, target, detail FROM web_audit_log ORDER BY id DESC LIMIT 1;
```
**Attendu** : ligne `(remove_password, <admin_pseudo>, alice, password cleared)`.

#### Test 3 — Helper écrit en DB (via wall-delete)
```bash
# Setup : un post mur de Bob
curl -X DELETE -H "Cookie: rev0auth_admin_session=<TOKEN>" \
  http://localhost:5173/japprends/wall/<ID_POST_BOB>
```
**Vérif DB** : nouvelle entrée action=`wall_delete`, target=`<ID_POST_BOB>`, detail=`wall post removed`.

---

### B. Audit endpoint — `GET /japprends/audit/data`

#### Test 4 — Sans session admin → 401
```bash
curl -i http://localhost:5173/japprends/audit/data
```
**Attendu** : `401`.

#### Test 5 — Endpoint JSON avec session admin
```bash
curl -i -H "Cookie: rev0auth_admin_session=<TOKEN>" \
  http://localhost:5173/japprends/audit/data
```
**Attendu** : `200`, array JSON d'entries `{id, action, actorPseudo, target, detail, createdAt}`.

#### Test 6 — Paramètre limit
```bash
curl -H "Cookie: rev0auth_admin_session=<TOKEN>" \
  "http://localhost:5173/japprends/audit/data?limit=5"
```
**Attendu** : max 5 entries.

---

### C. Audit page — `/japprends/audit`

#### Test 7 — Sans session → redirect login
Navigateur sans cookie admin → ouvrir `http://localhost:5173/japprends/audit`.
**Attendu** : redirect `303` vers `/japprends/login`.

#### Test 8 — Avec session admin → page affichée
Naviguer en étant admin connecté → `http://localhost:5173/japprends/audit`.
**Attendu** : titre "Audit log admin", table avec les entries générées aux tests #2 et #3. Filtre fonctionnel (taper "remove" filtre).

#### Test 9 — Bouton refresh
Cliquer "Rafraîchir" après avoir créé une nouvelle action (depuis un autre onglet) → la liste se met à jour.

---

### D. SongSurf logs — `/japprends/songsurf-logs`

#### Test 10 — Endpoint sans config env → 503
S'assurer que `SONGSURF_JWT_SECRET` et `SONGSURF_URL` ne sont **pas** définis.
```bash
curl -i -H "Cookie: rev0auth_admin_session=<TOKEN>" \
  http://localhost:5173/japprends/songsurf-logs
```
**Attendu** : `503`, `{"success":false,"error":"SongSurf non configuré"}`.

#### Test 11 — Endpoint avec config + SongSurf joignable
Avec `.env` configuré ET SongSurf qui tourne sur l'URL :
```bash
curl -H "Cookie: rev0auth_admin_session=<TOKEN>" \
  "http://localhost:5173/japprends/songsurf-logs?pseudo=alice&limit=10"
```
**Attendu** : `200`, JSON de SongSurf. Vérifier dans les logs SongSurf qu'une requête `GET /api/admin/dl-logs?pseudo=alice&limit=10` est arrivée avec un JWT valide en Cookie.

#### Test 12 — Endpoint avec SongSurf injoignable
SONGSURF_URL=http://localhost:9999 (port mort), curl → `502`, `{"success":false,"error":"SongSurf injoignable"}`.

#### Test 13 — Page avec session admin
Navigateur → `http://localhost:5173/japprends/songsurf-logs`.
**Attendu** : formulaire pseudo + limit + bouton "Charger". Cliquer Charger affiche soit JSON soit erreur.

#### Test 14 — Page sans session admin → redirect
**Attendu** : `303` vers `/japprends/login`.

---

### E. Members dashboard redirect

#### Test 15 — `/members/dashboard` → 301 vers `/home/friend`
```bash
curl -i http://localhost:5173/members/dashboard
```
**Attendu** : `HTTP/1.1 301`, header `Location: /home/friend`.

#### Test 16 — Suit le redirect en navigateur
Naviguer vers `/members/dashboard` → l'URL devient `/home/friend` et la page friend s'affiche.

---

## Validation type-check

```bash
cd /home/revoli/dev/rev0Univers/auth/frontend
npm run check
```
**Attendu** : `0 ERRORS`. Warnings : 22 (1 nouveau, cohérent avec le pattern existant des autres pages — `data` référencé directement, à corriger globalement plus tard si souhaité).

---

## Si un test échoue

- **Test 1** (table) : vérifier que `initDb()` est bien await dans `hooks.server.ts` au boot. Sinon CREATE TABLE n'est pas exécuté.
- **Tests 4-6** (endpoint audit JSON) : l'endpoint est désormais sur `/japprends/audit/data` (pas `/japprends/audit`) pour éviter tout conflit avec la page.
- **Test 10** : si retourne 200 au lieu de 503, c'est qu'il y a déjà une config env partielle. `unset SONGSURF_URL` et redémarrer.
- **Test 11** : JWT secret doit matcher exactement (32 bytes min) avec celui de SongSurf, sinon SongSurf retournera 401 → on aura 502.
