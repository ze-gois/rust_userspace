//! Processor-specific ELF semantics layered beneath the generic gABI model.

pub mod aarch64;
pub mod x86_64;

#[cfg(target_arch = "aarch64")]
pub use aarch64 as target;

#[cfg(target_arch = "x86_64")]
pub use x86_64 as target;
