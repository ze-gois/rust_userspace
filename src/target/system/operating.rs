//! Operating-system target boundary.
//!
//! The canonical phrase is “operating system”. Project module grammar places
//! the concrete noun first, hence `system::operating`. Concrete operating
//! systems remain explicit children such as `system::operating::linux`.

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "linux")]
pub use linux::{Error, Ok, Result};
