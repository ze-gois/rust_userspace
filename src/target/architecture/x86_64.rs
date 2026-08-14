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

    #[inline]
    pub fn align_down(value: u64) -> u64 {
        value & !(SIZE as u64 - 1)
    }

    #[inline]
    pub fn align_up(value: u64) -> Option<u64> {
        value.checked_add(SIZE as u64 - 1).map(align_down)
    }
}

pub use result::{Error, Ok, Result};

pub type PointerType = *const u64;

ample::struct_tuple!(
    #[derive(Debug)]
    pub struct Pointer(0: pub PointerType)
);

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
