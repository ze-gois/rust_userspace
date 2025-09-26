# Userspace

<div align="center">

![Version](https://img.shields.io/badge/version-0.1.0-blue.svg)
![License](https://img.shields.io/badge/license-BSD3-green.svg)
![Rust](https://img.shields.io/badge/rust-2024_edition-orange.svg)
![Architecture](https://img.shields.io/badge/arch-x86__64-purple.svg)
![Status](https://img.shields.io/badge/status-experimental-yellow.svg)

</div>

<div align="center">
  <strong>A modern standard library for userspace applications</strong><br>
  Safe, portable abstractions for systems programming without the standard library
</div>

<br>

<div align="center">
  <sub>Built with ❤️ by <a href="https://github.com/ze-gois">José Gois</a></sub>
</div>

<br>

## 📋 Overview

**Userspace** is a Rust implementation of a standard library for userspace applications, designed to work without depending on the Rust standard library (`no_std`). It provides safe abstractions for low-level operations, architecture-specific functionality, memory management, and executable file format handling.

### Key Features

- 🔒 **Memory Safety**: Leverage Rust's ownership model for secure systems programming
- 🧩 **Modular Architecture**: Well-defined components with clear interfaces
- 🔄 **Cross-Platform**: Architecture abstractions for portability (currently x86_64)
- 📦 **No Standard Library**: Works in `no_std` environments
- 📄 **ELF Support**: Parse and work with Executable and Linkable Format files
- 🧠 **Memory Management**: Stack manipulation and memory allocation utilities

## 🔍 Project Structure

```
userspace/
├── src/
│   ├── file/         # File format handling (ELF)
│   ├── macros/       # Utility macros
│   ├── memory/       # Memory management
│   │   ├── alloc/    # Allocation functionality
│   │   ├── page/     # Page management
│   │   └── stack/    # Stack handling
│   ├── target/       # Architecture abstractions
│   │   ├── architecture/   # CPU architecture specifics
│   │   └── operating_system/  # OS abstractions
│   ├── traits/       # Common interfaces
│   ├── types/        # Library-specific types
│   ├── entry.rs      # Binary entry point
│   ├── library.rs    # Main library definition
│   ├── panic.rs      # Panic handler
│   └── result.rs     # Error handling
├── Cargo.toml        # Project configuration
└── build.rs         # Build script
```

## 🚀 Getting Started

### Prerequisites

- Rust 2024 Edition or newer
- Cargo and Rustup

### Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
userspace = { git = "https://github.com/ze-gois/rust_userspace" }
```

### Usage Example

```rust
// Create a no_std binary
#![no_std]
#![no_main]

use userspace;

#[unsafe(no_mangle)]
pub extern "C" fn entry(stack_pointer: userspace::target::arch::PointerType) -> ! {
    // Convert raw stack pointer to a safe abstraction
    let stack = userspace::memory::Stack::from_pointer(
        userspace::target::arch::Pointer(stack_pointer)
    );

    // Access command-line arguments
    if let Some(arg) = stack.arguments.get(0) {
        userspace::info!("Program name: {:?}", arg);
    }

    // Work with the ELF format
    if let Some(arg0) = stack.arguments.get(0) {
        if !arg0.pointer.0.is_null() {
            unsafe {
                let cstr = core::ffi::CStr::from_ptr(arg0.pointer.0 as *mut i8);
                let path = cstr.to_str().unwrap();
                let elf = userspace::file::format::elf::header::Identifier::from_path(path);
                userspace::info!("ELF identifier: {:?}", elf);
            }
        }
    }

    loop {}
}
```

## 🛠️ Architecture

Userspace is designed with a layered architecture:

1. **Core Layer**: Basic types, traits and utilities
2. **Target Layer**: Architecture and OS abstractions
3. **Memory Layer**: Stack, pages, and allocation
4. **File Layer**: File format parsing and manipulation

Each layer builds upon the previous ones, providing increasingly higher-level abstractions while maintaining safety and performance.

### Memory Management

The memory subsystem provides:

- Safe stack traversal and argument extraction
- Page allocation primitives
- Basic heap allocation in no_std environments

### Architecture Abstraction

The target subsystem abstracts architecture details:

- Pointer types and operations
- Register access patterns
- CPU-specific features
- OS-specific functionality

Currently focused on x86_64, but designed to be extensible to other architectures.

## 🧪 Experimental Features

Userspace uses several experimental Rust features:

```rust
#![feature(generic_const_exprs)]
#![feature(generic_const_items)]
```

These enable advanced type-level programming required for zero-cost abstractions across architectures.

## 📚 Documentation

For more detailed documentation:

```bash
cargo doc --open
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -am 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📜 License

This project is licensed under the terms found in the [LICENSE](LICENSE) file.

## 🔮 Future Work

- Support for additional architectures (ARM, RISC-V)
- Enhanced file system abstractions
- Networking capabilities
- Threading and concurrency primitives
- Comprehensive test suite

---

<div align="center">
  <sub>
    Built for research purposes at the Federal University of Rio Grande do Norte (UFRN)<br>
    © 2023-2024 José Gois - https://userspace.builders
  </sub>
</div>
