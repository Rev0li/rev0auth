# AUTH-009-DUCKDNS

Date: 2026-04-03

## Objectif

Exposer publiquement l API en HTTPS avec renouvellement automatique des certificats via Caddy (ACME), sur un domaine DuckDNS.

## Cible

- Domaine public: api.revoli.duckdns.org
- Reverse proxy local: http://127.0.0.1:8080
- Endpoint de verification: /health

## Prerequis

- API joignable localement sur le serveur VPS
- Ports 80 et 443 ouverts (UFW)
- Docker deja en place (ticket AUTH-009-VPS-SETUP)

## Caddyfile minimal

```caddy
api.revoli.duckdns.org {
  reverse_proxy 127.0.0.1:8080
}
```

## Template dynamique lie au projet

Le repo contient un template Caddy dynamique:

- `infra/caddy/Caddyfile.template`
- `infra/caddy/caddy.env.example`
- script d'installation: `scripts/install-caddy-template.sh`

Variables du template:

- `API_DOMAIN` (obligatoire)
- `API_UPSTREAM` (defaut: `127.0.0.1:8080`)
- `ACME_EMAIL` (optionnel)
- `CADDY_LOG_LEVEL` (defaut: `INFO`)

Installation VPS (recommande):

```bash
sudo ./scripts/install-caddy-template.sh --dry-run
sudo ./scripts/install-caddy-template.sh
```

Version Makefile:

```bash
make caddy-install ARGS="--dry-run"
make caddy-install
```

## Stockage securise du token DuckDNS

Recommandation: fichier root lisible uniquement root.

1. Creer le fichier secret:

```bash
sudo install -m 600 -o root -g root /dev/null /etc/duckdns.env
sudo bash -c 'cat > /etc/duckdns.env <<EOF
DUCKDNS_TOKEN=replace-me
DUCKDNS_DOMAIN=api.revoli
EOF'
```

2. Ne jamais versionner ce fichier dans le repo.

## Mise a jour IP DuckDNS (systemd)

Script de refresh:

```bash
sudo install -d -m 755 /usr/local/bin
sudo bash -c 'cat > /usr/local/bin/duckdns-update.sh <<"EOF"
#!/usr/bin/env bash
set -Eeuo pipefail
source /etc/duckdns.env
curl -fsS "https://www.duckdns.org/update?domains=${DUCKDNS_DOMAIN}&token=${DUCKDNS_TOKEN}&ip=" | grep -q "OK"
EOF'
sudo chmod 755 /usr/local/bin/duckdns-update.sh
```

Service systemd:

```ini
# /etc/systemd/system/duckdns-update.service
[Unit]
Description=DuckDNS update
After=network-online.target
Wants=network-online.target

[Service]
Type=oneshot
ExecStart=/usr/local/bin/duckdns-update.sh
```

Timer systemd:

```ini
# /etc/systemd/system/duckdns-update.timer
[Unit]
Description=Run DuckDNS update every 5 minutes

[Timer]
OnBootSec=30s
OnUnitActiveSec=5min
Unit=duckdns-update.service

[Install]
WantedBy=timers.target
```

Activation:

```bash
sudo systemctl daemon-reload
sudo systemctl enable --now duckdns-update.timer
sudo systemctl start duckdns-update.service
```

## Verification Caddy / HTTPS

1. Verification locale API:

```bash
curl -f http://127.0.0.1:8080/health
```

2. Verification domaine HTTPS:

```bash
curl -f https://api.revoli.duckdns.org/health
```

3. Verification certificat:

```bash
echo | openssl s_client -connect api.revoli.duckdns.org:443 -servername api.revoli.duckdns.org 2>/dev/null | openssl x509 -noout -issuer -dates
```

## Definition of done

- Token DuckDNS stocke hors repo et protégé
- Domaine DuckDNS resout vers le VPS
- Caddy sert un certificat valide (ACME auto-renew)
- /health repond en HTTPS depuis Internet

## Depannage rapide

- 404/502: verifier que l API ecoute bien sur 127.0.0.1:8080
- TLS non emis: verifier ports 80/443 entrants
- Domaine non resolu: verifier service duckdns-update.timer
- Timeout externe: verifier UFW et NAT/routeur si VPS maison
