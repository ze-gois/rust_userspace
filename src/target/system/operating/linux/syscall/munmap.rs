use crate::target::architecture::x86::bit64::syscall;

#[cfg(target_arch = "x86_64")]
pub const NUMBER: usize = super::number::x86::bit64::MUNMAP;

/// # Safety
///
/// No live reference, pointer owner, or executable code may continue to rely on
/// the unmapped region after this call succeeds.
#[inline(always)]
pub unsafe fn munmap(address: *mut u8, length: usize) -> crate::Result {
    let raw_return = unsafe { syscall::syscall2(NUMBER, address as usize, length) };
    handle_result(raw_return)
}

pub fn handle_result(raw: usize) -> crate::Result {
    match super::Return::new(raw).classify() {
        core::result::Result::Ok(success) => core::result::Result::Ok(
            crate::Ok::Target(crate::target::Ok::OperatingSystem(
                crate::target::system::operating::linux::Ok::Syscall(
                    crate::target::system::operating::linux::syscall::Ok::Munmap(success),
                ),
            )),
        ),
        core::result::Result::Err(failure) => core::result::Result::Err(
            crate::Error::Target(crate::target::Error::OperatingSystem(
                crate::target::system::operating::linux::Error::Syscall(
                    crate::target::system::operating::linux::syscall::Error::Munmap(failure),
                ),
            )),
        ),
    }
}
