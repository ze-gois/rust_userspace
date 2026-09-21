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

    ample::result!( Ok; "Fstat Ok"; usize; [
        [0; OK; Default; usize; "Ok"; "All good"],
    ]);

    impl Ok {
        pub fn from_no(no: usize) -> Self {
            Ok::Default(no)
        }
    }
}

pub mod error {
    ample::result!(Error; "Fstat Error"; usize; [
        [1; ERROR; Default; usize; "Error"; "Something wicked this way comes"],
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
            crate::target::architecture::Ok::Syscall(
                crate::target::architecture::syscall::Ok::Syscall2(
                    crate::target::architecture::syscall::syscall2::Ok::Default(m),
                ),
            ),
        ))) => core::result::Result::Ok(crate::Ok::Target(crate::target::Ok::OperatingSystem(
            crate::target::system::operating::linux::Ok::Syscall(crate::target::system::operating::linux::syscall::Ok::Fstat(
                crate::target::system::operating::linux::syscall::fstat::Ok::Default(m),
            )),
        ))),
        _ => core::result::Result::Err(crate::Error::Target(crate::target::Error::OperatingSystem(
            crate::target::system::operating::linux::Error::Syscall(crate::target::system::operating::linux::syscall::Error::Fstat(
                Error::Default(3),
            )),
        ))),
    }
}
