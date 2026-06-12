# Tests à effectuer — handoff Claude

> **Ce fichier est réécrit à chaque nouvelle tâche par le Claude qui code.**
> Il décrit la **dernière tâche réalisée** et liste **les tests à exécuter** pour la valider.
> Branche : `dev/svelte` · Projet : `auth/frontend/`

---

## Dernière tâche réalisée

**Date** : 2026-06-12
**Tâche** : **Composeur d'avatar DiceBear « initial-face » + emojis OpenMoji côté admin**.

### Avatars — composeur « initial-face » (essai adventurer annulé sur choix user)
- **Style initial-face** (API 10.x) : visage dérivé de l'initiale du pseudo ; composable : **8 yeux, 2 têtes, 9 couleurs de fond**. Licence **CC0** (aucune mention requise).
- **`$lib/avatar-options.ts`** : catalogue généré depuis la définition officielle (`@dicebear/styles@10.1.0`), partagé client/serveur. `buildAvatarParams()` = validation **whitelist** (l'API DiceBear ignore silencieusement les valeurs inconnues — la validation est chez nous).
- **Composeur dans le profil** : aperçu **live** 140px + steppers Yeux/Tête + nuancier Fond + 🎲 Aléatoire + Sauvegarder.
- **Preview live durcie** (retour user "il faut sauvegarder+refresh") : `$derived.by` avec spread de `opts` (lecture explicite de chaque champ → tracking Svelte garanti) + message d'erreur visible si la génération DiceBear échoue (au lieu d'une image cassée). Si la preview semble figée : hard-refresh (Ctrl+Shift+R), HMR possiblement stale.
- **Contrat** : `POST /members/avatar { seed, options: {eyes, head, backgroundColor} }` ; `GET /avatars/[seed]?eyes=…&head=…&backgroundColor=…` (les 3 ensemble, sinon 400). Sans options = avatar aléatoire dérivé du seed (grille signup + fallback par pseudo inchangés).

### Emojis OpenMoji côté dashboard admin
- **MessagesTab** : la barre d'emojis unicode locale → 15 OpenMoji partagés (`$lib/emojis.ts`), rendu des bulles via `EmojiText`.
- **WallTab** : picker ajouté au composer + rendu des posts via `EmojiText`.

### État : testé en local ✅
| Test | Résultat |
|---|---|
| Aperçu composé `GET /avatars/x?hair=…` → 200 svg ; option hors whitelist → 400 | PASS |
| Aperçu sans options (signup/fallback) → 200 | PASS |
| `POST /members/avatar {seed, options}` → stocké + servi depuis la DB | PASS |
| Options malveillantes (`hair: "<svg>"`) → 400 | PASS |
| `npm run check` 0 erreur / 8 warnings · vitest 26/26 | PASS |

---

## Tests restants pour la prochaine session

### 1. Navigateur
- [ ] Composeur profil : steppers, nuanciers, 🎲, aperçu live, Sauvegarder → avatar mis à jour partout (navbar, membres, chat)
- [ ] Emojis OpenMoji dans le dashboard admin (Messages + Mur) : picker et rendu
- [ ] Signup : grille de 8 avatars initial-face aléatoires réactive au pseudo
- [ ] **Preview live du composeur** : chaque clic stepper/nuancier doit changer l'aperçu sans sauvegarder (hard-refresh d'abord si page ouverte avant les changements)
- [ ] Hérités : chat temps réel (2 navigateurs), delete account, login admin

### 2. Deploy VPS
Checklist Phase 3 + sortie réseau vers api.dicebear.com.

### 3. Validation type-check
```bash
cd /home/revoli/dev/rev0Univers/auth/frontend
npm run check   # attendu : 0 ERRORS / 8 WARNINGS
npm test        # attendu : 26/26
```

---

## Prochaine étape

Deploy VPS + tests prod, puis revue visuelle (shadcn-svelte/MyCss — y intégrer les mentions DiceBear CC BY + OpenMoji CC BY-SA), et profils membres custom.
