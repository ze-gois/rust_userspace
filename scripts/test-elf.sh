#!/usr/bin/env bash
set -euo pipefail

cargo check \
  -Z build-std \
  --target x86_64-unknown-linux-gnu \
  --no-default-features \
  --features host_tests,with_std \
  --example elf_inspection

cargo test   -Z build-std   --target x86_64-unknown-linux-gnu   --no-default-features   --features host_tests,with_std   --test elf_representation   --test elf_dynamic_initialization   --test elf_dynamic_flags   --test elf_relative_relocation   --test elf_relocation_validation   --test elf_dynamic_symbol_table_size   --test elf_dynamic_relative_relocation   --test elf_section_relative_relocation   --test elf_symbol_visibility   --test elf_symbol_table_validation   --test elf_symbol_table_section_indices   --test elf_symbol_sections   --test elf_string_table_validation   --test elf_section_link_order   --test elf_compression   --test elf_notes   --test elf_section_links   --test elf_section_group_members   --test elf_section_group_validation   --test elf_section_header_validation   --test elf_section_table_consistency   --test elf_initial_section_header_validation   --test elf_header_validation
