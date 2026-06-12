# Tests à effectuer — handoff Claude

> **Ce fichier est réécrit à chaque nouvelle tâche par le Claude qui code.**
> Il décrit la **dernière tâche réalisée** et liste **les tests à exécuter** pour la valider.
> Branche : `dev/svelte` · Projet : `auth/frontend/`

---

## Dernière tâche réalisée

**Date** : 2026-06-12
**Tâche** : **Polish du côté friend/membre** (5 demandes user).

### Changements
1. **Hero** : steps GitHub/star supprimés, emoji 👋 supprimé.
2. **"Ton statut" supprimé** (UI + endpoint `/members/status` + pastilles navbar/membres). La colonne `web_users.status` reste en DB (signup y écrit encore, l'admin la voit éventuellement).
3. **Messagerie refondue en popup** (`routes/home/friend/Chat.svelte`, FAB 💬 sur /home/friend) :
   - Fil **Admin permanent** épinglé en tête (badge "support"), même sans messages
   - Conversations privées membre↔membre + **recherche de membre** pour en démarrer une
   - API `/members/messages` réécrite : GET = threads groupés (unread par fil), GET `?with=` = fil chronologique, POST valide le destinataire (admin ou membre actif), PATCH `{with}` = marque le fil lu
   - **Onglet Messages supprimé de /members/profile** (load allégé)
   - **Confidentialité** : `/japprends/messages` (admin) filtré sur les conversations impliquant l'admin — les DM membres sont privés
4. **GitHub/LinkedIn retirés du profil** → conditions des demandes de service sur /home/friend :
   - SongSurf : ⭐ star `github.com/Rev0li/SongSurf` + pseudo GitHub requis
   - Jellyfin : recommandation `linkedin.com/in/oliver-kientzler` + nom LinkedIn requis
   - `/members/access/request` exige et stocke le pseudo (validation regex GitHub)
5. **Profil** : Bio conservée (futurs profils custom), **Commentaire supprimé** ; PUT `/members/profile/data` n'accepte plus que `{bio}`.

### État : testé en local ✅ (13 cas API + rendu HTML)

| Test | Résultat |
|---|---|
| Threads vides / envoi à admin / envoi à membre / destinataire inconnu 404 | PASS |
| Threads groupés avec unread, mark-read par fil, fil chronologique | PASS |
| Admin ne voit QUE ses conversations (DM membre↔membre invisibles) | PASS |
| Reply admin → apparaît dans le fil Admin du membre | PASS |
| Demande SongSurf avec pseudo GitHub stocké ; sans pseudo → 400 | PASS |
| PUT profil : bio seule (commentary envoyé = ignoré) | PASS |
| `/members/status` → 404 | PASS |
| HTML friend : plus de statut/hero-steps/👋, nouveaux flows services présents | PASS |
| HTML profil : plus de Messages/Commentaire/GitHub/LinkedIn | PASS |
| `npm run check` | 0 erreur / 7 warnings |
| `npm test` | 26/26 |

Comptes de test locaux : `MigrTester`/`MigrTest123!` ; `bob` (mdp retiré après test).

---

## Tests restants pour la prochaine session

### 1. Navigateur (la popup chat est client-side, non testable en curl)
- [ ] FAB 💬 : ouvrir, fil Admin épinglé, envoyer un message à l'admin
- [ ] Recherche de membre → démarrer un DM → bulles, scroll auto, badge unread qui se met à jour
- [ ] Demande SongSurf/Jellyfin depuis les cartes services (input + bouton désactivé si vide)
- [ ] Onboarding première connexion toujours OK (modal)
- [ ] Hérités : login admin formulaire (challenge 🔒), audit refresh, grilles avatars

### 2. Deploy VPS
Checklist Phase 3 inchangée (voir version précédente de ce fichier dans git ou `migration-svelte-betterauth.md`) : compose down/up, `FRONTEND_UPSTREAM` Caddy, script migration comptes, TOTP admin.

### 3. Validation type-check
```bash
cd /home/revoli/dev/rev0Univers/auth/frontend
npm run check   # attendu : 0 ERRORS / 7 WARNINGS
npm test        # attendu : 26/26
```

---

## Prochaine étape

Deploy VPS + tests prod (tout le travail du 2026-06-12 n'est visible qu'après le switch Caddy). Ensuite : revue visuelle complète (shadcn-svelte/MyCss), vrais profils membres "custom" (la bio est gardée pour ça), plugin passkey BetterAuth, dashboard admin externe.
