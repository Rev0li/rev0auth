#!/usr/bin/env bash
# gen_secret.sh — interactive secret generator for rev0auth
# Creates or updates .env with all required values.
# Usage: ./scripts/gen_secret.sh
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ENV_FILE="$ROOT/.env"

RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'; CYAN='\033[0;36m'; NC='\033[0m'
ok()   { echo -e "${GREEN}✓${NC} $*"; }
warn() { echo -e "${YELLOW}⚠${NC}  $*"; }
ask()  { echo -e "${CYAN}?${NC}  $*"; }
sep()  { echo -e "\n${CYAN}──────────────────────────────────────────${NC}"; }

gen_hex() { openssl rand -hex "${1:-32}"; }
gen_b32() {
    python3 -c "
import os, base64
raw = os.urandom(20)
print(base64.b32encode(raw).decode().rstrip('='))
" 2>/dev/null || openssl rand -base64 20 | tr '+/' 'AZ' | tr -d '=' | head -c 32
}

# Read current value of a key from .env (strips quotes)
current_val() {
    local key="$1"
    [[ -f "$ENV_FILE" ]] || { echo ""; return; }
    grep -E "^${key}=" "$ENV_FILE" | head -1 | cut -d= -f2- | tr -d "'" | tr -d '"' || true
}

prompt_value() {
    # prompt_value VARNAME "description" "default" "gen_command_or_empty"
    local key="$1" desc="$2" default="$3" gen_cmd="${4:-}"
    local current
    current=$(current_val "$key")

    sep
    echo -e "  Key  : ${CYAN}${key}${NC}"
    echo    "  Info : ${desc}"
    [[ -n "$current" ]]  && echo -e "  Current: ${YELLOW}${current:0:60}${NC}"
    [[ -n "$gen_cmd" ]]  && echo    "  (Enter = auto-generate)"
    [[ -z "$gen_cmd" && -n "$current" ]] && echo "  (Enter = keep current)"
    [[ -z "$gen_cmd" && -z "$current" && -n "$default" ]] && echo "  (Enter = use default: ${default})"

    ask "New value:"
    read -r input

    if [[ -z "$input" ]]; then
        if   [[ -n "$gen_cmd" ]]; then
            input=$(eval "$gen_cmd"); ok "Generated."
        elif [[ -n "$current" ]]; then
            input="$current";        ok "Kept existing."
        elif [[ -n "$default" ]]; then
            input="$default";        ok "Using default."
        else
            warn "Left empty."
        fi
    fi

    echo "${key}='${input}'" >> "$TMP_ENV"
    # Export so later prompts can reference it (e.g. DATABASE_URL uses POSTGRES_PASSWORD)
    export "${key}=${input}"
}

# ── Init ──────────────────────────────────────────────────────────────────────
TMP_ENV=$(mktemp)
trap 'rm -f "$TMP_ENV"' EXIT

echo ""
echo -e "${CYAN}╔══════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║    rev0auth — Secret & Config Generator  ║${NC}"
echo -e "${CYAN}╚══════════════════════════════════════════╝${NC}"
echo ""
echo "  Creates or updates ${CYAN}.env${NC}. Enter = auto-generate / keep current."
echo ""

# ── Database ─────────────────────────────────────────────────────────────────
sep
echo -e "  ${CYAN}── Database ──${NC}"
prompt_value "POSTGRES_PASSWORD" \
    "PostgreSQL password (used by Docker Compose postgres container)" \
    "" "gen_hex 24"

# Auto-build DATABASE_URL from the password just entered
_suggested_url="postgres://postgres:${POSTGRES_PASSWORD}@postgres:5432/rev0auth"
sep
echo -e "  Key  : ${CYAN}DATABASE_URL${NC}"
echo    "  Info : Full Postgres connection URL. Leave empty for Docker Compose default."
echo -e "  Suggested: ${YELLOW}${_suggested_url}${NC}"
echo    "  (Enter = use suggested, 'skip' = leave empty)"
ask "New value:"
read -r _db_input
if [[ "$_db_input" == "skip" || "$_db_input" == "" ]]; then
    # Leave empty — docker-compose injects the URL itself from POSTGRES_PASSWORD
    echo "DATABASE_URL=''" >> "$TMP_ENV"
    ok "Left empty (Docker Compose will build it from POSTGRES_PASSWORD)."
else
    echo "DATABASE_URL='${_db_input}'" >> "$TMP_ENV"
    ok "Set."
fi

# ── JWT / Auth ────────────────────────────────────────────────────────────────
sep
echo -e "  ${CYAN}── Auth ──${NC}"
echo    "  AUTH_JWT_SECRET must be identical in SongSurf .secrets."
prompt_value "AUTH_JWT_SECRET" \
    "JWT signing secret — min 32 bytes. Copy to SongSurf .secrets." \
    "" "gen_hex 32"

# ── Admin credentials ─────────────────────────────────────────────────────────
sep
echo -e "  ${CYAN}── Admin credentials ──${NC}"
prompt_value "ADMIN_DASH_PSEUDO"   "Admin username"                              "admin"   ""
prompt_value "ADMIN_DASH_SEED"     "Admin seed phrase (used for session signing)" ""       "gen_hex 16"
prompt_value "ADMIN_DASH_PASSWORD" "Admin login password (strong)"               ""        ""

# ── TOTP 2FA ──────────────────────────────────────────────────────────────────
sep
echo -e "  ${CYAN}── TOTP 2FA ──${NC}"
echo    "  Leave blank to skip. After generation, run: make admin-2fa-init"
echo    "  then scan the QR displayed with your authenticator app."
prompt_value "ADMIN_DASH_TOTP_SECRET" \
    "Base32 TOTP secret (empty = 2FA disabled)" \
    "" "gen_b32"

# ── Cookie domain (cross-domain SongSurf) ─────────────────────────────────────
sep
echo -e "  ${CYAN}── Cookie domain (SongSurf cross-domain) ──${NC}"
echo    "  Empty for local dev (cookie stays on localhost)."
echo    "  Set to root domain for prod: .rev0univers.com"
echo    "  This lets auth.rev0univers.com share the cookie with songsurf.rev0univers.com."
prompt_value "COOKIE_DOMAIN" \
    "Cookie Domain attribute (empty = localhost, .yourdomain.com = prod)" \
    "" ""

# ── SongSurf URL ─────────────────────────────────────────────────────────────
sep
echo -e "  ${CYAN}── SongSurf URL ──${NC}"
echo    "  URL du bouton 'Ouvrir Songsurf' affiché aux membres autorisés."
echo    "  Local : http://localhost:9000"
echo    "  Tailscale : http://<ip-tailscale>:9000 ou https://songsurf.xxx.ts.net"
prompt_value "SONGSURF_URL" \
    "URL d'accès à SongSurf (bouton membre)" \
    "http://localhost:9000" ""

# ── Donations ─────────────────────────────────────────────────────────────────
sep
echo -e "  ${CYAN}── Donation addresses (optional) ──${NC}"
echo    "  Format: NAME:ADDRESS,NAME:ADDRESS  — leave empty to disable."
prompt_value "DONATION_CRYPTO_ADDRESSES" \
    "Crypto donation addresses shown to members" \
    "" ""

# ── Write ─────────────────────────────────────────────────────────────────────
sep
echo ""

if [[ -f "$ENV_FILE" ]]; then
    cp "$ENV_FILE" "${ENV_FILE}.bak"
    warn "Backed up existing .env → .env.bak"
fi

cp "$TMP_ENV" "$ENV_FILE"
chmod 600 "$ENV_FILE"

echo ""
ok "Written to ${ENV_FILE} (chmod 600)"
echo ""
echo -e "  ${CYAN}Next steps:${NC}"
echo    "  1. make launch-all                  — build Docker image + start all services"
echo    "  2. Open http://localhost:4173/japprends/login — verify admin login + TOTP"
echo    "  3. Copy AUTH_JWT_SECRET to SongSurf .secrets (must be identical)"
echo ""
