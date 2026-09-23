use crate::target::architecture::{Architecture, traits::Callable};

pub mod flags;
pub mod prot;
pub mod protection;
pub mod sharing;

pub use flags::Flag;
pub use prot::Prot;
pub use protection::Protection;
pub use sharing::Sharing;

hooking!(MMAP);

#[inline(always)]
#[rustfmt::skip]
pub fn mmap(addr: *mut u8, length: usize, prot: i32, flags: i32, fd: i32, offset: i64) -> crate::Result {
    let arch_result = Architecture::syscall6(
        NUMBER,
        addr as usize,
        length,
        prot as usize,
        flags as usize,
        fd as usize,
        offset as usize,
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
    ample::result!(Error; "MMap error"; usize; [
        [13; EACCES; PermissionDenied; usize; "EACCES"; "Mapping permissions conflict with file access or file type"],
        [11; EAGAIN; TryAgain; usize; "EAGAIN"; "File is locked or too much memory would be locked"],
        [9; EBADF; BadFileDescriptor; usize; "EBADF"; "File descriptor is invalid for a file-backed mapping"],
        [17; EEXIST; AlreadyExists; usize; "EEXIST"; "MAP_FIXED_NOREPLACE range conflicts with an existing mapping"],
        [22; EINVAL; InvalidArgument; usize; "EINVAL"; "Address, length, offset, or mapping flags are invalid"],
        [23; ENFILE; SystemFileDescriptorLimit; usize; "ENFILE"; "System-wide open-file limit was reached"],
        [19; ENODEV; NoSuchDevice; usize; "ENODEV"; "Underlying filesystem does not support memory mapping"],
        [12; ENOMEM; OutOfMemory; usize; "ENOMEM"; "Virtual memory or mapping resources are unavailable"],
        [75; EOVERFLOW; Overflow; usize; "EOVERFLOW"; "Length and offset page counts overflow the supported range"],
        [1; EPERM; OperationNotPermitted; usize; "EPERM"; "Executable mapping, file seal, or huge-page policy forbids the mapping"],
        [26; ETXTBSY; TextFileBusy; usize; "ETXTBSY"; "MAP_DENYWRITE conflicts with a file open for writing"],
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
                crate::target::architecture::syscall::Ok::X86_64Syscall6(
                    crate::target::architecture::syscall::syscall6::Ok::Default(m),
                ),
            ),
        ))) => core::result::Result::Ok(crate::Ok::Target(crate::target::Ok::OperatingSystem(
            crate::target::operating_system::Ok::Syscall(crate::target::operating_system::syscall::Ok::MMap(
                crate::target::operating_system::syscall::mmap::Ok::Default(m),
            )),
        ))),
        crate::Result::Err(crate::Error::Target(crate::target::Error::Architecture(
            crate::target::architecture::Error::X86_64Syscall(
                crate::target::architecture::syscall::Error::X86_64Syscall6(
                    crate::target::architecture::syscall::syscall6::Error::Default(errno),
                ),
            ),
        ))) => core::result::Result::Err(crate::Error::Target(crate::target::Error::OperatingSystem(
            crate::target::operating_system::Error::Syscall(crate::target::operating_system::syscall::Error::MMap(
                Error::Default(errno),
            )),
        ))),
        _ => core::result::Result::Err(crate::Error::Target(crate::target::Error::OperatingSystem(
            crate::target::operating_system::Error::Syscall(crate::target::operating_system::syscall::Error::MMap(
                Error::Default(3),
            )),
        ))),
    }
}
