pub mod result;
pub mod syscall;

pub mod page {
    /// Base x86-64 page size in bytes.
    ///
    /// x86-64 also supports larger page sizes; this constant names the base
    /// page size rather than implying that every page has one size.
    pub const BASE_SIZE: usize = 0x1000;
}

pub use result::{Error, Ok, Result};

/// Raw x86-64 stack pointer value.
pub type StackPointer = *const u8;
