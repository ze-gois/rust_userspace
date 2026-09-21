pub mod x86;

#[cfg(target_arch = "x86_64")]
pub use x86::bit64::{Error, Ok, Pointer, PointerType, RawPointer, Result, page};


#[derive(Debug, Clone, Copy, Default)]
pub struct Architecture;

pub type Arch = Architecture;

impl core::ops::Sub for Pointer {
    type Output = usize;

    fn sub(self, rhs: Self) -> Self::Output {
        self.0 as usize - rhs.0 as usize
    }
}

impl core::ops::Add<usize> for Pointer {
    type Output = Pointer;

    fn add(self, offset: usize) -> Self::Output {
        Pointer(unsafe { self.0.add(offset) })
    }
}
