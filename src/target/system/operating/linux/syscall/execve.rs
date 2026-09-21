use crate::target::architecture::x86::bit64::syscall;

#[cfg(target_arch = "x86_64")]
pub const NUMBER: usize = super::number::x86::bit64::EXECVE;

/// Replace the current process image using Linux's `execve(2)` system call.
///
/// `filename`, `argv`, and `envp` must point to NUL-terminated strings and
/// NUL-terminated pointer arrays respectively. The call returns only when
/// replacing the process image fails.
/// # Safety
///
/// `file_pathname` must point to a readable NUL-terminated pathname.
/// `argument_vector` and `environment_vector` must point to readable,
/// NUL-terminated pointer arrays whose non-null entries point to readable
/// NUL-terminated strings.
#[inline(always)]
pub unsafe fn execve(
    file_pathname: *const u8,
    argument_vector: *const *const u8,
    environment_vector: *const *const u8,
) -> crate::Result {
    let raw_return = unsafe {
        syscall::syscall3(
            NUMBER,
            file_pathname as usize,
            argument_vector as usize,
            environment_vector as usize,
        )
    };
    handle_result(raw_return)
}

pub fn handle_result(raw: usize) -> crate::Result {
    match super::Return::new(raw).classify() {
        core::result::Result::Ok(success) => core::result::Result::Ok(
            crate::Ok::Target(crate::target::Ok::OperatingSystem(
                crate::target::system::operating::linux::Ok::Syscall(
                    crate::target::system::operating::linux::syscall::Ok::Execve(success),
                ),
            )),
        ),
        core::result::Result::Err(failure) => core::result::Result::Err(
            crate::Error::Target(crate::target::Error::OperatingSystem(
                crate::target::system::operating::linux::Error::Syscall(
                    crate::target::system::operating::linux::syscall::Error::Execve(failure),
                ),
            )),
        ),
    }
}
