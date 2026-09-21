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
  modules; modules name concrete nouns before qualifications. For example,
  `stack::initial`, not `initial_stack`.
- Acronyms and abbreviations are aliases rather than canonical vocabulary,
  except where a normative external identifier is reproduced at a standards
  boundary.
- ABI names, constants, and other identifiers reproduced from standards may
  retain their normative spelling and must remain traceable to that standard.
- Every identifier must state what it means; misleading naming is a semantic
  defect, not merely a style problem.
- `memory::stack` is preserved as an educational, operating-system-neutral
  model of stack memory.
- Linux process startup semantics do not define generic memory.
- `memory::heap` and `memory::stack` describe memory concepts, not file
  formats or operating-system conventions.
- `Allocating` is the allocation abstraction; allocation layout is described
  by `core::alloc::Layout` and is independent from representation size.
- Representation, in-memory Rust layout, residency, and ownership are distinct
  concepts.
- Serialization and deserialization are operations over representation.

## Lexicon discipline

The project follows community and standards terminology before inventing local
terminology.

A canonical identifier describes the concrete thing it names. Short forms
belong in aliases. Module hierarchy expresses grammatical hierarchy: concrete
nouns precede qualifications, and underscore-composed module names should be
split when the underscore hides a real submodule relation.

Where Rust organization differs from a specification's flat C vocabulary, the
Rust hierarchy may change the spelling while preserving an explicit semantic
mapping to the normative standard term.

## Current revision state

Representation:

- `REPRESENTATION_SIZE` is the canonical representation extent;
- `BYTES_ALIGN` no longer exists in `Bytes`.

Allocation:

- `memory::heap::Allocator` implements `ample::traits::Allocating` directly;
- typed allocation derives `Layout::array::<T>`;
- the global allocator preserves the original `mmap` base pointer;
- allocation no longer requires `T: Bytes`;
- the legacy `AllocatableResult` bridge was removed from the ample/userspace
  boundary.

## Intended direction

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
    ├── architecture
    └── system
        └── operating
            └── linux
                ├── syscall
                └── process
                    └── stack
                        └── initial
```

The tree is directional rather than a compatibility promise.

The guiding criterion for this PR is semantic clarity, not feature
preservation.
