pub mod x86;

#[cfg(target_arch = "x86_64")]
pub use x86::bit64::{StackPointer, page};
