//! File domain.
//!
//! File capabilities are introduced independently as their semantics are
//! justified. For now the public surface contains only file printing.

pub mod format;
pub mod print;
pub use print::print;
