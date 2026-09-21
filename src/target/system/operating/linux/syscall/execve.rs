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

pub fn handle_result(raw: usize) -> crate::Result {
    let result = super::Return::new(raw);

    if result.is_error() {
        core::result::Result::Err(crate::Error::Target(
            crate::target::Error::OperatingSystem(
                crate::target::system::operating::linux::Error::Syscall(
                    crate::target::system::operating::linux::syscall::Error::Execve(
                        Error::Default(result.raw()),
                    ),
                ),
            ),
        ))
    } else {
        core::result::Result::Ok(crate::Ok::Target(
            crate::target::Ok::OperatingSystem(
                crate::target::system::operating::linux::Ok::Syscall(
                    crate::target::system::operating::linux::syscall::Ok::Execve(
                        Ok::Default(result.raw()),
                    ),
                ),
            ),
        ))
    }
}
