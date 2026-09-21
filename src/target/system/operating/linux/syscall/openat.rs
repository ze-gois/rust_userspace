use crate::target::architecture::x86::bit64::syscall;

pub use super::open::{Error, Ok, Result};

pub const CURRENT_WORKING_DIRECTORY: isize = -100;
pub const AT_FDCWD: isize = CURRENT_WORKING_DIRECTORY;

#[cfg(target_arch = "x86_64")]
pub const NUMBER: usize = super::number::x86::bit64::OPENAT;

pub fn openat(
    directory_file_descriptor: isize,
    file_pathname: *const u8,
    flags: i32,
    mode: i32,
) -> crate::Result {
    let syscall_result = syscall::syscall4(
        NUMBER,
        directory_file_descriptor as usize,
        file_pathname as usize,
        flags as usize,
        mode as usize,
    );

    handle_result(syscall_result)
}

pub fn handle_result(result: crate::Result) -> crate::Result {
    match result {
        crate::Result::Ok(crate::Ok::Target(crate::target::Ok::Architecture(
            crate::target::architecture::Ok::Syscall(
                crate::target::architecture::x86::bit64::syscall::Ok::Syscall4(
                    crate::target::architecture::x86::bit64::syscall::syscall4::Ok::Default(value),
                ),
            ),
        ))) => core::result::Result::Ok(crate::Ok::Target(
            crate::target::Ok::OperatingSystem(crate::target::system::operating::linux::Ok::Syscall(
                crate::target::system::operating::linux::syscall::Ok::Openat(Ok::Default(value)),
            )),
        )),
        crate::Result::Err(crate::Error::Target(crate::target::Error::Architecture(
            crate::target::architecture::Error::Syscall(
                crate::target::architecture::x86::bit64::syscall::Error::Syscall4(
                    crate::target::architecture::x86::bit64::syscall::syscall4::Error::Default(raw),
                ),
            ),
        ))) => core::result::Result::Err(crate::Error::Target(
            crate::target::Error::OperatingSystem(
                crate::target::system::operating::linux::Error::Syscall(
                    crate::target::system::operating::linux::syscall::Error::Openat(
                        Error::Default(raw),
                    ),
                ),
            ),
        )),
        _ => core::result::Result::Err(crate::Error::Target(
            crate::target::Error::OperatingSystem(
                crate::target::system::operating::linux::Error::Syscall(
                    crate::target::system::operating::linux::syscall::Error::Openat(
                        Error::Default(usize::MAX),
                    ),
                ),
            ),
        )),
    }
}
