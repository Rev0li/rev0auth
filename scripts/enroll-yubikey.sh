#!/usr/bin/env bash
# enroll-yubikey.sh — YubiKey WebAuthn enrollment (bootstrap, no password needed)
#
# Flow:
#   1. Detects your YubiKey (waits until plugged in)
#   2. Checks the rev0auth web server is running on :3000
#   3. Opens the browser at /japprends/login (shows bootstrap registration UI)
#   4. You register the key in the browser (touch it when prompted)
#   5. Script fetches the credential JSON from the server
#   6. Saves ADMIN_WEBAUTHN_CREDENTIAL to .env
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ENV_FILE="$ROOT/.env"

GREEN='\033[0;32m'; YELLOW='\033[1;33m'; CYAN='\033[0;36m'; RED='\033[0;31m'; BOLD='\033[1m'; NC='\033[0m'
ok()    { echo -e "${GREEN}✓${NC} $*"; }
warn()  { echo -e "${YELLOW}⚠${NC}  $*"; }
err()   { echo -e "${RED}✗${NC} $*" >&2; exit 1; }
info()  { echo -e "${CYAN}→${NC} $*"; }
step()  { echo -e "\n${BOLD}${CYAN}[$1]${NC} $2"; }

WEB_URL="http://localhost:3000"
LOGIN_URL="$WEB_URL/japprends/login"
EXPORT_URL="$WEB_URL/japprends/webauthn/credential/export"

# ── Preflight ────────────────────────────────────────────────────────────────

[[ -f "$ENV_FILE" ]] || err ".env not found — run ./scripts/gen_secret.sh first."
command -v curl >/dev/null 2>&1 || err "curl is required but not found."

echo ""
echo -e "${CYAN}${BOLD}╔══════════════════════════════════════════╗${NC}"
echo -e "${CYAN}${BOLD}║     rev0auth — YubiKey Enrollment        ║${NC}"
echo -e "${CYAN}${BOLD}╚══════════════════════════════════════════╝${NC}"

# Show current state
current=$(grep -E '^ADMIN_WEBAUTHN_CREDENTIAL=' "$ENV_FILE" | head -1 \
          | cut -d= -f2- | tr -d "'" | tr -d '"' || true)
if [[ -n "$current" ]]; then
    warn "Une clé est déjà enregistrée en mémoire. Pour re-enregistrer :"
    warn "  1. Connecte-toi au dashboard admin"
    warn "  2. Va dans Sécurité → Supprimer la clé"
    warn "  3. Relance ce script"
    echo ""
    read -r -p "Continuer quand même ? (o/N) : " confirm
    [[ "$confirm" =~ ^[oO]$ ]] || { info "Annulé."; exit 0; }
fi

# ── Step 1: Detect YubiKey ────────────────────────────────────────────────────
step "1/3" "Détection YubiKey"

detect_key() {
    if command -v ykman >/dev/null 2>&1; then
        ykman list 2>/dev/null | grep -qi "yubikey" && return 0 || return 1
    fi
    lsusb 2>/dev/null | grep -qi "yubico\|yubikey" && return 0 || return 1
}

if detect_key; then
    ok "YubiKey déjà connectée."
else
    info "Insère ta YubiKey maintenant…"
    while ! detect_key; do
        printf "."
        sleep 1
    done
    echo ""
    ok "YubiKey détectée !"
fi

if command -v ykman >/dev/null 2>&1; then
    key_info=$(ykman list 2>/dev/null | grep -i yubikey | head -1 || true)
    [[ -n "$key_info" ]] && info "Clé : $key_info"
fi

# ── Step 2: Check server ──────────────────────────────────────────────────────
step "2/3" "Vérification serveur"

if curl -sf "$WEB_URL/japprends/webauthn/status" -o /dev/null 2>&1 || \
   curl -sf "$WEB_URL/" -o /dev/null 2>&1; then
    ok "Serveur disponible sur $WEB_URL"
else
    err "Serveur inaccessible sur $WEB_URL — lance : make launch-all"
fi

# Check bootstrap mode (no credential registered yet)
STATUS_JSON=$(curl -sf "$WEB_URL/japprends/webauthn/status" 2>/dev/null || echo '{}')
REGISTERED=$(echo "$STATUS_JSON" | python3 -c "import sys,json; print(json.load(sys.stdin).get('registered', False))" 2>/dev/null || echo "False")

if [[ "$REGISTERED" == "True" ]]; then
    warn "Le serveur a déjà une clé enregistrée en mémoire (redémarrage requis pour reset)."
    warn "Si tu veux recommencer, arrête le serveur, vide ADMIN_WEBAUTHN_CREDENTIAL dans .env, et relance."
    echo ""
fi

# ── Step 3: Browser registration ─────────────────────────────────────────────
step "3/3" "Enregistrement via le navigateur"

info "Ouverture du navigateur → $LOGIN_URL"
if command -v xdg-open >/dev/null 2>&1; then
    xdg-open "$LOGIN_URL" 2>/dev/null &
elif command -v open >/dev/null 2>&1; then
    open "$LOGIN_URL" 2>/dev/null &
else
    warn "Impossible d'ouvrir le navigateur automatiquement."
    info "Ouvre manuellement : $LOGIN_URL"
fi

echo ""
echo -e "  Dans le navigateur :"
echo    "  1. La page affiche 'Configuration initiale'"
echo    "  2. Clique 'Enregistrer la clé YubiKey'"
echo    "  3. Touche ta clé quand le navigateur le demande"
echo    "  4. Attends la confirmation 'Clé enregistrée !'"
echo ""
echo -e "  ${CYAN}?${NC}  Appuie sur Entrée une fois la clé enregistrée dans le navigateur…"
read -r _

# ── Fetch credential from server ─────────────────────────────────────────────

info "Récupération du credential depuis le serveur…"
CRED_RESP=$(curl -sf "$EXPORT_URL" --output - 2>&1) || true

CREDENTIAL_JSON=$(echo "$CRED_RESP" | python3 -c "
import sys, json
d = json.load(sys.stdin)
if not d.get('ok'):
    print('ERROR: ' + d.get('message', 'unknown'), file=sys.stderr)
    sys.exit(1)
print(d['credential_json'])
" 2>/dev/null) || {
    warn "Impossible de récupérer le credential automatiquement."
    warn "La clé n'est peut-être pas encore enregistrée — refais le step navigateur."
    echo ""
    echo "Si le problème persiste, colle le JSON manuellement :"
    echo -e "  ${CYAN}?${NC}  Colle le credential JSON et appuie sur Entrée :"
    read -r CREDENTIAL_JSON
    [[ -n "$CREDENTIAL_JSON" ]] || err "Rien entré. Annulé."
}
ok "Credential reçu."

# ── Save to .env ──────────────────────────────────────────────────────────────
cp "$ENV_FILE" "${ENV_FILE}.bak"

if grep -qE '^ADMIN_WEBAUTHN_CREDENTIAL=' "$ENV_FILE"; then
    python3 - "$ENV_FILE" "$CREDENTIAL_JSON" <<'PY'
import sys
path, cred = sys.argv[1], sys.argv[2]
lines = open(path).readlines()
out = [f"ADMIN_WEBAUTHN_CREDENTIAL='{cred}'\n" if l.startswith('ADMIN_WEBAUTHN_CREDENTIAL=') else l for l in lines]
open(path, 'w').writelines(out)
PY
else
    echo "ADMIN_WEBAUTHN_CREDENTIAL='${CREDENTIAL_JSON}'" >> "$ENV_FILE"
fi
chmod 600 "$ENV_FILE"

echo ""
ok "Credential sauvegardé dans .env  (.env.bak créé)"
echo ""
echo -e "  ${CYAN}Ensuite :${NC}"
echo    "  make stop-all && make launch-all   — redémarre pour activer la clé"
echo    "  Puis va sur $LOGIN_URL — touche ta YubiKey pour te connecter."
echo ""
