use crate::target::arch::{Arch, traits::Callable};

pub use super::open::flags;
pub use super::open::{Error, Ok, Result};

hooking!(OPENAT4);

pub fn openat4(
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
    // Err(crate::Error::Default(1))
    match result {
        crate::Result::Ok(crate::Ok::Target(crate::target::Ok::Arch(
            crate::target::arch::Ok::X86_64Syscall(
                crate::target::arch::syscall::Ok::X86_64Syscall4(
                    crate::target::arch::syscall::syscall4::Ok::Default(m),
                ),
            ),
        ))) => core::result::Result::Ok(crate::Ok::Target(crate::target::Ok::Os(
            crate::target::os::Ok::Syscall(crate::target::os::syscall::Ok::Open(
                crate::target::os::syscall::open::Ok::OPENAT4(m),
            )),
        ))),
        _ => core::result::Result::Err(crate::Error::Target(crate::target::Error::Os(
            crate::target::os::Error::Syscall(crate::target::os::syscall::Error::Open(
                Error::Default(3),
            )),
        ))),
    }
}
