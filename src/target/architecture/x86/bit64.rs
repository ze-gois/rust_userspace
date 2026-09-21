#![no_std]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(generic_const_items)]

pub mod callable;
pub mod result;
pub mod syscall;

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

pub type RawPointer = *const u8;
pub type PointerType = RawPointer;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pointer(pub RawPointer);

impl Pointer {
    pub fn current() -> Self {
        let pointer: RawPointer;
        unsafe { core::arch::asm!("mov {}, rsp", out(reg) pointer) };
        Self(pointer)
    }

    pub const fn as_ptr(self) -> RawPointer {
        self.0
    }
}
