#!/usr/bin/env bash
# gen_secret.sh — interactive secret generator for rev0auth
# Generates or prompts for every sensitive value and writes them to .env
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ENV_FILE="$ROOT/.env"
ENV_EXAMPLE="$ROOT/.env.example"

# ---- colours ----
RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'; CYAN='\033[0;36m'; NC='\033[0m'
ok()   { echo -e "${GREEN}✓${NC} $*"; }
warn() { echo -e "${YELLOW}⚠${NC}  $*"; }
ask()  { echo -e "${CYAN}?${NC}  $*"; }
sep()  { echo -e "\n${CYAN}──────────────────────────────────────────${NC}"; }

# ---- helpers ----
gen_hex() { openssl rand -hex "${1:-32}"; }
gen_b32() {
    # generate a 20-byte random base32 secret (TOTP standard)
    python3 -c "
import os, base64
raw = os.urandom(20)
print(base64.b32encode(raw).decode().rstrip('='))
" 2>/dev/null || openssl rand -base64 20 | tr '+/' 'AZ' | tr -d '=' | head -c 32
}

prompt_value() {
    # prompt_value KEY "description" "default or empty" "generate_command or empty"
    local key="$1" desc="$2" default="$3" gen_cmd="${4:-}"
    local current=""

    # read existing value from .env if present
    if [[ -f "$ENV_FILE" ]]; then
        current=$(grep -E "^${key}=" "$ENV_FILE" | cut -d= -f2- | tr -d "'" | tr -d '"' || true)
    fi

    sep
    echo -e "  Key   : ${CYAN}${key}${NC}"
    echo    "  Info  : ${desc}"
    [[ -n "$current" ]] && echo -e "  Current: ${YELLOW}${current:0:40}${current:40:1:+…}${NC}"
    [[ -n "$gen_cmd" ]] && echo    "  (press Enter to auto-generate)"
    [[ -n "$default" && -z "$gen_cmd" ]] && echo    "  (press Enter to keep: ${default})"

    ask "New value (or Enter to skip/generate):"
    read -r input

    if [[ -z "$input" ]]; then
        if [[ -n "$gen_cmd" ]]; then
            input=$(eval "$gen_cmd")
            ok "Generated: ${input:0:48}…"
        elif [[ -n "$current" ]]; then
            input="$current"
            ok "Kept existing value."
        elif [[ -n "$default" ]]; then
            input="$default"
            ok "Using default."
        else
            warn "Skipped — value left empty."
        fi
    fi

    echo "$key='$input'" >> "$TMP_ENV"
}

# ---- init temp file ----
TMP_ENV=$(mktemp)
trap 'rm -f "$TMP_ENV"' EXIT

echo ""
echo -e "${CYAN}╔══════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║    rev0auth — Secret & Config Generator  ║${NC}"
echo -e "${CYAN}╚══════════════════════════════════════════╝${NC}"
echo ""
echo "  This script creates or updates your ${CYAN}.env${NC} file."
echo "  Leave a field blank to auto-generate (where applicable) or keep the current value."
echo ""

# ---- DATABASE ----
sep
echo -e "  ${CYAN}── Database ──${NC}"
prompt_value "POSTGRES_PASSWORD"   "Password for the PostgreSQL user"                       "" "gen_hex 24"
prompt_value "DATABASE_URL"        "Full Postgres connection URL"                            "" ""

# ---- JWT ----
sep
echo -e "  ${CYAN}── Auth ──${NC}"
prompt_value "AUTH_JWT_SECRET"     "JWT signing secret (min 32 bytes — generated = 64 hex)" "" "gen_hex 32"

# ---- Admin credentials ----
sep
echo -e "  ${CYAN}── Admin credentials ──${NC}"
prompt_value "ADMIN_DASH_PSEUDO"   "Admin username (not 'admin')"                           "admin"  ""
prompt_value "ADMIN_DASH_SEED"     "Admin seed phrase (not 'rev0auth-seed')"                ""       "gen_hex 16"
prompt_value "ADMIN_DASH_PASSWORD" "Admin login password (strong, min 20 chars)"            ""       ""

# ---- TOTP ----
sep
echo -e "  ${CYAN}── TOTP 2FA ──${NC}"
echo    "  Leave blank to skip 2FA, or press Enter to generate a new TOTP secret."
echo    "  After generation, scan the QR with your authenticator app (make admin-2fa-init)."
prompt_value "ADMIN_DASH_TOTP_SECRET" "Base32 TOTP secret (empty = 2FA disabled)"          ""       "gen_b32"

# ---- WebAuthn / YubiKey ----
sep
echo -e "  ${CYAN}── WebAuthn / YubiKey ──${NC}"
echo    "  STEP 1: Set your domain details below."
echo    "  STEP 2: Start the server, log in, go to Dashboard → Security."
echo    "  STEP 3: Run this script again with --yubikey to paste the credential JSON."
echo ""
prompt_value "WEBAUTHN_RP_ID"      "WebAuthn relying-party ID (your domain, e.g. auth.example.com)" "localhost" ""
prompt_value "WEBAUTHN_RP_ORIGIN"  "WebAuthn origin (https://auth.example.com or http://localhost:3000)" "http://localhost:3000" ""

# ---- YubiKey credential (optional second pass) ----
if [[ "${1:-}" == "--yubikey" ]]; then
    sep
    echo -e "  ${CYAN}── YubiKey enrollment ──${NC}"
    echo    "  Paste the JSON credential shown in the Dashboard → Security tab after key registration."
    echo    "  It starts with: {\"cred_id\":..."
    echo ""
    ask "ADMIN_WEBAUTHN_CREDENTIAL JSON (paste and press Enter, or Enter to skip):"
    read -r yubikey_json
    if [[ -n "$yubikey_json" ]]; then
        echo "ADMIN_WEBAUTHN_CREDENTIAL='${yubikey_json}'" >> "$TMP_ENV"
        ok "YubiKey credential saved."
    else
        warn "Skipped — YubiKey credential not set."
        # preserve existing value if any
        if [[ -f "$ENV_FILE" ]]; then
            existing_cred=$(grep -E "^ADMIN_WEBAUTHN_CREDENTIAL=" "$ENV_FILE" | head -1 || true)
            [[ -n "$existing_cred" ]] && echo "$existing_cred" >> "$TMP_ENV"
        fi
    fi
else
    # preserve existing credential
    if [[ -f "$ENV_FILE" ]]; then
        existing_cred=$(grep -E "^ADMIN_WEBAUTHN_CREDENTIAL=" "$ENV_FILE" | head -1 || true)
        if [[ -n "$existing_cred" ]]; then
            echo "$existing_cred" >> "$TMP_ENV"
            ok "Preserved existing YubiKey credential."
        else
            echo "ADMIN_WEBAUTHN_CREDENTIAL=''" >> "$TMP_ENV"
        fi
    else
        echo "ADMIN_WEBAUTHN_CREDENTIAL=''" >> "$TMP_ENV"
    fi
fi

# ---- bind addresses ----
sep
echo -e "  ${CYAN}── Bind addresses (optional) ──${NC}"
prompt_value "API_BIND_ADDR"       "API listen address (default: 0.0.0.0:8080)" "0.0.0.0:8080" ""
prompt_value "WEB_BIND_ADDR"       "Web listen address (default: 0.0.0.0:3000)" "0.0.0.0:3000" ""
prompt_value "REV0AUTH_API_UPSTREAM" "Web → API upstream (default: 127.0.0.1:8080)" "127.0.0.1:8080" ""

# ---- write .env ----
sep
echo ""

# Backup existing .env if present
if [[ -f "$ENV_FILE" ]]; then
    cp "$ENV_FILE" "${ENV_FILE}.bak"
    warn "Backed up existing .env → .env.bak"
fi

cp "$TMP_ENV" "$ENV_FILE"
chmod 600 "$ENV_FILE"

echo ""
ok "Written to ${ENV_FILE}"
ok "Permissions set to 600 (owner read/write only)"
echo ""
echo -e "  ${CYAN}Next steps:${NC}"
echo    "  1. make launch-all              — start API + Web + DB"
echo    "  2. Log in at /japprends/login   — verify admin credentials + TOTP"
echo    "  3. Go to Dashboard → Security   — confirm YubiKey status"
echo    "  4. (optional) bash scripts/gen_secret.sh --yubikey   — to persist your key credential"
echo ""
