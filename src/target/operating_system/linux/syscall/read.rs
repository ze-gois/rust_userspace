use crate::target::arch::{Arch, traits::Callable};

hooking!(READ);

pub fn read(file_descriptor: isize, byte_buffer: *const u8, byte_length: usize) -> crate::Result {
    let arch_result = Arch::syscall3(
        NUMBER,
        file_descriptor as usize,
        byte_buffer as usize,
        byte_length as usize,
    );

    handle_result(arch_result)
}

pub mod ok {
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
        [1; ERROR;          Default;            usize; "Error"; "Something wicked this way comes"],
        [4;   EINTR;        Interrupted;      usize;            "EINTR";     "System call was interrupted"],
        [5;   EIO;          IOError;          usize;              "EIO";       "Input/output error"],
        [9;   EBADF;        BadFileDescriptor;usize;            "EBADF";     "Bad file descriptor"],
        [14;  EFAULT;       InvalidBuffer;    usize;           "EFAULT";    "Invalid buffer pointer"],
        [22;  EINVAL;       InvalidCount;     usize;           "EINVAL";    "Invalid count"],
        [21;  EISDIR;       IsDirectory;      usize;           "EISDIR";    "Is a directory"],
        [13;  EACCES;       NotReadable;      usize;           "EACCES";    "File not open for reading"],
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
            crate::target::os::Ok::Syscall(crate::target::os::syscall::Ok::Read(
                crate::target::os::syscall::read::Ok::Default(m),
            )),
        ))),
        _ => core::result::Result::Err(crate::Error::Target(crate::target::Error::Os(
            crate::target::os::Error::Syscall(crate::target::os::syscall::Error::Read(
                Error::Default(3),
            )),
        ))),
    }
}
