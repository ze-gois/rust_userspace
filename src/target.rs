#![no_std]
#![allow(unused)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(generic_const_items)]

pub mod architecture;
pub mod result;
pub mod system;

pub use architecture::Architecture;
pub use architecture::Architecture as Arch;
pub use architecture as arch;

pub use system::operating::OperatingSystem;
pub use system::operating::OperatingSystem as Os;
pub use system::operating as os;

pub use result::{Error, Ok, Result};
