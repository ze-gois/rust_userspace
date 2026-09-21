use crate::target::arch::{Arch, traits::Callable};

hooking!(FORK);

/// Create a new process using Linux's `fork(2)` system call.
///
/// The returned value is `0` in the child process and the child PID in the
/// parent process. On failure, the result contains the kernel error value.
#[inline(always)]
pub fn fork() -> crate::Result {
    let arch_result = Arch::syscall0(NUMBER);
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
        [1; ERROR; Default; usize; "Error"; "Fork failed"],
        [12; ENOMEM; OutOfMemory; usize; "ENOMEM"; "Insufficient memory"],
        [11; EAGAIN; ProcessLimit; usize; "EAGAIN"; "Process limit reached"],
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
                crate::target::arch::syscall::Ok::X86_64Syscall0(
                    crate::target::arch::syscall::syscall0::Ok::Default(value),
                ),
            ),
        ))) => core::result::Result::Ok(crate::Ok::Target(crate::target::Ok::Os(
            crate::target::os::Ok::Syscall(crate::target::os::syscall::Ok::Fork(
                crate::target::os::syscall::fork::Ok::Default(value),
            )),
        ))),
        _ => core::result::Result::Err(crate::Error::Target(crate::target::Error::Os(
            crate::target::os::Error::Syscall(crate::target::os::syscall::Error::Fork(
                Error::Default(1),
            )),
        ))),
    }
}
