#!/usr/bin/env bash
set -Eeuo pipefail
IFS=$'\n\t'

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

usage() {
  cat <<'EOF'
Usage:
  ./scripts/devtools.sh setup-vps [args...]
  ./scripts/devtools.sh caddy-install [--dry-run]
  ./scripts/devtools.sh launch-all          # docker compose up -d --build
  ./scripts/devtools.sh stop all            # docker compose down
  ./scripts/devtools.sh status
EOF
}

status() {
  cd "$ROOT_DIR"
  if docker compose ps --quiet 2>/dev/null | grep -q .; then
    echo "=== Docker Compose ==="
    docker compose ps --format "table {{.Name}}\t{{.Status}}\t{{.Ports}}"
  else
    echo "No compose services running. Frontend dev: cd frontend && npm run dev"
  fi
}

if [[ $# -lt 1 ]]; then
  usage
  exit 1
fi

cmd="$1"
shift || true

case "$cmd" in
  setup-vps)
    exec "$ROOT_DIR/scripts/setup-vps.sh" "$@"
    ;;
  caddy-install)
    exec "$ROOT_DIR/scripts/install-caddy-template.sh" "$@"
    ;;
  launch-all)
    # shellcheck disable=SC1090
    source "$ROOT_DIR/scripts/load-env.sh"
    exec "$ROOT_DIR/scripts/launch-all.sh"
    ;;
  stop)
    exec "$ROOT_DIR/scripts/stop.sh" "$@"
    ;;
  status)
    status
    ;;
  *)
    echo "Unknown command: $cmd" >&2
    usage
    exit 1
    ;;
esac
