#![no_std]
#![allow(unused)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(generic_const_items)]

pub mod architecture;
pub mod operating_system;
pub mod result;

pub use architecture as arch;
pub use architecture::Arch;
pub use operating_system as os;
pub use operating_system::Os;

pub use result::{Error, Ok, Result};
