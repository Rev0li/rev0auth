#!/usr/bin/env bash
# enroll-yubikey.sh — automatic YubiKey WebAuthn enrollment
#
# Flow:
#   1. Detects your YubiKey (waits until plugged in)
#   2. Checks the rev0auth web server is running on :3000
#   3. Opens the browser at the registration page
#   4. You register the key there (touch it when prompted)
#   5. Script auto-logs in via curl + fetches the credential JSON
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
DASHBOARD_URL="$WEB_URL/japprends/login"
EXPORT_URL="$WEB_URL/japprends/webauthn/credential/export"
LOGIN_URL="$WEB_URL/japprends/login"

# ── Preflight ────────────────────────────────────────────────────────────────

[[ -f "$ENV_FILE" ]] || err ".env not found — run ./scripts/gen_secret.sh first."
command -v curl >/dev/null 2>&1 || err "curl is required but not found."
command -v python3 >/dev/null 2>&1 || err "python3 is required but not found."

# Load env
set -a; source "$ENV_FILE"; set +a

[[ -n "${ADMIN_DASH_PSEUDO:-}"   ]] || err "ADMIN_DASH_PSEUDO not set in .env"
[[ -n "${ADMIN_DASH_PASSWORD:-}" ]] || err "ADMIN_DASH_PASSWORD not set in .env"

echo ""
echo -e "${CYAN}${BOLD}╔══════════════════════════════════════════╗${NC}"
echo -e "${CYAN}${BOLD}║     rev0auth — YubiKey Enrollment        ║${NC}"
echo -e "${CYAN}${BOLD}╚══════════════════════════════════════════╝${NC}"

# Show current state
current=$(grep -E '^ADMIN_WEBAUTHN_CREDENTIAL=' "$ENV_FILE" | head -1 \
          | cut -d= -f2- | tr -d "'" | tr -d '"' || true)
if [[ -n "$current" ]]; then
    warn "A key is already registered. Proceeding will REPLACE it."
fi

# ── Step 1: Detect YubiKey ────────────────────────────────────────────────────
step "1/4" "YubiKey detection"

detect_key() {
    if command -v ykman >/dev/null 2>&1; then
        ykman list 2>/dev/null | grep -qi "yubikey" && return 0 || return 1
    fi
    lsusb 2>/dev/null | grep -qi "yubico\|yubikey" && return 0 || return 1
}

if detect_key; then
    ok "YubiKey already connected."
else
    info "Please plug in your YubiKey now…"
    while ! detect_key; do
        printf "."
        sleep 1
    done
    echo ""
    ok "YubiKey detected!"
fi

# Show key info if ykman available
if command -v ykman >/dev/null 2>&1; then
    key_info=$(ykman list 2>/dev/null | grep -i yubikey | head -1 || true)
    [[ -n "$key_info" ]] && info "Key: $key_info"
fi

# ── Step 2: Check server ──────────────────────────────────────────────────────
step "2/4" "Server check"

if curl -sf "$WEB_URL/health" >/dev/null 2>&1 || \
   curl -sf "$WEB_URL/"       -o /dev/null 2>&1; then
    ok "Server running at $WEB_URL"
else
    err "Server not reachable at $WEB_URL — run: make launch-all"
fi

# ── Step 3: Open browser for registration ────────────────────────────────────
step "3/4" "Browser registration"

info "Opening browser → $DASHBOARD_URL"
if command -v xdg-open >/dev/null 2>&1; then
    xdg-open "$DASHBOARD_URL" 2>/dev/null &
elif command -v open >/dev/null 2>&1; then
    open "$DASHBOARD_URL" 2>/dev/null &
else
    warn "Could not open browser automatically."
fi

echo ""
echo -e "  In the browser:"
echo    "  1. Log in as admin"
echo    "  2. Go to Dashboard → Security tab"
echo    "  3. Click 'Register YubiKey'"
echo    "  4. Touch your key when the browser asks"
echo ""
echo -e "  ${CYAN}?${NC}  Press Enter once the key is registered in the browser…"
read -r _

# ── Step 4: Auto-fetch credential via curl ────────────────────────────────────
step "4/4" "Fetching credential from server"

# Generate OTP if TOTP is configured
OTP=""
TOTP_SECRET="${ADMIN_DASH_TOTP_SECRET:-}"
TOTP_SECRET_CLEAN=$(echo "$TOTP_SECRET" | tr -d '"' | tr -d "'")
if [[ -n "$TOTP_SECRET_CLEAN" && "$TOTP_SECRET_CLEAN" != '""' ]]; then
    info "Generating TOTP code…"
    OTP=$(python3 - "$TOTP_SECRET_CLEAN" <<'PY'
import sys, base64, hashlib, hmac, struct, time
secret = sys.argv[1].strip().replace(' ','').replace('-','').upper()
padding = "=" * (-len(secret) % 8)
key = base64.b32decode(secret + padding, casefold=True)
counter = int(time.time()) // 30
msg = struct.pack(">Q", counter)
digest = hmac.new(key, msg, hashlib.sha1).digest()
offset = digest[-1] & 0x0F
code = (struct.unpack(">I", digest[offset:offset+4])[0] & 0x7fffffff) % 1_000_000
print(f"{code:06d}")
PY
)
    ok "OTP generated."
fi

# Login via curl, capture session cookie
COOKIE_JAR=$(mktemp)
trap 'rm -f "$COOKIE_JAR"' EXIT

info "Logging in as '${ADMIN_DASH_PSEUDO}'…"
LOGIN_BODY=$(python3 -c "
import json, sys
d = {'pseudo': sys.argv[1], 'password': sys.argv[2]}
if sys.argv[3]: d['otp'] = sys.argv[3]
print(json.dumps(d))
" "$ADMIN_DASH_PSEUDO" "$ADMIN_DASH_PASSWORD" "$OTP")

LOGIN_RESP=$(curl -sf -X POST "$LOGIN_URL" \
    -H "Content-Type: application/json" \
    -d "$LOGIN_BODY" \
    -c "$COOKIE_JAR" \
    --output - 2>&1) || true

if ! echo "$LOGIN_RESP" | python3 -c "import sys,json; d=json.load(sys.stdin); sys.exit(0 if d.get('ok') else 1)" 2>/dev/null; then
    echo "$LOGIN_RESP"
    err "Admin login failed. Check ADMIN_DASH_PASSWORD / TOTP in .env."
fi
ok "Logged in."

# Fetch credential
info "Fetching credential from server…"
CRED_RESP=$(curl -sf "$EXPORT_URL" \
    -b "$COOKIE_JAR" \
    --output - 2>&1) || true

CREDENTIAL_JSON=$(echo "$CRED_RESP" | python3 -c "
import sys, json
d = json.load(sys.stdin)
if not d.get('ok'):
    print('ERROR: ' + d.get('message', 'unknown'), file=sys.stderr)
    sys.exit(1)
print(d['credential_json'])
" 2>/dev/null) || {
    warn "Could not auto-fetch credential from server."
    warn "The key may not have been registered yet — try again after completing the browser step."
    echo ""
    echo "If the problem persists, paste the JSON manually:"
    echo -e "  ${CYAN}?${NC}  Paste credential JSON (from Dashboard → Security) and press Enter:"
    read -r CREDENTIAL_JSON
    [[ -n "$CREDENTIAL_JSON" ]] || err "Nothing entered. Aborted."
}
ok "Credential received."

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
ok "Credential saved to .env  (.env.bak created)"
echo ""
echo -e "  ${CYAN}Next:${NC}"
echo    "  make stop-all && make launch-all   — restart to activate the key"
echo    "  Then log in — you will be asked to touch your YubiKey."
echo ""
