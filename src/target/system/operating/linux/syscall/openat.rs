use crate::target::architecture::x86::bit64::syscall;

pub use super::open::{Error, Ok, Result};

pub const CURRENT_WORKING_DIRECTORY: isize = -100;
pub const AT_FDCWD: isize = CURRENT_WORKING_DIRECTORY;

#[cfg(target_arch = "x86_64")]
pub const NUMBER: usize = super::number::x86::bit64::OPENAT;

/// # Safety
///
/// `file_pathname` must point to a readable NUL-terminated pathname.
pub unsafe fn openat(
    directory_file_descriptor: isize,
    file_pathname: *const u8,
    flags: i32,
    mode: i32,
) -> crate::Result {
    let raw_return = unsafe {
        syscall::syscall4(
            NUMBER,
            directory_file_descriptor as usize,
            file_pathname as usize,
            flags as usize,
            mode as usize,
        )
    };

    handle_result(raw_return)
}

pub fn handle_result(raw: usize) -> crate::Result {
    let result = super::Return::new(raw);

    if result.is_error() {
        core::result::Result::Err(crate::Error::Target(
            crate::target::Error::OperatingSystem(
                crate::target::system::operating::linux::Error::Syscall(
                    crate::target::system::operating::linux::syscall::Error::Openat(
                        Error::Default(result.raw()),
                    ),
                ),
            ),
        ))
    } else {
        core::result::Result::Ok(crate::Ok::Target(
            crate::target::Ok::OperatingSystem(
                crate::target::system::operating::linux::Ok::Syscall(
                    crate::target::system::operating::linux::syscall::Ok::Openat(
                        Ok::Default(result.raw()),
                    ),
                ),
            ),
        ))
    }
}
