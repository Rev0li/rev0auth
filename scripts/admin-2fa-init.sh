#!/usr/bin/env bash
set -Eeuo pipefail
IFS=$'\n\t'

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ENV_FILE="$ROOT_DIR/.env"

# shellcheck disable=SC1090
source "$ROOT_DIR/scripts/load-env.sh"

python3 - <<'PY' "$ENV_FILE"
import base64
import hashlib
import hmac
import os
import secrets
import struct
import sys
import time

env_path = sys.argv[1]

def make_secret() -> str:
    return base64.b32encode(secrets.token_bytes(20)).decode("ascii").rstrip("=")

def write_secret(path: str, secret: str) -> None:
    lines = []
    found = False
    if os.path.exists(path):
        with open(path, "r", encoding="utf-8") as handle:
            for line in handle:
                stripped = line.lstrip()
                if stripped.startswith("ADMIN_DASH_TOTP_SECRET=") or stripped.startswith("export ADMIN_DASH_TOTP_SECRET="):
                    if not found:
                        lines.append(f"ADMIN_DASH_TOTP_SECRET='{secret}'\n")
                        found = True
                    continue
                lines.append(line)
    if not found:
        if lines and not lines[-1].endswith("\n"):
            lines[-1] = lines[-1] + "\n"
        lines.append(f"ADMIN_DASH_TOTP_SECRET='{secret}'\n")
    with open(path, "w", encoding="utf-8") as handle:
        handle.writelines(lines)

def current_otp(secret_b32: str) -> str:
    normalized = secret_b32.strip().replace(" ", "").replace("-", "").upper()
    padding = "=" * (-len(normalized) % 8)
    key = base64.b32decode(normalized + padding, casefold=True)
    counter = int(time.time()) // 30
    msg = struct.pack(">Q", counter)
    digest = hmac.new(key, msg, hashlib.sha1).digest()
    offset = digest[-1] & 0x0F
    code = (struct.unpack(">I", digest[offset:offset + 4])[0] & 0x7fffffff) % 1_000_000
    return f"{code:06d}"

secret = os.environ.get("ADMIN_DASH_TOTP_SECRET", "").strip()
if not secret:
    secret = make_secret()

write_secret(env_path, secret)
print(secret)
print(current_otp(secret))
PY
