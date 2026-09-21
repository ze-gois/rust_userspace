use crate::target::architecture::x86::bit64::syscall;

#[cfg(target_arch = "x86_64")]
pub const NUMBER: usize = super::number::x86::bit64::FORK;

/// Create a new process using Linux's `fork(2)` system call.
///
/// The returned value is `0` in the child process and the child PID in the
/// parent process. On failure, the result contains the kernel error value.
#[inline(always)]
pub fn fork() -> crate::Result {
    let raw_return = unsafe { syscall::syscall0(NUMBER) };
    handle_result(raw_return)
}

pub fn handle_result(raw: usize) -> crate::Result {
    match super::Return::new(raw).classify() {
        core::result::Result::Ok(success) => core::result::Result::Ok(
            crate::Ok::Target(crate::target::Ok::OperatingSystem(
                crate::target::system::operating::linux::Ok::Syscall(
                    crate::target::system::operating::linux::syscall::Ok::Fork(success),
                ),
            )),
        ),
        core::result::Result::Err(failure) => core::result::Result::Err(
            crate::Error::Target(crate::target::Error::OperatingSystem(
                crate::target::system::operating::linux::Error::Syscall(
                    crate::target::system::operating::linux::syscall::Error::Fork(failure),
                ),
            )),
        ),
    }
}
