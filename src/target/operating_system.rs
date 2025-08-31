#[macro_use]
pub mod macros;
pub mod traits;

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "linux")]
pub use linux::*;

#[cfg(not(target_os = "linux"))]
pub mod linux;
#[cfg(not(target_os = "linux"))]
pub use linux::*;

pub struct Os;
