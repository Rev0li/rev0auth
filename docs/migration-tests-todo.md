# Tests à effectuer — handoff Claude

> **Ce fichier est réécrit à chaque nouvelle tâche par le Claude qui code.**
> Il décrit la **dernière tâche réalisée** et liste **les tests à exécuter** pour la valider.
> Branche : `dev/svelte` · Projet : `auth/frontend/`

---

## Dernière tâche réalisée

**Date** : 2026-06-12
**Tâche** : **Composeur d'avatar DiceBear « adventurer » + emojis OpenMoji côté admin**.

### Avatars — passage d'initial-face à adventurer (choix user) + composeur
- **Style adventurer** (API 10.x) : 45 coiffures, 26 yeux, 30 bouches, 15 sourcils, 5 lunettes, 6 boucles, 4 détails (taches de rousseur…), 4 couleurs de peau, 14 couleurs de cheveux. Licence **CC BY 4.0** → mention « DiceBear » affichée sous le composeur.
- **`$lib/avatar-options.ts`** : catalogue généré depuis la définition officielle (`@dicebear/styles@10.1.0`), partagé client/serveur. `buildAvatarParams()` = validation **whitelist** (l'API DiceBear ignore silencieusement les valeurs inconnues — la validation est chez nous).
- **Composeur dans le profil** : aperçu live 140px + steppers ◀ n/N ▶ par section (sections optionnelles ont « Aucun ») + nuanciers peau/cheveux + bouton 🎲 Aléatoire + Sauvegarder.
- **Contrat** : `POST /members/avatar { seed, options }` ; `GET /avatars/[seed]?hair=…&eyes=…` (toutes les options requises ensemble, sinon 400). Sans options = avatar aléatoire dérivé du seed (grille signup + fallback par pseudo inchangés).
- Sections optionnelles : `glasses`/`earrings`/`details` à `''` → `*Probability=0` côté API.

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
- [ ] Signup : grille de 8 avatars adventurer aléatoires réactive au pseudo
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
