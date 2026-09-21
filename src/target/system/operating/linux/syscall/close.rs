use crate::target::architecture::{Architecture, traits::Callable};

hooking!(CLOSE);

#[inline(always)]
pub fn close(fd: isize) -> crate::Result {
    let arch_result = Architecture::syscall1(NUMBER, fd as usize);
    handle_result(arch_result)
}

pub mod ok {

    ample::result!( Ok; "Close Ok"; usize; [
        [0; OK; Default; usize; "Ok"; "All good"],
    ]);

    impl Ok {
        pub fn from_no(no: usize) -> Self {
            Ok::Default(no)
        }
    }
}

pub mod error {
    ample::result!(Error; "Close Error"; usize; [
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
                crate::target::architecture::syscall::Ok::Syscall1(
                    crate::target::architecture::syscall::syscall1::Ok::Default(m),
                ),
            ),
        ))) => core::result::Result::Ok(crate::Ok::Target(crate::target::Ok::OperatingSystem(
            crate::target::system::operating::linux::Ok::Syscall(crate::target::system::operating::linux::syscall::Ok::Close(
                crate::target::system::operating::linux::syscall::close::Ok::Default(m),
            )),
        ))),
        _ => core::result::Result::Err(crate::Error::Target(crate::target::Error::OperatingSystem(
            crate::target::system::operating::linux::Error::Syscall(crate::target::system::operating::linux::syscall::Error::Close(
                Error::Default(3),
            )),
        ))),
    }
}
