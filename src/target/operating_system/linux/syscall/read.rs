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

    ample::result!(Error; "Read error"; usize; [
        [11; EAGAIN; WouldBlock; usize; "EAGAIN"; "Nonblocking read would block; EWOULDBLOCK is the socket alias"],
        [9; EBADF; BadFileDescriptor; usize; "EBADF"; "File descriptor is invalid or not open for reading"],
        [14; EFAULT; InvalidBuffer; usize; "EFAULT"; "Buffer is outside the accessible address space"],
        [4; EINTR; Interrupted; usize; "EINTR"; "Read was interrupted by a signal before data was read"],
        [22; EINVAL; InvalidArgument; usize; "EINVAL"; "Object or direct-I/O alignment makes the read invalid"],
        [5; EIO; InputOutput; usize; "EIO"; "Low-level input/output error occurred"],
        [21; EISDIR; IsDirectory; usize; "EISDIR"; "File descriptor refers to a directory"],
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
