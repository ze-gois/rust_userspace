# Project Revision

This branch is a deliberate semantic reset of `rust_userspace`.

The revision favors a smaller, defensible architecture over preserving legacy
functionality. Temporary regressions are acceptable when existing behavior
depends on unclear ownership, misplaced concepts, duplicated sources of truth,
or abstractions whose names no longer match their meaning.

## Architectural laws

- ELF is a file format and, when rebuilt, belongs under `file::format::elf`.
- The previous ELF implementation was deleted rather than repaired around
  legacy assumptions.
- Module names do not encode hierarchy with underscores. Hierarchy belongs in
  modules; concrete nouns precede qualifications.
- Acronyms and abbreviations are aliases rather than canonical vocabulary,
  except where a normative external identifier is reproduced at a standards
  boundary.
- ABI names and constants reproduced from standards retain traceability to the
  standard that names them.
- Every identifier must state what it means.
- `memory::stack` is an educational, operating-system-neutral model of stack
  memory.
- Linux process startup does not define generic memory.
- `memory::heap` and `memory::stack` describe memory concepts, not file
  formats or operating-system conventions.
- `Allocating` is the allocation abstraction; allocation layout is described
  by `core::alloc::Layout` and is independent from representation size.
- Representation, Rust memory layout, residency, and ownership are distinct.
- Serialization and deserialization are operations over representation.

## Lexicon discipline

The project follows community and standards terminology before inventing local
terminology.

Canonical identifiers describe the concrete things they name. Short forms
belong in aliases. Module hierarchy expresses grammatical hierarchy: concrete
nouns precede qualifications, and underscore-composed module names should be
split when the underscore hides a real submodule relation.

Where Rust organization differs from a specification's flat C vocabulary, the
Rust hierarchy may change spelling while preserving an explicit semantic
mapping to the normative standard term.

## Revision status

### Sprint I.2 — Representation Semantics

Completed:

- `REPRESENTATION_SIZE` is the canonical representation extent;
- `BYTES_ALIGN` no longer exists in `Bytes`;
- allocation no longer derives object storage from representation.

### Sprint I.3 — Allocation Semantics

Completed:

- `memory::heap::Allocator` implements `ample::traits::Allocating` directly;
- typed allocation derives `Layout::array::<T>`;
- the global allocator preserves the original `mmap` base pointer;
- allocation no longer requires `T: Bytes`;
- the legacy `Allocatable` / `AllocatableResult` bridge is gone.

### Sprint I.4 — Memory Reformation

In progress:

- process startup ABI was removed from `memory::stack`;
- `arguments`, `environment`, `auxiliary`, startup building, and the
  legacy stack list were removed from generic memory;
- `Stack` now describes a memory region using lower and upper bounds, a stack
  pointer, and growth direction;
- `page` now exposes direct `align_down` and `align_up` operations;
- obsolete memory result and origin layers were removed;
- the legacy ELF implementation, its tests, fixtures, and external leaks were
  deleted early because they constrained the memory revision with incorrect
  semantics.

Deleting the old ELF implementation does not complete the later ELF sprints.
`file::format::elf` will be rebuilt from standards after the memory, target,
process, and file boundaries are sound.

## Intended direction

```text
userspace
├── memory
│   ├── alloc
│   ├── heap
│   ├── page
│   └── stack
├── file
└── target
```

The tree is directional rather than a compatibility promise.

The guiding criterion for this PR is semantic clarity, not feature
preservation.
