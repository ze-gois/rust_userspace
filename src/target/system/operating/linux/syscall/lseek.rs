use crate::target::architecture::x86::bit64::syscall;

pub mod whence;
pub use whence::Whence;

#[cfg(target_arch = "x86_64")]
pub const NUMBER: usize = super::number::x86::bit64::LSEEK;

#[inline(always)]
pub fn lseek(file_descriptor: i32, offset: i64, whence: Whence) -> crate::Result {
    let raw_return = unsafe {
        syscall::syscall3(
            NUMBER,
            file_descriptor as usize,
            offset as usize,
            whence.raw() as usize,
        )
    };

    handle_result(raw_return)
}

pub fn handle_result(raw: usize) -> crate::Result {
    match super::Return::new(raw).classify() {
        core::result::Result::Ok(success) => core::result::Result::Ok(
            crate::Ok::Target(crate::target::Ok::OperatingSystem(
                crate::target::system::operating::linux::Ok::Syscall(
                    crate::target::system::operating::linux::syscall::Ok::Lseek(success),
                ),
            )),
        ),
        core::result::Result::Err(failure) => core::result::Result::Err(
            crate::Error::Target(crate::target::Error::OperatingSystem(
                crate::target::system::operating::linux::Error::Syscall(
                    crate::target::system::operating::linux::syscall::Error::Lseek(failure),
                ),
            )),
        ),
    }
}
