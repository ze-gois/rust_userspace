#!/usr/bin/env bash
set -euo pipefail

cargo test   -Z build-std   --target x86_64-unknown-linux-gnu   --no-default-features   --features host_tests,with_std   --test elf_representation   --test elf_dynamic_initialization   --test elf_dynamic_flags   --test elf_relative_relocation   --test elf_dynamic_symbol_table_size   --test elf_dynamic_relative_relocation   --test elf_section_relative_relocation   --test elf_symbol_visibility   --test elf_section_link_order   --test elf_compression_43
