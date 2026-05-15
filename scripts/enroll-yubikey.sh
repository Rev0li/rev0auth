#!/usr/bin/env bash
# enroll-yubikey.sh — persist YubiKey WebAuthn credential to .env
#
# Workflow:
#   1. make launch-all
#   2. Open http://localhost:3000/japprends/login  → log in as admin
#   3. Dashboard → Security tab → click "Register YubiKey" → touch the key
#   4. Copy the JSON blob shown on screen
#   5. Run this script and paste it
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ENV_FILE="$ROOT/.env"

GREEN='\033[0;32m'; YELLOW='\033[1;33m'; CYAN='\033[0;36m'; RED='\033[0;31m'; NC='\033[0m'
ok()   { echo -e "${GREEN}✓${NC} $*"; }
warn() { echo -e "${YELLOW}⚠${NC}  $*"; }
err()  { echo -e "${RED}✗${NC} $*" >&2; exit 1; }

[[ -f "$ENV_FILE" ]] || err ".env not found at $ENV_FILE — run ./scripts/gen_secret.sh first."

# Show current state
current=$(grep -E '^ADMIN_WEBAUTHN_CREDENTIAL=' "$ENV_FILE" | head -1 | cut -d= -f2- | tr -d "'" | tr -d '"' || true)
echo ""
echo -e "${CYAN}╔══════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║     rev0auth — YubiKey Enrollment        ║${NC}"
echo -e "${CYAN}╚══════════════════════════════════════════╝${NC}"
echo ""

if [[ -n "$current" ]]; then
    warn "A credential is already registered:"
    echo -e "  ${YELLOW}${current:0:80}…${NC}"
    echo ""
    echo "Enrolling a new key will REPLACE the existing one."
    echo -e "  ${CYAN}?${NC}  Continue? [y/N]"
    read -r confirm
    [[ "$confirm" =~ ^[Yy]$ ]] || { echo "Aborted."; exit 0; }
    echo ""
fi

echo "  Prerequisite checklist:"
echo "  ✦ Server is running  (make launch-all → make status)"
echo "  ✦ You are logged in as admin in the browser"
echo "  ✦ You went to Dashboard → Security tab and registered your key"
echo ""
echo -e "  ${CYAN}?${NC}  Paste the credential JSON from the Security tab and press Enter:"
echo    "     (It starts with {\"cred_id\":...)"
echo ""
read -r json_input

if [[ -z "$json_input" ]]; then
    warn "Nothing pasted — aborted. No changes made."
    exit 0
fi

# Basic sanity check — must look like JSON
if [[ "$json_input" != \{* ]]; then
    err "Input does not look like JSON (must start with '{'}). Aborted."
fi

# Backup and update
cp "$ENV_FILE" "${ENV_FILE}.bak"
warn "Backed up .env → .env.bak"

# Replace or append the credential line
if grep -qE '^ADMIN_WEBAUTHN_CREDENTIAL=' "$ENV_FILE"; then
    # Replace in-place using Python (handles embedded quotes safely)
    python3 - "$ENV_FILE" "$json_input" <<'PY'
import sys
path, cred = sys.argv[1], sys.argv[2]
lines = open(path).readlines()
out = []
for line in lines:
    if line.startswith('ADMIN_WEBAUTHN_CREDENTIAL='):
        out.append(f"ADMIN_WEBAUTHN_CREDENTIAL='{cred}'\n")
    else:
        out.append(line)
open(path, 'w').writelines(out)
PY
else
    echo "ADMIN_WEBAUTHN_CREDENTIAL='${json_input}'" >> "$ENV_FILE"
fi

chmod 600 "$ENV_FILE"
echo ""
ok "Credential saved to .env."
echo ""
echo -e "  ${CYAN}Next steps:${NC}"
echo    "  make stop-all && make launch-all   — restart to load the new credential"
echo    "  Then log in again — the Security tab should show your key as registered."
echo ""
