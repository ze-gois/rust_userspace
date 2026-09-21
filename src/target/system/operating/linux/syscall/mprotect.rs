use crate::target::architecture::{Architecture, traits::Callable};

pub use super::mmap::{Prot, prot};

hooking!(MPROTECT);

#[inline(always)]
pub fn mprotect(addr: *mut u8, len: usize, prot: i32) -> crate::Result {
    let arch_result = Architecture::syscall3(NUMBER, addr as usize, len, prot as usize);
    handle_result(arch_result)
}

pub mod ok {

    ample::result!( Ok; "Mprotect Ok"; usize; [
        [0; OK; Default; usize; "Ok"; "All good"],
    ]);

    impl Ok {
        pub fn from_no(no: usize) -> Self {
            Ok::Default(no)
        }
    }
}

pub mod error {
    ample::result!(Error; "Mprotect Error"; usize; [
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
                crate::target::architecture::syscall::Ok::Syscall3(
                    crate::target::architecture::syscall::syscall3::Ok::Default(m),
                ),
            ),
        ))) => core::result::Result::Ok(crate::Ok::Target(crate::target::Ok::OperatingSystem(
            crate::target::os::Ok::Syscall(crate::target::os::syscall::Ok::MProtect(
                crate::target::os::syscall::mprotect::Ok::Default(m),
            )),
        ))),
        _ => core::result::Result::Err(crate::Error::Target(crate::target::Error::OperatingSystem(
            crate::target::os::Error::Syscall(crate::target::os::syscall::Error::MProtect(
                Error::Default(3),
            )),
        ))),
    }
}
