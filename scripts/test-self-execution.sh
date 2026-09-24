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

run_and_capture() {
    local argument="${1-}"
    local output
    local status

    set +e
    if [[ -z "$argument" ]]; then
        output="$("$BINARY" 2>&1)"
    else
        output="$("$BINARY" "$argument" 2>&1)"
    fi
    status=$?
    set -e

    printf '%s\n' "$output"

    if [[ "$status" -ne 0 ]]; then
        echo "userspace exited abnormally with status $status" >&2
        exit "$status"
    fi

    CAPTURED_OUTPUT="$output"
}

run_and_capture
if [[ "$CAPTURED_OUTPUT" != "userspace" ]]; then
    echo "no argument must execute once without self-loading" >&2
    exit 1
fi

run_and_capture "not-a-natural-number"
if [[ "$CAPTURED_OUTPUT" != "userspace" ]]; then
    echo "non-natural argument must execute once without self-loading" >&2
    exit 1
fi

run_and_capture "3"

EXPECTED="$(printf 'userspace 3\nuserspace 2\nuserspace 1\nuserspace 0')"
if [[ "$CAPTURED_OUTPUT" != "$EXPECTED" ]]; then
    echo "natural-number argument did not decrement through self execution" >&2
    exit 1
fi

echo "static PIE natural-number self execution: 3 -> 2 -> 1 -> 0"
