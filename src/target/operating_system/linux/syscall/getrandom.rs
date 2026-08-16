use crate::target::arch::{Arch, traits::Callable};

hooking!(GETRANDOM);

pub fn getrandom(byte_buffer: *mut u8, byte_length: usize, flags: u32) -> crate::Result {
    let arch_result = Arch::syscall3(NUMBER, byte_buffer as usize, byte_length, flags as usize);

    handle_result(arch_result)
}

pub mod ok {
    ample::result!( Ok; "GetRandom Ok"; usize; [
        [0; OK; Default; usize; "Ok"; "All good"],
    ]);

    impl Ok {
        pub fn from_no(no: usize) -> Self {
            Ok::Default(no)
        }
    }
}

pub mod error {
    ample::result!(Error; "GetRandom error"; usize; [
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
        crate::Result::Ok(crate::Ok::Target(crate::target::Ok::Arch(
            crate::target::arch::Ok::X86_64Syscall(
                crate::target::arch::syscall::Ok::X86_64Syscall3(
                    crate::target::arch::syscall::syscall3::Ok::Default(m),
                ),
            ),
        ))) => core::result::Result::Ok(crate::Ok::Target(crate::target::Ok::Os(
            crate::target::os::Ok::Syscall(crate::target::os::syscall::Ok::GetRandom(
                crate::target::os::syscall::getrandom::Ok::Default(m),
            )),
        ))),
        _ => core::result::Result::Err(crate::Error::Target(crate::target::Error::Os(
            crate::target::os::Error::Syscall(crate::target::os::syscall::Error::GetRandom(
                Error::Default(3),
            )),
        ))),
    }
}
