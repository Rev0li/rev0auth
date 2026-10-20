# Caddy + DuckDNS

Mis à jour : 2026-10-20 (AUTH-06)

## Le principe

Caddy, sur le VPS, reçoit tout le trafic HTTPS et le renvoie au bon service :
le portail rev0auth (conteneur local) ou les services du NAS (via Tailscale).

Tout est sous un seul domaine DuckDNS :

| Adresse | Service |
|---|---|
| `rev0li.duckdns.org` | portail rev0auth |
| `songsurf.rev0li.duckdns.org` | SongSurf (NAS) |
| `jellyfin.rev0li.duckdns.org` | Jellyfin (NAS) |
| `portfolio.rev0li.duckdns.org` | portfolio (NAS) |

Les anciennes adresses (`rev0lisurf`, `revoli-jellyfin`, `revoli-portfolio`)
redirigent en 308 vers les nouvelles, le temps de mettre à jour les liens et
les applis.

## DNS : rien à déclarer

DuckDNS résout n'importe quel nom sous `rev0li.duckdns.org` vers la même IP
que `rev0li.duckdns.org`. Ajouter un service = ajouter un bloc au Caddyfile.

## Certificats : challenge HTTP, sans jeton

Pour chaque nom, Let's Encrypt appelle
`http://<nom>/.well-known/acme-challenge/…` et Caddy répond : ça prouve que le
nom pointe vers ce serveur. Il faut seulement les ports 80 et 443 ouverts.
Caddy obtient et renouvelle un certificat par nom, tout seul.

Le challenge DNS (`acme_dns duckdns <token>`) n'est plus utilisé. Il demandait
le jeton du compte DuckDNS sur le serveur, et DuckDNS ne publie qu'une valeur
de vérification à la fois, ce qui fait échouer un certificat qui couvre en
même temps le domaine et ses sous-domaines.

Contrepartie : chaque certificat est publié dans les journaux de
transparence, donc les noms des sous-domaines sont publics.

## Appliquer une modification

Modèle : `infra/caddy/Caddyfile.template` (variables dans
`infra/caddy/caddy.env.example`). Sur le VPS, le Caddyfile est
`/etc/caddy/Caddyfile` (`640 root:caddy`).

```bash
sudo cp /etc/caddy/Caddyfile /etc/caddy/Caddyfile.bak-$(date +%F)
sudo caddy validate --config /etc/caddy/Caddyfile --adapter caddyfile
sudo systemctl reload caddy
sudo journalctl -u caddy --since "5 min ago" | grep -iE "obtain|error"
```

Vérifier :

```bash
curl -sI https://songsurf.rev0li.duckdns.org | head -1
curl -sI https://rev0lisurf.duckdns.org | grep -i location
```

## Liens

- [Install to Launch](install-to-launch.md)
- [AUTH-009 DuckDNS](auth-009-duckdns.md)
- [Operations Index](operations/README.md)
