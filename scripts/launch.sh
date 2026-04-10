#!/usr/bin/env bash
set -Eeuo pipefail
IFS=$'\n\t'

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
RUN_DIR="$ROOT_DIR/.run"
LOG_DIR="$RUN_DIR/logs"

# shellcheck disable=SC1090
source "$ROOT_DIR/scripts/load-env.sh"

usage() {
  echo "Usage: ./scripts/launch.sh <api|web> [--bg]"
}

if [[ $# -lt 1 ]]; then
  usage
  exit 1
fi

SERVICE="$1"
shift || true

BACKGROUND=0
if [[ "${1:-}" == "--bg" ]]; then
  BACKGROUND=1
fi

case "$SERVICE" in
  api)
    PACKAGE="rev0auth-api"
    ;;
  web)
    PACKAGE="rev0auth-web"
    ;;
  *)
    echo "Unknown service: $SERVICE" >&2
    usage
    exit 1
    ;;
esac

mkdir -p "$LOG_DIR"
PID_FILE="$RUN_DIR/${SERVICE}.pid"
LOG_FILE="$LOG_DIR/${SERVICE}.log"

is_running() {
  local pid="$1"
  kill -0 "$pid" >/dev/null 2>&1
}

if [[ -f "$PID_FILE" ]]; then
  existing_pid="$(cat "$PID_FILE" 2>/dev/null || true)"
  if [[ -n "$existing_pid" ]] && is_running "$existing_pid"; then
    echo "$SERVICE already running (pid=$existing_pid)"
    exit 0
  fi
  rm -f "$PID_FILE"
fi

cd "$ROOT_DIR"

if [[ "$BACKGROUND" -eq 1 ]]; then
  echo "Starting $SERVICE in background..."
  nohup cargo run -p "$PACKAGE" >"$LOG_FILE" 2>&1 &
  pid="$!"
  echo "$pid" >"$PID_FILE"
  echo "$SERVICE started (pid=$pid)"
  echo "logs: $LOG_FILE"
else
  echo "Starting $SERVICE in foreground..."
  exec cargo run -p "$PACKAGE"
fi
