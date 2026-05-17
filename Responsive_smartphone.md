# Responsive Smartphone — Refonte visuelle mobile

Scope : rev0auth (web crate)  
Priorité : future — à planifier après stabilisation fonctionnelle

---

## Contexte

L'interface actuelle est pensée desktop first.
Sur mobile les points de douleur identifiés :
- Les cards services (SongSurf / Jellyfin) se superposent ou débordent sur petits écrans
- La zone de composition du mur communautaire est difficile à utiliser au doigt
- Le dashboard admin (Members, Messages, Donations) n'a pas de layout mobile adapté
- La navbar ne se replie pas (pseudo + boutons dépassent)
- Le portal d'inscription : le `temp-password-box` peut être hors viewport sans scroll

---

## Breakpoints cibles

| Nom       | Largeur       | Usage typique          |
|-----------|---------------|------------------------|
| `sm`      | < 480 px      | Téléphone portrait      |
| `md`      | 480 – 768 px  | Téléphone paysage / petite tablette |
| `lg`      | > 768 px      | Desktop (état actuel)  |

---

## rev0auth — Pages à retravailler

### `/` (home — login)
- [ ] Centrer la card login, padding réduit sur `sm`
- [ ] Input + bouton full-width sous `sm`

### `/portal` (inscription)
- [ ] `temp-password-box` : s'assurer qu'il scroll en vue après apparition (`scrollIntoView`)
- [ ] Textarea referral : resize désactivé sur mobile (déjà fait), vérifier hauteur initiale

### `/home/friend` (espace membre)
- [ ] Navbar : replier pseudo + boutons dans un menu hamburger sous `sm`
- [ ] Hero steps : passer de `row` à `column` sous `sm`, arrows horizontales → verticales
- [ ] `services-grid` : 1 colonne sous `sm` (actuellement `auto-fill minmax(240px)`)
- [ ] Section mur : textarea compose, bouton Poster — vérifier zone tactile (min 44px)
- [ ] Chat FAB : vérifier position et taille du bouton flottant

### `/members/profile` (profil)
- [ ] Tabs profil : scroll horizontal ou stack vertical sous `sm`
- [ ] Section Don : `crypto-addr-val` (adresse longue) — word-break ou ellipsis

### `/japprends/*` (dashboard admin)
- [ ] Tab Messages : layout `aside + panel` → stack vertical sous `md`
- [ ] Tab Members : `member-gallery` grid → 1-2 colonnes sous `sm`
- [ ] Tab Donations : tableau → cards empilées sous `sm`
- [ ] Navigation tabs : scroll horizontal si overflow

---

