#[macro_use]
pub mod macros;

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "linux")]
pub use linux::*;

#[derive(Debug, Clone, Copy, Default)]
pub struct OperatingSystem;

pub type Os = OperatingSystem;
