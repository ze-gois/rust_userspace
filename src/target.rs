#![no_std]
#![allow(unused)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(generic_const_items)]

pub mod architecture;
pub mod operating_system;
pub mod result;
pub mod system;

pub use architecture as arch;
pub use operating_system as os;

pub use result::{Error, Ok, Result};
