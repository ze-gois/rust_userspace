use crate::target::architecture::x86::bit64::syscall;

pub use super::mmap::Protection;

#[cfg(target_arch = "x86_64")]
pub const NUMBER: usize = super::number::x86::bit64::MPROTECT;

/// # Safety
///
/// The caller must ensure that changing protection for the region does not
/// invalidate live references or executing code assumptions.
#[inline(always)]
pub unsafe fn mprotect(
    address: *mut u8,
    length: usize,
    protection: Protection,
) -> crate::Result {
    let raw_return = unsafe {
        syscall::syscall3(NUMBER, address as usize, length, protection.bits() as usize)
    };
    handle_result(raw_return)
}

pub fn handle_result(raw: usize) -> crate::Result {
    match super::Return::new(raw).classify() {
        core::result::Result::Ok(success) => core::result::Result::Ok(
            crate::Ok::Target(crate::target::Ok::OperatingSystem(
                crate::target::system::operating::linux::Ok::Syscall(
                    crate::target::system::operating::linux::syscall::Ok::Mprotect(success),
                ),
            )),
        ),
        core::result::Result::Err(failure) => core::result::Result::Err(
            crate::Error::Target(crate::target::Error::OperatingSystem(
                crate::target::system::operating::linux::Error::Syscall(
                    crate::target::system::operating::linux::syscall::Error::Mprotect(failure),
                ),
            )),
        ),
    }
}
