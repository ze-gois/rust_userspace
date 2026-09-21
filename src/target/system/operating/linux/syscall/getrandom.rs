use crate::target::architecture::x86::bit64::syscall;

#[cfg(target_arch = "x86_64")]
pub const NUMBER: usize = super::number::x86::bit64::GETRANDOM;

pub fn getrandom(byte_buffer: *mut u8, byte_length: usize, flags: u32) -> crate::Result {
    let raw_return = syscall::syscall3(NUMBER, byte_buffer as usize, byte_length, flags as usize);

    handle_result(raw_return)
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

pub fn handle_result(raw: usize) -> crate::Result {
    let result = super::Return::new(raw);

    if result.is_error() {
        core::result::Result::Err(crate::Error::Target(
            crate::target::Error::OperatingSystem(
                crate::target::system::operating::linux::Error::Syscall(
                    crate::target::system::operating::linux::syscall::Error::Getrandom(
                        Error::Default(result.raw()),
                    ),
                ),
            ),
        ))
    } else {
        core::result::Result::Ok(crate::Ok::Target(
            crate::target::Ok::OperatingSystem(
                crate::target::system::operating::linux::Ok::Syscall(
                    crate::target::system::operating::linux::syscall::Ok::Getrandom(
                        Ok::Default(result.raw()),
                    ),
                ),
            ),
        ))
    }
}
