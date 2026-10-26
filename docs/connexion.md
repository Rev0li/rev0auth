# connexion.md — Mettre un service derrière la porte du portail

Les services de l'univers (SongSurf, bientôt Jellyfin) ne gèrent plus la
connexion eux-mêmes. Caddy demande au portail, avant chaque requête, si la
personne a le droit d'entrer (`forward_auth`). Plus de jeton à valider côté
service. Remplace l'ancien passage de JWT (`AUTH_JWT_SECRET`), retiré en AUTH-09.

---

## Comment ça marche

```
navigateur ──► Caddy ──► GET /auth/verify?service=<nom>  (portail, avec le cookie de session)
                 │
                 ├─ 204 + X-User-Id / X-User-Name / X-User-Role ──► service
                 ├─ 302 vers la connexion du portail (navigation sans session)
                 └─ 401 (appel d'API sans session)
```

- La session est celle du portail : un cookie posé sur le domaine parent,
  envoyé à tous les sous-domaines (AUTH-07). Caddy le retire avant de
  transmettre au service (`sans_cookie_portail`).
- Les `X-User-*` envoyés par le client sont effacés avant la vérification.
- Le service est fixé dans la config Caddy (`?service=`), jamais déduit de la
  requête. Liste dans `frontend/src/lib/server/forward-auth.ts` (`SERVICES`).
- Accès : tout membre connecté a accès à tous les services (AUTH-18) : on
  n'entre que sur invitation, il n'y a plus d'accès à accorder par service.

---

## Ce que le service reçoit

| En-tête | Contenu |
|---|---|
| `X-User-Name` | pseudo exact, unique, non modifiable |
| `X-User-Role` | `guest` · `member` · `mod` · `admin` |
| `X-User-Id` | identifiant du portail (UUID), stable |
| `X-Proxy-Token` | preuve que la requête vient de Caddy (secret partagé) |

**Le service doit vérifier `X-Proxy-Token`** (comparaison en temps constant)
avant de croire les `X-User-*`. Sans ça, n'importe quel appareil qui joint le
service directement (réseau local, Tailscale) pourrait se déclarer admin.
Preuve absente ou fausse → requête anonyme.

---

## Ajouter un service

1. Portail : ajouter le nom dans `SERVICES` (`forward-auth.ts`), avec ses
   tests (`forward-auth.test.ts`).
2. Secret de preuve : `openssl rand -hex 32`, posé dans l'environnement du
   service et dans un fichier privé du VPS (`/etc/caddy/secrets/…`, root:caddy,
   640) qui définit un snippet `header_up X-Proxy-Token "…"`. Pas de variable
   d'environnement pour Caddy : il est lancé avec `--environ` et l'écrirait
   dans son journal.
3. Caddy : sur le bloc du sous-domaine, `import porte_portail <nom>` puis le
   `reverse_proxy` avec `import sans_cookie_portail` et le snippet de preuve.
   Exempter le preflight CORS (`OPTIONS`) si une extension ou un autre site
   appelle le service. Modèle : `infra/caddy/Caddyfile.template`, bloc `songsurf.`.
4. Service : lire l'identité dans les en-têtes, vérifier la preuve, ne plus
   rediriger vers une page de connexion (Caddy s'en charge). Déconnexion =
   renvoyer vers le portail.

---

## Appels serveur du portail vers un service

Un appel fait par le portail lui-même passe aussi par la porte : il porte la
session de l'admin, comme le ferait son navigateur. Exemple :
`fetchSongsurfDlLogs` dans `frontend/src/lib/server/songsurf.ts` (seul le
cookie de session est repris, et seulement vers le domaine du portail en https).
