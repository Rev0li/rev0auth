#!/usr/bin/env bash
set -Eeuo pipefail
IFS=$'\n\t'

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "Building and starting all services via Docker Compose..."
docker compose up -d --build

echo ""
echo "Services launched. Use 'docker compose ps' or 'make status' to inspect."
echo "Logs: docker compose logs -f [api|web|postgres]"
