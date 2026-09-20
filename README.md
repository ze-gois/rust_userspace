# userspace

[![crates.io](https://img.shields.io/crates/v/userspace.svg)](https://crates.io/crates/userspace)
[![docs.rs](https://docs.rs/userspace/badge.svg)](https://docs.rs/userspace)

A `no_std` userspace systems layer for the [userspace.party](https://userspace.party) ecosystem.

## Role

`userspace` explores the machinery a process needs below a conventional standard library: operating-system entry points, memory, files, executable loading, target-specific behavior, and freestanding startup.

The project currently focuses on Linux/x86_64 while keeping architecture and operating-system concerns separated in the source tree.

## Current surface

The crate contains work around:

- files, seeking, opening and low-level I/O;
- ELF parsing and loading;
- memory allocation, pages, heap and process stack handling;
- architecture- and OS-specific target modules;
- freestanding entry/startup and panic paths;
- shared traits and result types built on `ample`.

The ELF path includes `PT_LOAD` mapping, PIE/`ET_DYN` load bias, `PT_INTERP` handling, segment permissions, auxiliary-vector preparation, and rollback of newly created mappings when loading fails.

## Use

```bash
cargo add userspace
```

The default configuration is `no_std`. A `with_std` feature exists for host/build-side contexts where the standard library is intentionally available.

For local API documentation:

```bash
cargo doc --open
```

## Development

Host-side loader fixtures and regression tests live under `tests/`. The repository also contains its own linker/build configuration for freestanding execution.

## Ecosystem

- Ecosystem: https://userspace.party
- Crate homepage: https://userspace.party/userspace
- API documentation: https://docs.rs/userspace
- crates.io: https://crates.io/crates/userspace
- Source: https://github.com/ze-gois/rust_userspace
- Workspace hub: https://github.com/ze-gois/rust_userspace_hub

`userspace` is developed independently and published in dependency order as part of the coordinated hub release.

## Status

Experimental. The project is suitable for systems research and active development; interfaces should not yet be treated as stable.

## License

See [LICENSE](LICENSE).
