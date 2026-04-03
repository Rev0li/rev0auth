#!/usr/bin/env bash
set -Eeuo pipefail
IFS=$'\n\t'

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

"$ROOT_DIR/scripts/launch.sh" api --bg
"$ROOT_DIR/scripts/launch.sh" web --bg

echo "All services launched in background."
echo "Use ./scripts/devtools.sh status to inspect processes."
