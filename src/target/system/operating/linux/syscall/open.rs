use crate::target::architecture::x86::bit64::syscall;

pub mod access;
pub use access::Access;

pub mod flags;
pub use flags::Flags;

pub mod mode;
pub use mode::Mode;

#[cfg(target_arch = "x86_64")]
pub const NUMBER: usize = super::number::x86::bit64::OPEN;

/// # Safety
///
/// `file_pathname` must point to a readable NUL-terminated pathname.
pub unsafe fn open(
    file_pathname: *const u8,
    access: Access,
    flags: Flags,
    mode: Mode,
) -> crate::Result {
    let raw_return = unsafe {
        syscall::syscall3(
            NUMBER,
            file_pathname as usize,
            (access.raw() | flags.bits()) as usize,
            mode.bits() as usize,
        )
    };

    handle_result(raw_return)
}

pub fn handle_result(raw: usize) -> crate::Result {
    match super::Return::new(raw).classify() {
        core::result::Result::Ok(success) => core::result::Result::Ok(
            crate::Ok::Target(crate::target::Ok::OperatingSystem(
                crate::target::system::operating::linux::Ok::Syscall(
                    crate::target::system::operating::linux::syscall::Ok::Open(success),
                ),
            )),
        ),
        core::result::Result::Err(failure) => core::result::Result::Err(
            crate::Error::Target(crate::target::Error::OperatingSystem(
                crate::target::system::operating::linux::Error::Syscall(
                    crate::target::system::operating::linux::syscall::Error::Open(failure),
                ),
            )),
        ),
    }
}
