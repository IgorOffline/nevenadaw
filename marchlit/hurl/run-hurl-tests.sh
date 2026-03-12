#!/usr/bin/env bash
set -euo pipefail

HURL_HOME="${HURL_HOME:-$HOME/Documents/util/hurl}"
DIR="$(cd "$(dirname "$0")" && pwd)"
HOST="${HOST:-http://localhost:8000}"
UID_VAL="${UID_VAL:-$(date +%s)-$$}"

echo "Running Hurl tests against $HOST with uid=$UID_VAL"

FILES=("$@")
if [ ${#FILES[@]} -eq 0 ]; then
  shopt -s nullglob
  FILES=("$DIR"/*.hurl)
fi

echo "Test files:"
printf '  %s\n' "${FILES[@]}"

"$HURL_HOME/hurl" --test \
  --jobs 1 \
  --variable "host=$HOST" \
  --variable "uid=$UID_VAL" \
  "${FILES[@]}"
