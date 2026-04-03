#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "[preflight] branch: $(git branch --show-current)"

if [[ -z "${ADMIN_DASH_PASSWORD:-}" ]]; then
  echo "[preflight][error] ADMIN_DASH_PASSWORD is not set"
  echo "export ADMIN_DASH_PASSWORD='change-me'"
  exit 1
fi

echo "[preflight] ADMIN_DASH_PASSWORD: ok"

echo "[preflight] cargo check api"
~/.cargo/bin/cargo check -p rev0auth-api >/dev/null

echo "[preflight] cargo check web"
~/.cargo/bin/cargo check -p rev0auth-web >/dev/null

echo "[preflight] optional test api"
~/.cargo/bin/cargo test -p rev0auth-api >/dev/null

echo "[preflight] git status summary"
git status --short | head -n 40

echo "[preflight] ok"
