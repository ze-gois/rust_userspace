# Project Revision

This branch is a deliberate semantic reset of `rust_userspace`.

The revision favors a smaller, defensible architecture over preserving legacy
functionality. Temporary regressions are acceptable when the existing behavior
depends on unclear ownership, misplaced concepts, duplicated sources of truth,
or abstractions whose names no longer match their meaning.

## Architectural laws

- `ELF` is a file format and belongs under `file::format::elf`.
- The existing ELF implementation may be deleted entirely and rebuilt from the
  GABI rather than repaired around legacy assumptions.
- Module names do not encode hierarchy with underscores. Hierarchy belongs in
  modules; modules are nouns before qualifications. For example,
  `stack::initial`, not `initial_stack`.
- ABI names and external constants may retain their normative spelling.
- `memory::stack` is preserved as an educational, operating-system-neutral
  model of stack memory.
- Linux process startup semantics do not define generic memory. Linux-specific
  `argc`, `argv`, `envp`, auxiliary vectors, and startup-stack construction
  belong under the Linux target/process domain.
- `memory::heap` and `memory::stack` describe memory concepts, not file
  formats or operating-system conventions.
- `Allocating` is retained as a valuable abstraction, but allocation layout
  must be independent from serialized `Bytes::BYTES_SIZE`.
- Serialization layout, in-memory Rust layout, residency, and ownership are
  distinct concepts.
- A module that survives this revision must be able to justify its namespace and
  semantics without relying on accidental legacy behavior.

## Intended direction

The project should converge toward a structure where the major boundaries are:

```text
userspace
├── memory
│   ├── page
│   ├── heap
│   └── stack
├── file
│   └── format
│       └── elf
└── target
    ├── arch
    └── os
        └── linux
            ├── syscall
            └── process
                └── stack
                    └── initial
```

This tree is a direction, not a compatibility promise. Names and intermediate
modules may change as the semantics become clearer.

## Revision order

1. Trim dead, misleading, and semantically misplaced code.
2. Rebuild memory allocation boundaries, preserving and correcting
   `Allocating`.
3. Re-establish `memory::stack` as a generic educational memory abstraction.
4. Separate Linux process-startup ABI from generic memory.
5. Rebuild `file::format::elf` from the GABI and ample's representation
   primitives.
6. Add resolved/owned ELF descriptors only after the wire model is sound.
7. Reintroduce loading and execution policy only after the format and memory
   layers have defensible boundaries.

The guiding criterion for this PR is not feature preservation. It is semantic
clarity.
