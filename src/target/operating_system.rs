#[macro_use]
pub mod macros;
pub mod traits;

pub mod linux;
pub use linux::*;

pub struct OperatingSystem;

pub type Os = OperatingSystem;
