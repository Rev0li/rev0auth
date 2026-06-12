# Tests à effectuer — handoff Claude

> **Ce fichier est réécrit à chaque nouvelle tâche par le Claude qui code.**
> Il décrit la **dernière tâche réalisée** et liste **les tests à exécuter** pour la valider.
> Branche : `dev/svelte` · Projet : `auth/frontend/`

---

## Dernière tâche réalisée

**Date** : 2026-06-12
**Tâche** : **Avatars DiceBear initial-face + emojis OpenMoji + notifications quasi temps réel**.

### Avatars — DiceBear « initial-face »
- Le style n'existe que via l'**API HTTP 10.x** (pas de package npm) → fetch **côté serveur uniquement** (`lib/server/dicebear.ts`, cache mémoire 300 entrées, timeout 5 s).
- **Stockage inchangé** : le SVG choisi est écrit dans `web_users.avatar_bytes` → le service des avatars reste 100 % local pour les visiteurs.
- `GET /avatars/[seed]` (public) : proxy d'aperçu pour les grilles (le navigateur ne parle jamais à api.dicebear.com), cache navigateur 24 h.
- **Nouveau contrat** : `POST /members/avatar {seed}` (plus `{avatar_id}`) ; signup envoie `avatar_seed`. Seed validé `/^[a-zA-Z0-9_-]{1,48}$/`.
- **Grilles** : 8 variantes seedées par le pseudo (`pseudo`, `pseudo-2`…`pseudo-8`) — profil + signup (réactive à la frappe du pseudo).
- **Fallback** : sans avatar choisi, `GET /members/avatar/[pseudo]` sert initial-face seedé par le pseudo (plus de 404).
- Catalogue `$lib/avatars.ts` (fox/wolf/…) **supprimé**.
- ⚠️ Dépendance externe **au choix d'avatar seulement** : si api.dicebear.com est down, la sélection échoue (502) mais les avatars déjà stockés restent servis.

### Emojis — OpenMoji (CC BY-SA 4.0)
- 15 SVG auto-hébergés dans `static/openmoji/` : 😀 🤣 😍 🥳 🤔 👍 ❤️ 🔥 🎉 💡 🎬 🍿 🎵 🚀 🦄
- `$lib/emojis.ts` (catalogue + `splitEmojis`) et `$lib/EmojiText.svelte` (rendu sans `@html`, sûr).
- Pickers chat + mur = images OpenMoji ; l'insertion reste le **caractère unicode** (DB inchangée) ; bulles de chat, aperçus de conversations et posts du mur rendus en OpenMoji.
- Licence CC BY-SA 4.0 → prévoir une mention « Emojis : OpenMoji.org » (footer/about) au moment de la revue visuelle.

### Notifications quasi temps réel (demande user en cours de session)
- Polling 8 s dans `Chat.svelte` : badge non-lus rafraîchi même popup fermée ; fil ouvert rafraîchi (scroll auto + mark-read) ; **pause quand l'onglet est caché** (`document.hidden`).
- Pas de WebSocket/SSE pour l'instant — à reconsidérer si besoin de vrai push (adapter-node le permettrait).

### État : testé en local ✅
| Test | Résultat |
|---|---|
| `GET /avatars/migrtester` → 200 svg ; seed invalide → 400 | PASS |
| `POST /members/avatar {seed}` → stocké, servi depuis la DB | PASS |
| Fallback avatar par pseudo (compte sans avatar) → 200 svg | PASS |
| `/openmoji/*.svg` servis ; 15 visibles dans le SSR friend | PASS |
| `npm run check` 0 erreur / 7 warnings · vitest 26/26 | PASS |

---

## Tests restants pour la prochaine session

### 1. Navigateur
- [ ] Grille profil : 8 variantes initial-face, sélection → Sauvegarder → l'avatar se met à jour (cache-bust `?v=`)
- [ ] Signup avec invitation valide : la grille apparaît quand le pseudo est valide et change avec lui
- [ ] Emojis OpenMoji : pickers chat + mur, rendu dans bulles/posts/aperçus
- [ ] Temps réel : deux navigateurs, envoyer un message → badge de l'autre se met à jour en ≤ 8 s sans reload
- [ ] Hérités : suppression de compte, toggle thème navbars, login admin formulaire

### 2. Deploy VPS
Checklist Phase 3 inchangée + **le VPS doit pouvoir joindre api.dicebear.com en sortie** (sélection d'avatar).

### 3. Validation type-check
```bash
cd /home/revoli/dev/rev0Univers/auth/frontend
npm run check   # attendu : 0 ERRORS / 7 WARNINGS
npm test        # attendu : 26/26
```

---

## Prochaine étape

Deploy VPS + tests prod, puis revue visuelle (shadcn-svelte/MyCss) avec mention licence OpenMoji, et profils membres custom.
