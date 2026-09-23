use crate::target::architecture::{Architecture, traits::Callable};

pub mod stat;
pub use stat::Stat;

hooking!(FSTAT);

#[inline(always)]
pub fn fstat(fd: isize, stat: *const Stat) -> crate::Result {
    let arch_result = Architecture::syscall2(NUMBER, fd as usize, stat as usize);
    handle_result(arch_result)
}

pub mod ok {

    ample::result!( Ok; "MUnMap Ok"; usize; [
        [0; OK; Default; usize; "Ok"; "All good"],
    ]);

    impl Ok {
        pub fn from_no(no: usize) -> Self {
            Ok::Default(no)
        }
    }
}

pub mod error {
    ample::result!(Error; "FStat error"; usize; [
        [9; EBADF; BadFileDescriptor; usize; "EBADF"; "File descriptor is not a valid open file descriptor"],
        [14; EFAULT; InvalidBuffer; usize; "EFAULT"; "Status buffer is outside the accessible address space"],
        [75; EOVERFLOW; Overflow; usize; "EOVERFLOW"; "File metadata cannot be represented by the stat structure"],
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
    // Err(crate::Error::Default(1))
    match result {
        crate::Result::Ok(crate::Ok::Target(crate::target::Ok::Architecture(
            crate::target::architecture::Ok::X86_64Syscall(
                crate::target::architecture::syscall::Ok::X86_64Syscall2(
                    crate::target::architecture::syscall::syscall2::Ok::Default(m),
                ),
            ),
        ))) => core::result::Result::Ok(crate::Ok::Target(crate::target::Ok::OperatingSystem(
            crate::target::operating_system::Ok::Syscall(crate::target::operating_system::syscall::Ok::FStat(
                crate::target::operating_system::syscall::fstat::Ok::Default(m),
            )),
        ))),
        _ => core::result::Result::Err(crate::Error::Target(crate::target::Error::OperatingSystem(
            crate::target::operating_system::Error::Syscall(crate::target::operating_system::syscall::Error::FStat(
                Error::Default(3),
            )),
        ))),
    }
}
