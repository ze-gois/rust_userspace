use crate::target::arch::{Arch, traits::Callable};

pub use super::mmap::{Prot, prot};

hooking!(MPROTECT);

#[inline(always)]
pub fn mprotect(addr: *mut u8, len: usize, prot: i32) -> crate::Result {
    let arch_result = Arch::syscall3(NUMBER, addr as usize, len, prot as usize);
    handle_result(arch_result)
}

pub mod ok {
    r#struct!(OkSyscallMUnMap { value: usize });

    result!( Ok; "MUnMap Ok"; usize; [
        [0; OK; Default; usize; "Ok"; "All good"],
    ]);

    impl Ok {
        pub fn from_no(no: usize) -> Self {
            Ok::Default(no)
        }
    }
}

pub mod error {
    result!(Error; "MUnMap error"; usize; [
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
        crate::Result::Ok(crate::Ok::Target(crate::target::Ok::Arch(
            crate::target::arch::Ok::X86_64Syscall(
                crate::target::arch::syscall::Ok::X86_64Syscall3(
                    crate::target::arch::syscall::syscall3::Ok::Default(m),
                ),
            ),
        ))) => core::result::Result::Ok(crate::Ok::Target(crate::target::Ok::Os(
            crate::target::os::Ok::Syscall(crate::target::os::syscall::Ok::MProtect(
                crate::target::os::syscall::mprotect::Ok::Default(m),
            )),
        ))),
        _ => core::result::Result::Err(crate::Error::Target(crate::target::Error::Os(
            crate::target::os::Error::Syscall(crate::target::os::syscall::Error::MProtect(
                Error::Default(3),
            )),
        ))),
    }
}
