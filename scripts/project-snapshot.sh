#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
STAMP="$(date +%Y%m%d-%H%M%S)"
OUT_DIR="$ROOT_DIR/backups"
OUT_FILE="$OUT_DIR/rev0auth-snapshot-$STAMP.tar.gz"

mkdir -p "$OUT_DIR"

tar \
  --exclude='.git' \
  --exclude='target' \
  --exclude='backups' \
  -czf "$OUT_FILE" \
  -C "$ROOT_DIR" \
  Cargo.toml Cargo.lock Makefile .env.example \
  crates docs scripts infra docker-compose.yml

echo "Snapshot created: $OUT_FILE"
