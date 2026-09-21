use crate::target::architecture::x86::bit64::syscall;

#[cfg(target_arch = "x86_64")]
pub const NUMBER: usize = super::number::x86::bit64::GETRANDOM;

pub fn getrandom(byte_buffer: *mut u8, byte_length: usize, flags: u32) -> crate::Result {
    let arch_result = syscall::syscall3(NUMBER, byte_buffer as usize, byte_length, flags as usize);

    handle_result(arch_result)
}

pub mod ok {
    ample::result!( Ok; "Getrandom Ok"; usize; [
        [0; OK; Default; usize; "Ok"; "All good"],
    ]);

    impl Ok {
        pub fn from_no(no: usize) -> Self {
            Ok::Default(no)
        }
    }
}

pub mod error {
    ample::result!(Error; "Getrandom error"; usize; [
        [1; ERROR; Default; usize; "Error"; "Something wicked this way comes"],
        [4; EINTR; Interrupted; usize; "EINTR"; "System call was interrupted"],
        [14; EFAULT; InvalidBuffer; usize; "EFAULT"; "Invalid buffer pointer"],
        [22; EINVAL; InvalidFlags; usize; "EINVAL"; "Invalid flags"],
        [11; EAGAIN; WouldBlock; usize; "EAGAIN"; "Randomness is not ready"],
        [13; EPERM; PermissionDenied; usize; "EPERM"; "Operation not permitted"],
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
            crate::target::architecture::Ok::Syscall(
                crate::target::architecture::x86::bit64::syscall::Ok::Syscall3(
                    crate::target::architecture::x86::bit64::syscall::syscall3::Ok::Default(m),
                ),
            ),
        ))) => core::result::Result::Ok(crate::Ok::Target(crate::target::Ok::OperatingSystem(
            crate::target::system::operating::linux::Ok::Syscall(crate::target::system::operating::linux::syscall::Ok::Getrandom(
                crate::target::system::operating::linux::syscall::getrandom::Ok::Default(m),
            )),
        ))),
        _ => core::result::Result::Err(crate::Error::Target(crate::target::Error::OperatingSystem(
            crate::target::system::operating::linux::Error::Syscall(crate::target::system::operating::linux::syscall::Error::Getrandom(
                Error::Default(3),
            )),
        ))),
    }
}
