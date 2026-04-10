#!/usr/bin/env bash
set -Eeuo pipefail
IFS=$'\n\t'

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# shellcheck disable=SC1090
source "$ROOT_DIR/scripts/load-env.sh"

if [[ -z "${ADMIN_DASH_TOTP_SECRET:-}" ]]; then
  echo "admin OTP unavailable: ADMIN_DASH_TOTP_SECRET is not set" >&2
  exit 1
fi

python3 - <<'PY'
import base64
import hashlib
import hmac
import os
import struct
import time

secret = os.environ["ADMIN_DASH_TOTP_SECRET"].strip().replace(" ", "").replace("-", "").upper()
padding = "=" * (-len(secret) % 8)
key = base64.b32decode(secret + padding, casefold=True)
counter = int(time.time()) // 30
msg = struct.pack(">Q", counter)
digest = hmac.new(key, msg, hashlib.sha1).digest()
offset = digest[-1] & 0x0F
code = (struct.unpack(">I", digest[offset:offset + 4])[0] & 0x7fffffff) % 1_000_000
print(f"{code:06d}")
PY
