#!/usr/bin/env bash
# vps-save-yubikey.sh — sauvegarde la YubiKey enrollée sur le VPS dans .env + recreate
#
# Workflow :
#   1. Enregistre ta clé via le navigateur (page bootstrap /japprends/login)
#   2. Touche la clé pour te connecter → tu as une session admin
#   3. DevTools → Application → Cookies → copie rev0auth_admin_session
#   4. Lance ce script
set -euo pipefail

SSH_KEY="$HOME/.ssh/vps"
SSH_PORT="4991"
VPS_USER="revovps"
VPS_HOST="94.23.107.22"
VPS_DIR="~/app/auth"

SSH="ssh -i $SSH_KEY -p $SSH_PORT $VPS_USER@$VPS_HOST"

GREEN='\033[0;32m'; CYAN='\033[0;36m'; RED='\033[0;31m'; NC='\033[0m'
ok()   { echo -e "${GREEN}✓${NC} $*"; }
info() { echo -e "${CYAN}→${NC} $*"; }
err()  { echo -e "${RED}✗${NC} $*" >&2; exit 1; }

echo ""
echo -e "${CYAN}╔══════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║   rev0auth — Sauvegarde YubiKey VPS      ║${NC}"
echo -e "${CYAN}╚══════════════════════════════════════════╝${NC}"
echo ""
echo "  Étapes préalables :"
echo "  1. https://rev0li.duckdns.org/japprends/login"
echo "  2. Enregistrer la clé (mode bootstrap)"
echo "  3. Toucher la clé pour se connecter"
echo "  4. DevTools → Application → Cookies → rev0auth_admin_session"
echo ""
read -rp "  Colle le cookie rev0auth_admin_session : " SESSION_COOKIE
[[ -n "$SESSION_COOKIE" ]] || err "Cookie vide. Annulé."

# ── Récupérer le credential depuis le VPS ────────────────────────────────────
info "Connexion au VPS et récupération du credential..."

CRED_JSON=$($SSH "curl -sf http://localhost:3000/japprends/webauthn/credential/export \
  -H 'Cookie: rev0auth_admin_session=$SESSION_COOKIE'" \
  | python3 -c "
import sys, json
try:
    d = json.load(sys.stdin)
except Exception as e:
    print(f'JSON invalide: {e}', file=sys.stderr); sys.exit(1)
if not d.get('ok'):
    print('Erreur: ' + d.get('message', 'réponse inattendue'), file=sys.stderr); sys.exit(1)
print(d['credential_json'])
") || err "Impossible de récupérer le credential. Cookie valide ? Serveur accessible ?"

ok "Credential récupéré."

# ── Mettre à jour .env sur le VPS ────────────────────────────────────────────
info "Mise à jour de .env sur le VPS..."

# Passe le JSON via stdin pour éviter les problèmes d'échappement
echo "$CRED_JSON" | $SSH "python3 - '$VPS_DIR/.env'" <<'REMOTE'
import sys, os

cred = sys.stdin.read().strip()
env_path = os.path.expanduser(sys.argv[1])

# Backup
import shutil
shutil.copy(env_path, env_path + '.bak')

lines = open(env_path).readlines()
found = False
out = []
for line in lines:
    if line.startswith('ADMIN_WEBAUTHN_CREDENTIAL='):
        out.append(f"ADMIN_WEBAUTHN_CREDENTIAL='{cred}'\n")
        found = True
    else:
        out.append(line)
if not found:
    out.append(f"ADMIN_WEBAUTHN_CREDENTIAL='{cred}'\n")

open(env_path, 'w').writelines(out)
os.chmod(env_path, 0o600)
print('ok')
REMOTE

ok ".env mis à jour (.env.bak créé)."

# ── Relancer le container web ─────────────────────────────────────────────────
info "Relance du container web..."
$SSH "cd $VPS_DIR && docker compose up -d --force-recreate web"

ok "Container relancé."
echo ""
echo "  Ta clé YubiKey est persistée et active."
echo "  Teste : https://rev0li.duckdns.org/japprends/login"
echo ""
