#!/usr/bin/env bash
set -Eeuo pipefail
IFS=$'\n\t'

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
RUN_DIR="$ROOT_DIR/.run"

usage() {
  echo "Usage: ./scripts/stop.sh <api|web|all>"
}

if [[ $# -lt 1 ]]; then
  usage
  exit 1
fi

stop_one() {
  local service="$1"
  local pid_file="$RUN_DIR/${service}.pid"

  if [[ ! -f "$pid_file" ]]; then
    echo "$service not running (no pid file)"
    return
  fi

  local pid
  pid="$(cat "$pid_file" 2>/dev/null || true)"

  if [[ -z "$pid" ]]; then
    rm -f "$pid_file"
    echo "$service not running (empty pid file)"
    return
  fi

  if kill -0 "$pid" >/dev/null 2>&1; then
    kill "$pid"
    echo "Stopped $service (pid=$pid)"
  else
    echo "$service not running (stale pid=$pid)"
  fi

  rm -f "$pid_file"
}

case "$1" in
  api|web)
    stop_one "$1"
    ;;
  all)
    stop_one api
    stop_one web
    ;;
  *)
    echo "Unknown service: $1" >&2
    usage
    exit 1
    ;;
esac
