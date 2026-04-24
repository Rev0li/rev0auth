# Scan Architecture Web - 2026-04-04

## 1) Contexte

Objectif: preparer une refactorisation complete de l'architecture web pour la rendre maintenable, evolutive, et rapide a retoucher.

Scope scanne:
- backend web Rust (routing + handlers)
- pages HTML/CSS/JS embarquees
- structure frontend/admin dashboard

## 2) Resultat du scan (etat actuel)

### 2.1 Taille et concentration de code

- `main.rs` concentre trop de responsabilites.
  - `crates/web/src/main.rs` fait ~2286 lignes.
- `dashboard.rs` et `friend.rs` sont de gros blocs HTML+CSS+JS inline.
  - `crates/web/src/pages/dashboard.rs` ~1523 lignes.
  - `crates/web/src/pages/friend.rs` ~946 lignes.
  - `crates/web/src/pages/profile.rs` ~761 lignes.

Impact:
- maintenance lente
- regression facile
- difficultes a tester finement

### 2.2 Couplage fort routes + logique metier

Les routes et handlers sont majoritairement declares dans le meme fichier monolithique:
- routes: `crates/web/src/main.rs#L409`
- handlers multiples: `crates/web/src/main.rs#L521`

Impact:
- front, auth, moderation, messages, donations melanges
- faible lisibilite des boundaries

### 2.3 Frontend inline massif

Plusieurs pages contiennent leur CSS et JS inline:
- `crates/web/src/pages/dashboard.rs#L14` (style) et `crates/web/src/pages/dashboard.rs#L694` (script)
- `crates/web/src/pages/friend.rs#L11` (style) et `crates/web/src/pages/friend.rs#L501` (script)
- `crates/web/src/pages/profile.rs#L11` (style) et `crates/web/src/pages/profile.rs#L251` (script)

Impact:
- duplication de patterns (`bindEnterToClick`, `set*Msg`, `escapeHtml`)
- coherences UX difficiles a appliquer globalement

### 2.4 Styles partages existants mais non utilises

Un module style existe mais n'est pas exploite dans les pages majeures:
- `crates/web/src/styles/mod.rs#L1`

Impact:
- opportunite perdue de centralisation design system

### 2.5 Risques fonctionnels et evolution

- dashboard admin melange monitor, moderation, users, chat, donations dans un seul script
  - `crates/web/src/pages/dashboard.rs#L989` et suivants
- les flux messages ont evolue vite, la lisibilite de la logique conversationnelle reste fragile
  - `crates/web/src/pages/dashboard.rs#L1203`
  - `crates/web/src/main.rs#L1512`

Impact:
- toute nouvelle feature frontend peut casser des flux existants
- onboarding / chat / moderation se perturbent plus facilement

## 3) Probleme racine

1. Architecture page-centric en "string template" unique.
2. Aucun decoupage par domaines frontend (chat, onboarding, services, moderation).
3. JS utilitaire non mutualise.
4. Styles non factorises en tokens/variables communes exploitables partout.

## 4) Cible d'architecture (proposee)

## 4.1 Backend web Rust

Decouper `main.rs` en modules:

- `crates/web/src/app/router.rs`
  - composition des sous-routes
- `crates/web/src/app/state.rs`
  - `WebState` + constructeurs
- `crates/web/src/handlers/admin/*.rs`
  - auth, users, dashboard, moderation
- `crates/web/src/handlers/member/*.rs`
  - profile, messages, donations, status
- `crates/web/src/handlers/public/*.rs`
  - home, portal, login public
- `crates/web/src/models/*.rs`
  - DTO/response structs
- `crates/web/src/services/*.rs`
  - logique metier (messages, onboarding, access requests)

## 4.2 Frontend (sans changer de stack)

Conserver server-side pages Rust, mais externaliser CSS/JS:

- `crates/web/src/pages/templates/*.rs`
  - HTML structure seulement
- `crates/web/assets/css/`
  - `tokens.css`, `base.css`, `components.css`, `dashboard.css`, `member.css`
- `crates/web/assets/js/`
  - `core/dom.js`, `core/http.js`, `core/state.js`
  - `modules/chat.js`, `modules/onboarding.js`, `modules/services.js`, `modules/admin-dashboard.js`

## 4.3 Design system minimal

Definir une base stable:
- tokens couleur/spacing/radius/typography
- composants: card, button, input, badge, tabs, chat-bubble
- conventions d'etats: success/error/pending/disabled

## 4.4 Dashboard admin cible

Layout 3 zones:
1. barre KPI + filtres globaux
2. colonne conversations (threads)
3. panneau detail (conversation active + moderation contextuelle)

Sections separables:
- Monitoring
- Users & Access
- Chat & Support
- Donations & Review
- System diagnostics

## 5) Plan de refacto recommande (frontend first)

### Phase A - Stabilisation UI (rapide)

1. Extraire utilitaires JS communs (`bindEnterToClick`, `escapeHtml`, message toaster).
2. Extraire composants CSS communs (`card`, `actions`, `btn`, `status-msg`).
3. Uniformiser wording et feedback visuel.

### Phase B - Decoupage dashboard admin

1. Isoler chat admin en module dedie.
2. Isoler users/access en module dedie.
3. Isoler donations/review en module dedie.
4. Introduire un state central dashboard leger.

### Phase C - Decoupage member pages

1. `friend`: modules `chat`, `services`, `onboarding`, `status`.
2. `profile`: modules `profile-edit`, `messages`, `donations`, `avatar`.
3. Nettoyer duplication CSS/JS.

### Phase D - Backend architecture

1. Migration progressive des handlers hors `main.rs`.
2. Ajout d'un dossier services metier.
3. Tests ciblant chaque domaine (chat, onboarding, donations).

## 6) Gains attendus

- modifications frontend plus rapides
- baisse des regressions cross-feature
- meilleure lisibilite pour onboarding dev
- evolution dashboard admin plus propre vers V1/V2

## 7) Priorites immediates (ordre execution)

1. Refacto frontend dashboard admin (structure + modules JS)
2. Refacto frontend member (services/chat/onboarding)
3. Factorisation CSS design tokens
4. Decoupage backend `main.rs`

## 8) Risques migration et mitigation

1. Risque: casser routes actuelles.
   - mitigation: garder les paths stables et migrer handler par handler.
2. Risque: regressions JS silencieuses.
   - mitigation: checklist smoke test apres chaque lot.
3. Risque: divergence UX admin/member.
   - mitigation: composants CSS partages et patterns d'interaction communs.
