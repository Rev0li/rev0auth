#!/usr/bin/env bash
set -Eeuo pipefail
IFS=$'\n\t'

# Installs project-linked dynamic Caddy template on VPS.
# Usage:
#   sudo ./scripts/install-caddy-template.sh [--dry-run]

DRY_RUN=0
if [[ "${1:-}" == "--dry-run" ]]; then
  DRY_RUN=1
fi

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TEMPLATE="$ROOT_DIR/infra/caddy/Caddyfile.template"
ENV_EXAMPLE="$ROOT_DIR/infra/caddy/caddy.env.example"
TARGET_CADDYFILE="/etc/caddy/Caddyfile"
TARGET_ENV="/etc/caddy/rev0auth-caddy.env"
SYSTEMD_OVERRIDE_DIR="/etc/systemd/system/caddy.service.d"
SYSTEMD_OVERRIDE_FILE="$SYSTEMD_OVERRIDE_DIR/rev0auth-env.conf"

log() {
  printf '[install-caddy-template] %s\n' "$*"
}

run() {
  if [[ "$DRY_RUN" -eq 1 ]]; then
    printf '[dry-run] %s\n' "$*"
    return 0
  fi
  eval "$@"
}

require_root() {
  if [[ "$EUID" -ne 0 ]]; then
    if command -v sudo >/dev/null 2>&1; then
      exec sudo -E bash "$0" "$@"
    fi
    echo "This script must run as root." >&2
    exit 1
  fi
}

main() {
  require_root "$@"

  if [[ ! -f "$TEMPLATE" ]]; then
    echo "Template not found: $TEMPLATE" >&2
    exit 1
  fi

  log "Ensuring Caddy package is installed..."
  run "apt-get update"
  run "apt-get install -y caddy"

  log "Installing Caddyfile from project template..."
  run "install -d -m 755 /etc/caddy"
  run "cp '$TEMPLATE' '$TARGET_CADDYFILE'"
  run "chown root:root '$TARGET_CADDYFILE'"
  run "chmod 644 '$TARGET_CADDYFILE'"

  if [[ ! -f "$TARGET_ENV" ]]; then
    log "Creating env file from example (edit before restart if needed)..."
    run "cp '$ENV_EXAMPLE' '$TARGET_ENV'"
    run "chown root:root '$TARGET_ENV'"
    run "chmod 600 '$TARGET_ENV'"
  else
    log "Env file already exists: $TARGET_ENV"
  fi

  log "Configuring systemd override to load environment variables..."
  run "install -d -m 755 '$SYSTEMD_OVERRIDE_DIR'"
  run "cat > '$SYSTEMD_OVERRIDE_FILE' <<'EOF'
[Service]
EnvironmentFile=/etc/caddy/rev0auth-caddy.env
EOF"

  log "Reloading systemd and validating Caddy config..."
  run "systemctl daemon-reload"
  run "caddy validate --config '$TARGET_CADDYFILE' --adapter caddyfile"

  log "Restarting Caddy..."
  run "systemctl enable caddy"
  run "systemctl restart caddy"

  log "Done. Verify with: curl -fI https://\$(grep ^WEB_DOMAIN= '$TARGET_ENV' | cut -d= -f2)/japprends/login"
}

main "$@"
