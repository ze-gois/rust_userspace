# Project Revision

This branch is a deliberate semantic reset of `rust_userspace`.

The revision favors a smaller, defensible architecture over preserving legacy
functionality. Temporary regressions are acceptable when existing behavior
depends on unclear ownership, misplaced concepts, duplicated sources of truth,
or abstractions whose names no longer match their meaning.

## Architectural laws

- ELF is a file format and belongs under `file::format::elf`.
- The previous ELF implementation was deleted rather than repaired around
  legacy assumptions.
- Internal module names do not encode hierarchy with underscores.
- Concrete nouns precede qualifications in module hierarchy.
- Acronyms and abbreviations are aliases rather than canonical vocabulary,
  except where a normative external identifier is reproduced at a standards
  boundary.
- Names and constants reproduced from standards remain traceable to the
  standard that names them.
- Every identifier must state what it means.
- `memory::stack` is an educational, operating-system-neutral model of stack
  memory.
- Generic memory does not depend on Linux process or syscall semantics.
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

Where Rust organization differs from a specification's flat vocabulary, the
Rust hierarchy may change spelling while preserving an explicit semantic
mapping to the normative standard term.

## Revision status

### Sprint I.2 — Representation Semantics

Completed.

- `REPRESENTATION_SIZE` is the canonical representation extent.
- `BYTES_ALIGN` was removed from `Bytes`.
- allocation no longer derives object storage from representation.

### Sprint I.3 — Allocation Semantics

Completed.

- `ample::traits::Allocating` is the canonical allocation capability.
- allocation consumes `core::alloc::Layout`.
- `Allocatable` and `AllocatableResult` were removed.
- the global allocator preserves the original allocation base pointer.

### Sprint I.4 — Memory Reformation

Completed.

- process startup ABI was removed from `memory::stack`;
- `Stack` describes a bounded memory region, pointer, and growth direction;
- `memory::page` exposes direct `align_down` and `align_up` operations;
- `memory::allocator::global` replaced the abbreviated `memory::alloc`;
- `memory::heap::Allocator` is operating-system neutral;
- Linux supplies the concrete allocation implementation under
  `target::system::operating::linux::memory::heap`.

### Sprint I.5 — Target & Process Boundary

Completed.

- the canonical operating-system hierarchy is
  `target::system::operating`;
- `target::os` and `Os` remain aliases only;
- the canonical architecture type is `Architecture`; `Arch` is an alias;
- x86-64 implementation code is organized as
  `target::architecture::x86::bit64`, while the normative Rust target name
  `x86_64` remains documented at the target boundary;
- target result domains use `Architecture` and `OperatingSystem`, not
  abbreviations;
- raw architecture pointers are byte-addressed;
- false Linux `openat4` code was removed;
- Linux `openat` now uses its four-argument ABI;
- `lseek::whence::Whence` and `mmap::protection::Protection` use the
  standards vocabulary while exposing normative constants such as
  `SEEK_SET` and `PROT_READ`.

### Sprint I.6 — File Substrate

Completed.

- legacy `Readable` was removed;
- raw `load`, `open`, `seek`, `information`, and `print` helpers were
  removed from the generic file domain;
- fabricated `'static` slices and ownerless raw file buffers were removed;
- `File` and `Information` are no longer falsely generated as
  representations;
- `file` remains as an intentionally minimal domain boundary until concrete
  capabilities are justified.

### Sprint I.7 — ELF Zero

In progress.

The old ELF tree was deleted early as trimming. The new ELF model starts from
the normative GABI and ample's representation primitives; no legacy ELF type is
a compatibility constraint.

The guiding criterion for this PR is semantic clarity, not feature
preservation.
