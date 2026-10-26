# console-admin.md — La console admin sur le réseau privé (AUTH-11)

La console (`/japprends/*`) n'existe pas sur le portail public : elle a son
propre nom, `admin.<domaine du portail>`, que Caddy ne sert qu'aux appareils
du réseau privé (Tailscale). Même avec un mot de passe et une clé volés, elle
est injoignable depuis Internet.

---

## Comment ça marche

```
Internet ──► rev0li.duckdns.org/japprends/…        → 404 (Caddy, puis l'appli)
tailnet  ──► admin.rev0li.duckdns.org/japprends/…  → console (rôle admin + clé)
```

- **Même appli, deux noms.** `ADMIN_ORIGIN` donne le nom de la console ;
  `frontend/src/lib/server/console-host.ts` décide ce que chaque nom montre,
  d'après l'en-tête `Host` posé par Caddy :
  - portail public : `/japprends/*` → 404, sauf `/japprends/api/songsurf-events`
    (le NAS y pousse ses évènements, authentifiés par secret partagé) ;
  - nom de la console : la console, `/_app/*` et `/auth/logout` ; `/` mène à
    l'entrée de la console, le reste renvoie au portail public.
- **Connexion par la clé seule** (AUTH-21) : sans session, l'entrée de la
  console demande la clé d'elle-même. La clé désigne l'admin (toutes les clés
  des admins actifs sont proposées), **PIN de la clé exigé** (vérification de
  l'utilisateur WebAuthn) ; une session BetterAuth normale est alors ouverte
  et déjà validée pour la console. Le défi est lié au navigateur par un cookie
  httpOnly de 5 min, à usage unique. Session ouverte par le plugin serveur
  `admin-key-session` (`auth-v2.ts`, `SERVER_ONLY` : jamais routé).
- **Connexion par mot de passe** (toujours possible) : sur le portail public,
  puis retour à la console (`/?redirect=…`) et clé pour valider la session.
  La session est partagée par le domaine parent (AUTH-07).
- **Durée** : la console reste ouverte 12 h, puis redemande la clé.
- **Bouton de l'espace membre** : vérifie que la console répond (fichier de
  version) ; hors réseau privé, il affiche « Console : active Tailscale ».
- **Clé de sécurité** : le RP ID reste le nom du portail (domaine parent),
  valable pour la console : les clés déjà enregistrées marchent toujours.
  L'origine WebAuthn attendue est `ADMIN_ORIGIN`.
- **Liens d'invitation** : construits sur le portail public, pas sur le nom de
  la console.
- Sans `ADMIN_ORIGIN` (dev) : pas de séparation, console sous `/japprends`.

---

## Le nom de la console depuis le tailnet

Le nom `admin.…` est public (DuckDNS résout tous les sous-noms vers l'IP
publique du VPS) : c'est ce qui permet d'avoir son certificat par le challenge
HTTP. Mais Caddy coupe toute requête qui ne vient pas d'une adresse Tailscale
(`100.64.0.0/10`, `fd7a:115c:a1e0::/48`).

Les appareils du tailnet doivent donc résoudre ce nom vers l'**IP Tailscale**
du VPS :

1. Un petit serveur DNS sur le VPS (dnsmasq), à l'écoute de l'interface
   Tailscale seulement, qui répond pour ce seul nom.
2. Dans la console Tailscale, DNS › nameserver personnalisé = IP Tailscale du
   VPS, **restreint au domaine** `admin.<domaine>` (split DNS). Tous les
   appareils du tailnet, téléphone compris, l'utilisent pour ce nom seulement.

Vérifier depuis un appareil du tailnet : `dig +short admin.<domaine>` doit
donner l'IP Tailscale du VPS. Hors tailnet, il donne l'IP publique et la
connexion est coupée.

---

## Mise en place sur le VPS

1. DNS : dnsmasq + split DNS Tailscale (ci-dessus).
2. Caddy : bloc `admin.{$WEB_DOMAIN}` et 404 sur `/japprends` du portail,
   modèle dans `infra/caddy/Caddyfile.template`.
3. `.env` : `ADMIN_ORIGIN=https://admin.<domaine>`, puis déploiement.

Dans cet ordre : une fois l'appli déployée, la console n'est plus servie sur
le portail public.
