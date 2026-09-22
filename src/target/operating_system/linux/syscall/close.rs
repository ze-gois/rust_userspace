use crate::target::arch::{Arch, traits::Callable};

hooking!(CLOSE);

#[inline(always)]
pub fn close(fd: isize) -> crate::Result {
    let arch_result = Arch::syscall1(NUMBER, fd as usize);
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
    ample::result!(Error; "Close error"; usize; [
        [9; EBADF; BadFileDescriptor; usize; "EBADF"; "File descriptor is not a valid open file descriptor"],
        [4; EINTR; Interrupted; usize; "EINTR"; "Close was interrupted by a signal"],
        [5; EIO; InputOutput; usize; "EIO"; "Input/output error occurred"],
        [28; ENOSPC; NoSpaceLeft; usize; "ENOSPC"; "No space left on the device; may be reported late by close"],
        [122; EDQUOT; QuotaExceeded; usize; "EDQUOT"; "Disk quota exceeded; may be reported late by close"],
        [usize::MAX; ERROR; Default; usize; "UNKNOWN"; "Unclassified Linux errno"],
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
                crate::target::arch::syscall::Ok::X86_64Syscall1(
                    crate::target::arch::syscall::syscall1::Ok::Default(m),
                ),
            ),
        ))) => core::result::Result::Ok(crate::Ok::Target(crate::target::Ok::Os(
            crate::target::os::Ok::Syscall(crate::target::os::syscall::Ok::Close(
                crate::target::os::syscall::close::Ok::Default(m),
            )),
        ))),
        _ => core::result::Result::Err(crate::Error::Target(crate::target::Error::Os(
            crate::target::os::Error::Syscall(crate::target::os::syscall::Error::Close(
                Error::Default(3),
            )),
        ))),
    }
}
