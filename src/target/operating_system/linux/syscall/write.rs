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
    ample::result!(Error; "Write error"; usize; [
        [11; EAGAIN; WouldBlock; usize; "EAGAIN"; "Nonblocking write would block; EWOULDBLOCK is the socket alias"],
        [9; EBADF; BadFileDescriptor; usize; "EBADF"; "File descriptor is invalid or not open for writing"],
        [89; EDESTADDRREQ; DestinationAddressRequired; usize; "EDESTADDRREQ"; "Datagram socket has no peer address"],
        [122; EDQUOT; QuotaExceeded; usize; "EDQUOT"; "User disk quota is exhausted"],
        [14; EFAULT; InvalidBuffer; usize; "EFAULT"; "Buffer is outside the accessible address space"],
        [27; EFBIG; FileTooLarge; usize; "EFBIG"; "Write would exceed the maximum file size or file-size limit"],
        [4; EINTR; Interrupted; usize; "EINTR"; "Write was interrupted by a signal before data was written"],
        [22; EINVAL; InvalidArgument; usize; "EINVAL"; "Object or direct-I/O alignment makes the write invalid"],
        [5; EIO; InputOutput; usize; "EIO"; "Low-level input/output or write-back error occurred"],
        [28; ENOSPC; NoSpaceLeft; usize; "ENOSPC"; "Device has no space for the data"],
        [1; EPERM; OperationNotPermitted; usize; "EPERM"; "Operation was prevented, for example by a file seal"],
        [32; EPIPE; BrokenPipe; usize; "EPIPE"; "Pipe or socket reading end is closed"],
        [usize::MAX; UNKNOWN; Unknown; usize; "UNKNOWN"; "Unclassified Linux errno"],
    ]);

    impl Error {
        pub fn from_no(no: usize) -> Self {
            Error::Unknown(no)
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
