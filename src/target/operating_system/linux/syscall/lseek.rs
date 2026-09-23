use crate::target::architecture::{Architecture, traits::Callable};

pub mod flags;
pub use flags::Flag;

pub mod whence;
pub use whence::Whence;

hooking!(LSEEK);

#[inline(always)]
pub fn lseek(fd: i32, offset: i64, whence: i32) -> crate::Result {
    let arch_result = Architecture::syscall3(NUMBER, fd as usize, offset as usize, whence as usize);

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
    ample::result!(Error; "LSeek error"; usize; [
        [9; EBADF; BadFileDescriptor; usize; "EBADF"; "File descriptor is not open"],
        [22; EINVAL; InvalidArgument; usize; "EINVAL"; "Whence is invalid or the resulting offset is invalid"],
        [6; ENXIO; NoSuchDeviceOrAddress; usize; "ENXIO"; "SEEK_DATA or SEEK_HOLE request is beyond available data"],
        [75; EOVERFLOW; Overflow; usize; "EOVERFLOW"; "Resulting file offset cannot be represented"],
        [29; ESPIPE; IllegalSeek; usize; "ESPIPE"; "File descriptor refers to a pipe, socket, FIFO, or other non-seekable object"],
        [4096; ERROR; Default; usize; "UNKNOWN"; "Unclassified Linux errno"],
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
            crate::target::architecture::Ok::X86_64Syscall(
                crate::target::architecture::syscall::Ok::X86_64Syscall3(
                    crate::target::architecture::syscall::syscall3::Ok::Default(m),
                ),
            ),
        ))) => core::result::Result::Ok(crate::Ok::Target(crate::target::Ok::OperatingSystem(
            crate::target::operating_system::Ok::Syscall(crate::target::operating_system::syscall::Ok::LSeek(
                crate::target::operating_system::syscall::lseek::Ok::Default(m),
            )),
        ))),
        _ => core::result::Result::Err(crate::Error::Target(crate::target::Error::OperatingSystem(
            crate::target::operating_system::Error::Syscall(crate::target::operating_system::syscall::Error::LSeek(
                Error::Default(3),
            )),
        ))),
    }
}
