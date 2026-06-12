# Tests à effectuer — handoff Claude

> **Ce fichier est réécrit à chaque nouvelle tâche par le Claude qui code.**
> Il décrit la **dernière tâche réalisée** et liste **les tests à exécuter** pour la valider.
> Branche : `dev/svelte` · Projet : `auth/frontend/`

---

## Dernière tâche réalisée

**Date** : 2026-06-12
**Tâche** : **Retouches UI friend/profil/admin** (6 demandes user).

### Changements
1. **Profil/Avatar** : bouton "Supprimer" retiré (l'endpoint DELETE `/members/avatar` existe toujours, plus d'UI).
2. **Profil/Compte** : vrai bouton **"Supprimer mon compte"** dans la Zone dangereuse, avec étape de confirmation → `DELETE /members/account` → redirect `/`.
3. **Emojis** : rangée de 10 emojis (😄 😂 ❤️ 👍 🎉 🔥 💡 🎬 🍿 🎵) dans le composer du **chat** (Chat.svelte) et du **mur** ; clic = insertion dans le texte. Texte d'intro du mur : « Partage tes idées d'amélioration, ou les films et séries que tu aimerais voir ajoutés à Jellyfin 🍿 ».
4. **Theme toggle dans le header** : `ThemeToggle` a un mode `inline` ; navbar friend = toggle (le bouton Déconnexion y est retiré), navbar profil = toggle + Déconnexion. Le layout n'affiche plus le bouton flottant sur ces deux pages (il reste partout ailleurs : login, signup, admin).
5. **Fix flash d'onglets profil** : `transition:fade` → `in:fade` (l'ancien onglet ne fait plus d'animation de sortie, donc plus de superposition visible).
6. **Admin/Membres** : bouton d'accès **GitHub retiré** (restent SongSurf et Jellyfin).

### État : validé ✅
- `npm run check` 0 erreur / 7 warnings · vitest 26/26
- SSR vérifié : friend (texte mur 🍿, emojis, toggle navbar, plus de Déconnexion), profil (toggle + Déconnexion, plus de bouton Supprimer avatar)

---

## Tests restants pour la prochaine session

### 1. Navigateur
- [ ] Onglet Compte : "Supprimer mon compte" → confirmation → suppression réelle (créer un compte jetable via invitation)
- [ ] Emojis chat + mur : insertion au clic, envoi correct
- [ ] Toggle thème dans les deux navbars + bouton flottant toujours présent sur login/signup/admin
- [ ] Changement d'onglet profil : plus de flash
- [ ] Hérités : popup chat complète (fil Admin, DM, recherche), demandes services, login admin formulaire

### 2. Deploy VPS
Checklist Phase 3 inchangée (`migration-svelte-betterauth.md`) : compose down/up, `FRONTEND_UPSTREAM` Caddy, script migration comptes, TOTP admin.

### 3. Validation type-check
```bash
cd /home/revoli/dev/rev0Univers/auth/frontend
npm run check   # attendu : 0 ERRORS / 7 WARNINGS
npm test        # attendu : 26/26
```

---

## Prochaine étape

Deploy VPS + tests prod, puis revue visuelle complète (shadcn-svelte/MyCss) et profils membres custom.
