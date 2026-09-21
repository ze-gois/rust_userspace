use crate::target::architecture::x86::bit64::syscall;

pub mod stat;
pub use stat::Stat;

#[cfg(target_arch = "x86_64")]
pub const NUMBER: usize = super::number::x86::bit64::FSTAT;

/// # Safety
///
/// `status` must designate writable storage large enough for one Linux
/// `struct stat` value for the active ABI.
#[inline(always)]
pub unsafe fn fstat(file_descriptor: isize, status: *mut Stat) -> crate::Result {
    let raw_return = unsafe {
        syscall::syscall2(NUMBER, file_descriptor as usize, status as usize)
    };
    handle_result(raw_return)
}

pub fn handle_result(raw: usize) -> crate::Result {
    match super::Return::new(raw).classify() {
        core::result::Result::Ok(success) => core::result::Result::Ok(
            crate::Ok::Target(crate::target::Ok::OperatingSystem(
                crate::target::system::operating::linux::Ok::Syscall(
                    crate::target::system::operating::linux::syscall::Ok::Fstat(success),
                ),
            )),
        ),
        core::result::Result::Err(failure) => core::result::Result::Err(
            crate::Error::Target(crate::target::Error::OperatingSystem(
                crate::target::system::operating::linux::Error::Syscall(
                    crate::target::system::operating::linux::syscall::Error::Fstat(failure),
                ),
            )),
        ),
    }
}
