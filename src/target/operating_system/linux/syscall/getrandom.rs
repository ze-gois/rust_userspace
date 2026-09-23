use crate::target::architecture::{Architecture, traits::Callable};

pub mod flags;
pub use flags::Flags;

hooking!(GETRANDOM);

pub fn getrandom(byte_buffer: *mut u8, byte_length: usize, flags: u32) -> crate::Result {
    let arch_result = Architecture::syscall3(NUMBER, byte_buffer as usize, byte_length, flags as usize);

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
        [11; EAGAIN; WouldBlock; usize; "EAGAIN"; "Requested entropy is unavailable and nonblocking behavior was requested"],
        [14; EFAULT; InvalidBuffer; usize; "EFAULT"; "Output buffer is outside the accessible address space"],
        [4; EINTR; Interrupted; usize; "EINTR"; "Request was interrupted by a signal"],
        [22; EINVAL; InvalidArgument; usize; "EINVAL"; "Invalid flags were supplied"],
        [38; ENOSYS; NotImplemented; usize; "ENOSYS"; "Kernel does not implement getrandom"],
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
                    crate::target::architecture::syscall::syscall3::Ok::Default(m),
                ),
            ),
        ))) => core::result::Result::Ok(crate::Ok::Target(crate::target::Ok::OperatingSystem(
            crate::target::operating_system::Ok::Syscall(crate::target::operating_system::syscall::Ok::GetRandom(
                crate::target::operating_system::syscall::getrandom::Ok::Default(m),
            )),
        ))),
        _ => core::result::Result::Err(crate::Error::Target(crate::target::Error::OperatingSystem(
            crate::target::operating_system::Error::Syscall(crate::target::operating_system::syscall::Error::GetRandom(
                Error::Default(3),
            )),
        ))),
    }
}
