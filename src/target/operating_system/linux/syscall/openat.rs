use crate::target::architecture::{Architecture, traits::Callable};

pub use crate::target::operating_system::syscall::open::flags;
pub use crate::target::operating_system::syscall::open::{Error, Ok, Result};

hooking!(OPENAT);

pub fn openat(
    directory_file_descriptor: isize,
    file_pathname: *const u8,
    flags: i32,
    mode: i32,
) -> crate::Result {
    let syscall_result = Architecture::syscall4(
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
            crate::target::architecture::Ok::X86_64Syscall(
                crate::target::architecture::syscall::Ok::X86_64Syscall4(
                    crate::target::architecture::syscall::syscall4::Ok::Default(m),
                ),
            ),
        ))) => core::result::Result::Ok(crate::Ok::Target(
            crate::target::Ok::OperatingSystem(
                crate::target::operating_system::Ok::Syscall(
                    crate::target::operating_system::syscall::Ok::Open(
                        crate::target::operating_system::syscall::open::Ok::OPENAT(m),
                    ),
                ),
            ),
        )),
        _ => core::result::Result::Err(crate::Error::Target(
            crate::target::Error::OperatingSystem(
                crate::target::operating_system::Error::Syscall(
                    crate::target::operating_system::syscall::Error::Open(
                        Error::Default(3),
                    ),
                ),
            ),
        )),
    }
}
