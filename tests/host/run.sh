#!/bin/sh
set -eu

ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/../.." && pwd)
OUT="$ROOT/target/host-probe"

mkdir -p "$OUT"

rustc \
    --crate-name ample \
    --crate-type=rlib \
    --edition=2024 \
    "$ROOT/crates/ample/src/library.rs" \
    -o "$OUT/libample.rlib"

rustc \
    --crate-name userspace \
    --crate-type=rlib \
    --edition=2024 \
    --cfg 'feature="with_std"' \
    "$ROOT/src/library.rs" \
    --extern ample="$OUT/libample.rlib" \
    -L dependency="$OUT" \
    -o "$OUT/libuserspace.rlib"

CARGO_MANIFEST_DIR="$ROOT" rustc \
    --test \
    --edition=2024 \
    "$ROOT/tests/host/elf_layout.rs" \
    --cfg 'feature="host_tests"' \
    --extern userspace="$OUT/libuserspace.rlib" \
    --extern ample="$OUT/libample.rlib" \
    -L dependency="$OUT" \
    -o "$OUT/elf_layout_tests"

"$OUT/elf_layout_tests" --nocapture
