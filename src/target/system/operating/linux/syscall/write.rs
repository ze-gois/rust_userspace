use crate::target::architecture::Architecture;

#[cfg(target_arch = "x86_64")]
pub const NUMBER: usize = super::number::x86::bit64::WRITE;

pub fn write(file_descriptor: isize, byte_buffer: *const u8, byte_count: usize) -> crate::Result {
    let syscall_result = Architecture::syscall3(
        NUMBER,
        file_descriptor as usize,
        byte_buffer as usize,
        byte_count as usize,
    );

    handle_result(syscall_result)
}

pub mod ok {

    ample::result!( Ok; "Write Ok"; usize; [
        [0; OK; Default; usize; "Ok"; "All good"],
    ]);

    impl Ok {
        pub fn from_no(no: usize) -> Self {
            Ok::Default(no)
        }
    }
}

pub mod error {
    ample::result!(Error; "Write Error"; usize; [
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
        crate::Result::Ok(crate::Ok::Target(crate::target::Ok::Architecture(
            crate::target::architecture::Ok::Syscall(
                crate::target::architecture::x86::bit64::syscall::Ok::Syscall3(
                    crate::target::architecture::x86::bit64::syscall::syscall3::Ok::Default(m),
                ),
            ),
        ))) => core::result::Result::Ok(crate::Ok::Target(crate::target::Ok::OperatingSystem(
            crate::target::system::operating::linux::Ok::Syscall(crate::target::system::operating::linux::syscall::Ok::Write(
                crate::target::system::operating::linux::syscall::write::Ok::Default(m),
            )),
        ))),
        crate::Result::Err(crate::Error::Target(crate::target::Error::Architecture(
            crate::target::architecture::Error::Syscall(
                crate::target::architecture::x86::bit64::syscall::Error::Syscall3(
                    crate::target::architecture::x86::bit64::syscall::syscall3::Error::Default(raw),
                ),
            ),
        ))) => core::result::Result::Err(crate::Error::Target(
            crate::target::Error::OperatingSystem(
                crate::target::system::operating::linux::Error::Syscall(
                    crate::target::system::operating::linux::syscall::Error::Write(
                        Error::Default(raw),
                    ),
                ),
            ),
        )),
        _ => core::result::Result::Err(crate::Error::Target(
            crate::target::Error::OperatingSystem(
                crate::target::system::operating::linux::Error::Syscall(
                    crate::target::system::operating::linux::syscall::Error::Write(
                        Error::Default(usize::MAX),
                    ),
                ),
            ),
        )),
    }
}
