#!/usr/bin/env bash
set -Eeuo pipefail
IFS=$'\n\t'

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

usage() {
  echo "Usage: ./scripts/stop.sh all"
}

case "${1:-}" in
  all)
    cd "$ROOT_DIR"
    echo "Stopping Docker Compose stack..."
    docker compose down
    ;;
  *)
    usage
    exit 1
    ;;
esac
