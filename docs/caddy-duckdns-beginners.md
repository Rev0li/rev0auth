# Caddy + DuckDNS (pour les nuls)

Date: 2026-04-03

## Objectif

Avoir un reverse proxy sur le VPS (pas sur le NAS) avec tes domaines DuckDNS.

## Ton cas (resume)

- avant: domaines DuckDNS -> NAS/Tailscale
- maintenant: Caddy tourne sur le VPS
- certains services restent sur le NAS via Tailscale (`100.121.1.89`)

## Important

Le token DuckDNS ne doit pas apparaitre dans les commits ni dans le Caddyfile.
S'il a ete expose, fais une rotation du token DuckDNS.

## Etape 1 - DNS

Pour chaque sous-domaine, le DNS doit pointer vers l'IP publique du VPS.

Exemple:

- `revoli-portfolio.duckdns.org` -> VPS
- `revoli-songsurf.duckdns.org` -> VPS
- `revoli-jellyfin.duckdns.org` -> VPS
- `hi-revoli.duckdns.org` -> VPS

## Etape 2 - Ports VPS

Ouvrir uniquement:

- `80/tcp`
- `443/tcp`

Le reste doit rester ferme.

## Etape 3 - Caddyfile simple (recommande)

Si tu utilises Caddy standard (apt), commence sans `acme_dns`.
Let's Encrypt utilisera le challenge HTTP automatiquement.

```caddy
{
    email admin@example.com
}

revoli-portfolio.duckdns.org {
    reverse_proxy 127.0.0.1:3000
}

revoli-songsurf.duckdns.org {
    reverse_proxy 100.121.1.89:8080
}

revoli-jellyfin.duckdns.org {
    reverse_proxy 100.121.1.89:8096
}

hi-revoli.duckdns.org {
    # Option simple: un service frontal de logs
    reverse_proxy 127.0.0.1:9000

    # Si tu veux router par chemin, remplace par des handles:
    # handle_path /jellyfin/* {
    #     reverse_proxy 100.121.1.89:8096
    # }
    # handle_path /songsurf/* {
    #     reverse_proxy 100.121.1.89:8080
    # }
}
```

## Etape 4 - Installer / valider Caddy

Sur le VPS:

```bash
sudo ./scripts/install-caddy-template.sh --dry-run
sudo ./scripts/install-caddy-template.sh
```

Si tu utilises le Caddyfile ci-dessus en direct:

```bash
sudo caddy validate --config /etc/caddy/Caddyfile --adapter caddyfile
sudo systemctl restart caddy
sudo systemctl status caddy --no-pager
```

## Etape 5 - Verifier que le VPS reach le NAS

Le VPS doit avoir Tailscale et joindre `100.121.1.89`.

```bash
curl -I http://100.121.1.89:8080
curl -I http://100.121.1.89:8096
```

Si ca ne repond pas:

- verifier Tailscale sur VPS et NAS
- verifier ACL tailnet
- verifier firewall NAS

## Etape 6 - Tests finaux

```bash
curl -I https://revoli-portfolio.duckdns.org
curl -I https://revoli-songsurf.duckdns.org
curl -I https://revoli-jellyfin.duckdns.org
curl -I https://hi-revoli.duckdns.org
```

## Si tu veux absolument `acme_dns duckdns`

Ca demande un Caddy build avec le module DNS DuckDNS.
Ce n'est pas le chemin le plus simple pour commencer.

Recommandation:

1. d'abord faire fonctionner le mode simple (HTTP challenge)
2. ensuite seulement passer en DNS challenge si necessaire

## Liens

- [Install to Launch](install-to-launch.md)
- [AUTH-009 DuckDNS](auth-009-duckdns.md)
- [Operations Index](operations/README.md)