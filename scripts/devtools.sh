#!/usr/bin/env bash
set -Eeuo pipefail
IFS=$'\n\t'

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
RUN_DIR="$ROOT_DIR/.run"

usage() {
  cat <<'EOF'
Usage:
  ./scripts/devtools.sh setup-vps [args...]
  ./scripts/devtools.sh caddy-install [--dry-run]
  ./scripts/devtools.sh launch-all
  ./scripts/devtools.sh launch <api|web> [--bg]
  ./scripts/devtools.sh stop <api|web|all>
  ./scripts/devtools.sh status
  ./scripts/devtools.sh test
EOF
}

status() {
  for service in api web; do
    pid_file="$RUN_DIR/${service}.pid"
    if [[ -f "$pid_file" ]]; then
      pid="$(cat "$pid_file" 2>/dev/null || true)"
      if [[ -n "$pid" ]] && kill -0 "$pid" >/dev/null 2>&1; then
        echo "$service: running (pid=$pid)"
      else
        echo "$service: stopped (stale pid file)"
      fi
    else
      echo "$service: stopped"
    fi
  done
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
    exec "$ROOT_DIR/scripts/launch-all.sh"
    ;;
  launch)
    exec "$ROOT_DIR/scripts/launch.sh" "$@"
    ;;
  stop)
    exec "$ROOT_DIR/scripts/stop.sh" "$@"
    ;;
  status)
    status
    ;;
  test)
    cd "$ROOT_DIR"
    exec cargo test
    ;;
  *)
    echo "Unknown command: $cmd" >&2
    usage
    exit 1
    ;;
esac
