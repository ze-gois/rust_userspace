use crate::target::arch::{Arch, traits::Callable};

pub use super::mmap::{Prot, prot};

hooking!(MPROTECT);

#[inline(always)]
pub fn mprotect(addr: *mut u8, len: usize, prot: i32) -> crate::Result {
    let arch_result = Arch::syscall3(NUMBER, addr as usize, len, prot as usize);
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
    const UNKNOWN_ERROR_DISCRIMINANT: usize = usize::MAX;

    ample::result!(Error; "MProtect error"; usize; [
        [13; EACCES; PermissionDenied; usize; "EACCES"; "Requested protection conflicts with access allowed by the underlying memory object"],
        [22; EINVAL; InvalidArgument; usize; "EINVAL"; "Address, alignment, or protection flags are invalid"],
        [12; ENOMEM; OutOfMemory; usize; "ENOMEM"; "Address range is unmapped or kernel mapping resources are unavailable"],
        [UNKNOWN_ERROR_DISCRIMINANT; ERROR; Default; usize; "UNKNOWN"; "Unclassified Linux errno"],
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
