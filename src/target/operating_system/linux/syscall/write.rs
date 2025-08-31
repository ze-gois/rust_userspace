use crate::target::arch::{Arch, traits::Callable};

hooking!(WRITE);

pub fn write(file_descriptor: isize, byte_buffer: *const u8, byte_count: usize) -> crate::Result {
    let syscall_result = Arch::syscall3(
        NUMBER,
        file_descriptor as usize,
        byte_buffer as usize,
        byte_count as usize,
    );

    handle_result(syscall_result)
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
        [0;  ERROR2;   Default;           usize; "Error"; "Something wicked this way comes"],
        [1;  ERROR;   Error;             usize; "Error"; "Something wicked this way comes"],
        [9;  EBADF;   BadFileDescriptor; usize;   "EBADF";     "Bad file descriptor"],
        [14; EFAULT;  InvalidBuffer;     usize;  "EFAULT";    "Invalid buffer pointer"],
        [27; EFBIG;   BufferTooLarge;    usize;   "EFBIG";     "Buffer too large"],
        [4;  EINTR;   Interrupted;       usize;   "EINTR";     "System call was interrupted"],
        [5;  EIO;     IOError;           usize;     "EIO";       "Input/output error"],
        [28; ENOSPC;  NoSpaceLeft;       usize;  "ENOSPC";    "No space left on device"],
        [32; EPIPE;   BrokenPipe;        usize;   "EPIPE";     "Broken pipe"],
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
            crate::target::os::Ok::Syscall(crate::target::os::syscall::Ok::Write(
                crate::target::os::syscall::write::Ok::Default(m),
            )),
        ))),
        _ => core::result::Result::Err(crate::Error::Target(crate::target::Error::Os(
            crate::target::os::Error::Syscall(crate::target::os::syscall::Error::Write(
                Error::Default(3),
            )),
        ))),
    }
}
