#!/usr/bin/env bash
# deploy-preflight.sh — pre-deploy sanity checks (Docker-based)
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

GREEN='\033[0;32m'; YELLOW='\033[1;33m'; RED='\033[0;31m'; CYAN='\033[0;36m'; NC='\033[0m'
ok()   { echo -e "${GREEN}[ok]${NC}    $*"; }
warn() { echo -e "${YELLOW}[warn]${NC}  $*"; }
fail() { echo -e "${RED}[fail]${NC}  $*"; FAILED=1; }

FAILED=0

echo ""
echo -e "${CYAN}╔══════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║     rev0auth — Deploy Preflight          ║${NC}"
echo -e "${CYAN}╚══════════════════════════════════════════╝${NC}"
echo ""

# ── Git ───────────────────────────────────────────────────────────────────────
echo -e "${CYAN}── Git ──${NC}"
echo "  Branch : $(git branch --show-current)"
echo "  Commit : $(git log -1 --format='%h %s')"
dirty=$(git status --short)
if [[ -n "$dirty" ]]; then
    warn "Uncommitted changes:"
    git status --short | sed 's/^/    /'
else
    ok "Working tree clean"
fi

# ── Docker ────────────────────────────────────────────────────────────────────
echo ""
echo -e "${CYAN}── Docker ──${NC}"
if ! command -v docker >/dev/null 2>&1; then
    fail "docker not found — install Docker Engine"
else
    ok "docker $(docker --version | grep -oP '[\d.]+'| head -1)"
fi

if ! docker compose version >/dev/null 2>&1; then
    fail "docker compose plugin not found"
else
    ok "docker compose $(docker compose version --short 2>/dev/null || echo 'ok')"
fi

# ── .env checks ───────────────────────────────────────────────────────────────
echo ""
echo -e "${CYAN}── Environment ──${NC}"

if [[ ! -f "$ROOT_DIR/.env" ]]; then
    fail ".env missing — run: ./scripts/gen_secret.sh"
else
    ok ".env found"
    # shellcheck disable=SC1090
    set -a; source "$ROOT_DIR/.env"; set +a

    check_var() {
        local var="$1" desc="$2"
        local val="${!var:-}"
        if [[ -z "$val" ]]; then
            fail "${var} is empty — ${desc}"
        else
            ok "${var} set"
        fi
    }

    check_var "POSTGRES_PASSWORD"   "required for Docker postgres container"
    check_var "AUTH_JWT_SECRET"     "required for JWT signing + SongSurf validation"
    check_var "ADMIN_DASH_PASSWORD" "required for admin login"
    check_var "ADMIN_DASH_PSEUDO"   "required for admin login"
    check_var "ADMIN_DASH_SEED"     "required for session signing"

    # Warn (not fail) for optional but important vars
    [[ -z "${ADMIN_DASH_TOTP_SECRET:-}" ]] && warn "ADMIN_DASH_TOTP_SECRET empty — 2FA disabled"
    [[ -z "${COOKIE_DOMAIN:-}" ]]          && warn "COOKIE_DOMAIN empty — SongSurf cross-domain cookie disabled (OK for local dev)"
fi

# ── Build files ───────────────────────────────────────────────────────────────
echo ""
echo -e "${CYAN}── Build files ──${NC}"
for f in Dockerfile.frontend docker-compose.yml .dockerignore; do
    if [[ -f "$ROOT_DIR/$f" ]]; then
        ok "$f present"
    else
        fail "$f missing"
    fi
done

# ── Docker build dry-run ──────────────────────────────────────────────────────
echo ""
echo -e "${CYAN}── Docker build check ──${NC}"
echo "  Running 'docker compose build' (this may take a while on first run)..."
if docker compose build --quiet 2>&1; then
    ok "docker compose build succeeded"
else
    fail "docker compose build failed — check Dockerfile.frontend"
fi

# ── Summary ───────────────────────────────────────────────────────────────────
echo ""
if [[ "$FAILED" -eq 0 ]]; then
    echo -e "${GREEN}All checks passed. Ready to deploy.${NC}"
    echo ""
    echo "  make launch-all          — start all services"
    echo "  make status              — verify containers are up"
    echo "  docker compose logs -f   — follow logs"
else
    echo -e "${RED}Preflight failed — fix the issues above before deploying.${NC}"
    exit 1
fi
echo ""
