use crate::target::arch::{Arch, traits::Callable};

pub use super::open::{Error, Ok, Result};

pub const CURRENT_WORKING_DIRECTORY: isize = -100;
pub const AT_FDCWD: isize = CURRENT_WORKING_DIRECTORY;

hooking!(OPENAT);

pub fn openat(
    directory_file_descriptor: isize,
    file_pathname: *const u8,
    flags: i32,
    mode: i32,
) -> crate::Result {
    let syscall_result = Arch::syscall4(
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
            crate::target::arch::Ok::X86_64Syscall(
                crate::target::arch::syscall::Ok::X86_64Syscall4(
                    crate::target::arch::syscall::syscall4::Ok::Default(value),
                ),
            ),
        ))) => core::result::Result::Ok(crate::Ok::Target(
            crate::target::Ok::OperatingSystem(crate::target::os::Ok::Syscall(
                crate::target::os::syscall::Ok::OpenAt(Ok::Default(value)),
            )),
        )),
        _ => core::result::Result::Err(crate::Error::Target(
            crate::target::Error::OperatingSystem(crate::target::os::Error::Syscall(
                crate::target::os::syscall::Error::OpenAt(Error::Default(3)),
            )),
        )),
    }
}
