#![no_std]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(generic_const_items)]

pub mod callable;
pub mod result;
pub mod syscall;
// pub use syscall::*;

pub mod page {
    pub const SIZE: usize = 0x1000;
}

pub use result::{Error, Ok, Result};

pub type PointerType = *const u64;

#[derive(Debug, Clone, Copy)]
pub struct Pointer(pub PointerType);

impl Pointer {
    pub fn current() -> Self {
        let p: PointerType;
        unsafe { core::arch::asm!("mov {}, rsp", out(reg) p) };
        Pointer(p)
    }

    pub fn as_ptr(self) -> PointerType {
        self.0
    }
}
