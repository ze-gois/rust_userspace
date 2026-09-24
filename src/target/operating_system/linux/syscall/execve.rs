use crate::target::architecture::{Architecture, traits::Callable};

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
    let arch_result = Architecture::syscall3(NUMBER, filename as usize, argv as usize, envp as usize);
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
        [7; E2BIG; ArgumentListTooLong; usize; "E2BIG"; "Argument and environment data or executable path is too large"],
        [13; EACCES; PermissionDenied; usize; "EACCES"; "Search or execute permission is denied, file is unsuitable, or filesystem is noexec"],
        [11; EAGAIN; TryAgain; usize; "EAGAIN"; "Process resource limit conditions prevent execve"],
        [14; EFAULT; InvalidPointer; usize; "EFAULT"; "Path, argument vector, or environment vector contains an invalid pointer"],
        [22; EINVAL; InvalidArgument; usize; "EINVAL"; "Executable format contains invalid or excessive interpreter information"],
        [5; EIO; InputOutput; usize; "EIO"; "Input/output error occurred while reading the executable"],
        [21; EISDIR; IsDirectory; usize; "EISDIR"; "An ELF interpreter is a directory"],
        [80; ELIBBAD; InvalidInterpreter; usize; "ELIBBAD"; "ELF interpreter is not in a recognized format"],
        [40; ELOOP; TooManySymbolicLinks; usize; "ELOOP"; "Too many symbolic links or recursive script interpretations were encountered"],
        [24; EMFILE; ProcessFileDescriptorLimit; usize; "EMFILE"; "Process file-descriptor limit was reached"],
        [36; ENAMETOOLONG; NameTooLong; usize; "ENAMETOOLONG"; "Executable or interpreter pathname is too long"],
        [23; ENFILE; SystemFileDescriptorLimit; usize; "ENFILE"; "System-wide open-file limit was reached"],
        [2; ENOENT; FileNotFound; usize; "ENOENT"; "Executable, script interpreter, or ELF interpreter was not found"],
        [8; ENOEXEC; InvalidExecutable; usize; "ENOEXEC"; "Executable format is not recognized or is otherwise invalid"],
        [12; ENOMEM; OutOfMemory; usize; "ENOMEM"; "Insufficient kernel memory is available"],
        [20; ENOTDIR; NotDirectory; usize; "ENOTDIR"; "A pathname component is not a directory"],
        [1; EPERM; OperationNotPermitted; usize; "EPERM"; "Filesystem, process state, or capability rules forbid execution"],
        [26; ETXTBSY; TextFileBusy; usize; "ETXTBSY"; "Executable is open for writing by one or more processes"],
        [4096; ERROR; Default; usize; "UNKNOWN"; "Unclassified Linux errno"],
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
        crate::Result::Ok(crate::Ok::Target(crate::target::Ok::Architecture(
            crate::target::architecture::Ok::X86_64Syscall(
                crate::target::architecture::syscall::Ok::X86_64Syscall3(
                    crate::target::architecture::syscall::syscall3::Ok::Default(value),
                ),
            ),
        ))) => core::result::Result::Ok(crate::Ok::Target(crate::target::Ok::OperatingSystem(
            crate::target::operating_system::Ok::Syscall(crate::target::operating_system::syscall::Ok::Execve(
                crate::target::operating_system::syscall::execve::Ok::Default(value),
            )),
        ))),
        _ => core::result::Result::Err(crate::Error::Target(crate::target::Error::OperatingSystem(
            crate::target::operating_system::Error::Syscall(crate::target::operating_system::syscall::Error::Execve(
                Error::Default(1),
            )),
        ))),
    }
}
