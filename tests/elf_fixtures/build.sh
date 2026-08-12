#!/bin/sh
set -eu

ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/../.." && pwd)
OUT="$ROOT/target/elf-fixtures"
CC=${CC:-cc}
AS=${AS:-as}
LD=${LD:-ld}

mkdir -p "$OUT"

"$AS" --64 "$ROOT/tests/elf_fixtures/start.s" -o "$OUT/start.o"
"$AS" --64 "$ROOT/tests/elf_fixtures/bss_large.s" -o "$OUT/bss_large.o"

# Conventional non-PIE ET_EXEC, normally near 0x400000.
"$LD" -static -e _start -Ttext=0x400000 \
    -o "$OUT/static_exec" "$OUT/start.o"

# Minimal ET_DYN image without PT_INTERP; useful for load-bias tests.
"$LD" -shared -e _start -o "$OUT/pie_minimal" "$OUT/start.o"

# ET_EXEC deliberately colliding with the userspace link range.
"$LD" -static -e _start -Ttext=0x100000 \
    -o "$OUT/collision_100000" "$OUT/start.o"

# Large BSS tail for p_filesz/p_memsz and zero-fill tests.
"$LD" -static -e _start -Ttext=0x400000 \
    -o "$OUT/bss_large" "$OUT/bss_large.o"

# Dynamically linked PIE with PT_INTERP/PT_DYNAMIC.
"$CC" -fPIE -pie \
    -o "$OUT/dynamic_pie" "$ROOT/tests/elf_fixtures/dynamic.c"

printf '%s\n' "ELF fixtures written to $OUT"
for fixture in static_exec pie_minimal dynamic_pie bss_large collision_100000; do
    printf '%-18s' "$fixture"
    readelf -hW "$OUT/$fixture" | awk '/Type:|Entry point address:/ { printf "%s ", $0 } END { print "" }'
done
