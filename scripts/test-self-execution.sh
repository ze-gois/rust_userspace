#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

cargo build --bin userspace

TARGET_DIRECTORY="$(
    cargo metadata --format-version 1 --no-deps |
        python3 -c 'import json, sys; print(json.load(sys.stdin)["target_directory"])'
)"
BINARY="$TARGET_DIRECTORY/x86_64-unknown-none/debug/userspace"

if [[ ! -x "$BINARY" ]]; then
    echo "userspace binary not found or not executable: $BINARY" >&2
    exit 1
fi

ELF_TYPE="$(readelf -h "$BINARY" | awk '/Type:/{print $2; exit}')"
if [[ "$ELF_TYPE" != "DYN" ]]; then
    echo "expected static PIE ET_DYN, got ELF type: $ELF_TYPE" >&2
    exit 1
fi

if readelf -l "$BINARY" | grep -q 'INTERP'; then
    echo "static PIE unexpectedly contains PT_INTERP" >&2
    exit 1
fi

RELOCATION_TYPES="$(
    readelf -rW "$BINARY" |
        awk '/R_X86_64_/ { print $3 }' |
        sort -u
)"
if [[ -n "$RELOCATION_TYPES" ]]; then
    while IFS= read -r RELOCATION_TYPE; do
        if [[ "$RELOCATION_TYPE" != "R_X86_64_RELATIVE" ]]; then
            echo "unsupported static PIE relocation before execution: $RELOCATION_TYPE" >&2
            readelf -rW "$BINARY" >&2
            exit 1
        fi
    done <<< "$RELOCATION_TYPES"
fi

set +e
OUTPUT="$("$BINARY" 2>&1)"
STATUS=$?
set -e

printf '%s\n' "$OUTPUT"

if [[ "$STATUS" -ne 0 ]]; then
    echo "userspace self execution exited abnormally with status $STATUS" >&2
    exit "$STATUS"
fi

for EXECUTION in 1 2 3; do
    MARKER="userspace self execution ${EXECUTION}/3"
    COUNT="$(printf '%s\n' "$OUTPUT" | grep -Fxc "$MARKER" || true)"
    if [[ "$COUNT" -ne 1 ]]; then
        echo "expected exactly one: $MARKER; observed $COUNT" >&2
        exit 1
    fi
done

echo "static PIE self execution: 3/3"
