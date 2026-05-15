#!/usr/bin/env bash
# Génère la ligne ADMIN_WEBAUTHN_CREDENTIAL à coller dans .env
# Prérequis : être connecté sur /japprends/login avec ta YubiKey

read -rp "Cookie rev0auth_admin_session : " COOKIE

curl -sf https://rev0li.duckdns.org/japprends/webauthn/credential/export \
  -H "Cookie: rev0auth_admin_session=$COOKIE" \
  | python3 -c "
import sys, json
d = json.load(sys.stdin)
print(f\"ADMIN_WEBAUTHN_CREDENTIAL='{d['credential_json']}'\")"
