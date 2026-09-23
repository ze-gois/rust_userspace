use crate::target::architecture::{Architecture, traits::Callable};

hooking!(FORK);

/// Create a new process using Linux's `fork(2)` system call.
///
/// The returned value is `0` in the child process and the child PID in the
/// parent process. On failure, the result contains the kernel error value.
#[inline(always)]
pub fn fork() -> crate::Result {
    let arch_result = Architecture::syscall0(NUMBER);
    handle_result(arch_result)
}

pub mod ok {
    ample::result!(Ok; "Fork Ok"; usize; [
        [0; OK; Default; usize; "Ok"; "Fork succeeded"],
    ]);

    impl Ok {
        pub fn from_no(no: usize) -> Self {
            Ok::Default(no)
        }
    }
}

pub mod error {
    ample::result!(Error; "Fork error"; usize; [
        [11; EAGAIN; TryAgain; usize; "EAGAIN"; "A process, thread, PID, cgroup, or scheduler limit prevents fork"],
        [12; ENOMEM; OutOfMemory; usize; "ENOMEM"; "Kernel memory is insufficient or the PID namespace cannot create a child"],
        [38; ENOSYS; NotImplemented; usize; "ENOSYS"; "Fork is not supported on this platform"],
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
                crate::target::architecture::syscall::Ok::X86_64Syscall0(
                    crate::target::architecture::syscall::syscall0::Ok::Default(value),
                ),
            ),
        ))) => core::result::Result::Ok(crate::Ok::Target(crate::target::Ok::OperatingSystem(
            crate::target::operating_system::Ok::Syscall(crate::target::operating_system::syscall::Ok::Fork(
                crate::target::operating_system::syscall::fork::Ok::Default(value),
            )),
        ))),
        _ => core::result::Result::Err(crate::Error::Target(crate::target::Error::OperatingSystem(
            crate::target::operating_system::Error::Syscall(crate::target::operating_system::syscall::Error::Fork(
                Error::Default(1),
            )),
        ))),
    }
}
