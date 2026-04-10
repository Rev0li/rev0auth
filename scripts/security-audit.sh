#!/usr/bin/env bash
set -Eeuo pipefail
IFS=$'\n\t'

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

echo "[audit] security checks: start"

MIGRATION_FILE="$ROOT_DIR/crates/api/migrations/0001_auth_schema.sql"
if ! rg -n "password_hash\s+TEXT\s+NOT\s+NULL" "$MIGRATION_FILE" >/dev/null; then
  echo "[audit][error] password_hash column missing in auth schema"
  exit 1
fi

if rg -n "\bpassword\b\s+TEXT" "$MIGRATION_FILE" >/dev/null; then
  echo "[audit][error] clear-text password column detected in auth schema"
  exit 1
fi

if ! rg -n "Argon2|hash_password\(" "$ROOT_DIR/crates/api/src/auth/password.rs" >/dev/null; then
  echo "[audit][error] Argon2 password hashing not detected"
  exit 1
fi

if ! rg -n "ADMIN_DASH_TOTP_SECRET" "$ROOT_DIR/crates/web/src/main.rs" >/dev/null; then
  echo "[audit][error] admin TOTP 2FA hook missing"
  exit 1
fi

echo "[audit] security checks: ok"
