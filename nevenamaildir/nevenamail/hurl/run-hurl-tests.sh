#!/usr/bin/env bash
set -euo pipefail

DIR="$(cd "$(dirname "$0")" && pwd)"
HOST="${HOST:-http://localhost:8000}"
UID_VAL="${UID_VAL:-$(date +%s)-$$}"

echo "Running Hurl tests against $HOST with uid=$UID_VAL"

FILES=()

if [ "$#" -gt 0 ]; then
  for f in "$@"; do
    [[ "$f" == *.hurl ]] && FILES+=("$f")
  done
else
  shopt -s nullglob
  FILES=("$DIR"/*.hurl)
fi

if [ ${#FILES[@]} -eq 0 ]; then
  echo "No .hurl files to run."
  exit 0
fi

echo "Test files:"
printf '  %s\n' "${FILES[@]}"

hurl --test \
  --jobs 1 \
  --variable "host=$HOST" \
  --variable "uid=$UID_VAL" \
  "${FILES[@]}"