use crate::target::arch::{Arch, traits::Callable};

hooking!(EXECVE);

/// Replace the current process image using Linux's `execve(2)` system call.
///
/// `filename`, `argv`, and `envp` must point to NUL-terminated strings and
/// NUL-terminated pointer arrays respectively. The call returns only when
/// replacing the process image fails.
#[inline(always)]
pub fn execve(
    filename: *const u8,
    argv: *const *const u8,
    envp: *const *const u8,
) -> crate::Result {
    let arch_result = Arch::syscall3(NUMBER, filename as usize, argv as usize, envp as usize);
    handle_result(arch_result)
}

pub mod ok {
    ample::result!(Ok; "Execve Ok"; usize; [
        [0; OK; Default; usize; "Ok"; "Execve succeeded"],
    ]);

    impl Ok {
        pub fn from_no(no: usize) -> Self {
            Ok::Default(no)
        }
    }
}

pub mod error {
    ample::result!(Error; "Execve error"; usize; [
        [1; ERROR; Default; usize; "Error"; "Execve failed"],
        [2; ENOENT; FileNotFound; usize; "ENOENT"; "Executable or interpreter not found"],
        [8; ENOEXEC; InvalidExecutable; usize; "ENOEXEC"; "Invalid executable format"],
        [13; EACCES; PermissionDenied; usize; "EACCES"; "Permission denied"],
        [14; EFAULT; InvalidPointer; usize; "EFAULT"; "Invalid pointer"],
        [22; EINVAL; InvalidArgument; usize; "EINVAL"; "Invalid executable argument"],
        [7; E2BIG; ArgumentListTooLong; usize; "E2BIG"; "Argument list too long"],
    ]);

    impl Error {
        pub fn from_no(no: usize) -> Self {
            Error::Default(no)
        }
    }
}

pub use error::Error;
pub use ok::Ok;

pub type Result = core::result::Result<Ok, Error>;

pub fn handle_result(result: crate::Result) -> crate::Result {
    match result {
        crate::Result::Ok(crate::Ok::Target(crate::target::Ok::Arch(
            crate::target::arch::Ok::X86_64Syscall(
                crate::target::arch::syscall::Ok::X86_64Syscall3(
                    crate::target::arch::syscall::syscall3::Ok::Default(value),
                ),
            ),
        ))) => core::result::Result::Ok(crate::Ok::Target(crate::target::Ok::Os(
            crate::target::os::Ok::Syscall(crate::target::os::syscall::Ok::Execve(
                crate::target::os::syscall::execve::Ok::Default(value),
            )),
        ))),
        _ => core::result::Result::Err(crate::Error::Target(crate::target::Error::Os(
            crate::target::os::Error::Syscall(crate::target::os::syscall::Error::Execve(
                Error::Default(1),
            )),
        ))),
    }
}
