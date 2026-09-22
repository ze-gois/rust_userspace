use crate::target::arch::{Arch, traits::Callable};

pub mod flags;
pub use flags::Flag;

pub mod at_flags;
pub use at_flags::AtFlag;

pub mod mode;
pub use mode::Mode;

hooking!(OPEN);

pub fn open(file_pathname: *const u8, flags: i32, mode: i32) -> crate::Result {
    let syscall_result = Arch::syscall3(
        NUMBER,
        file_pathname as usize,
        flags as usize,
        mode as usize,
    );

    handle_result(syscall_result)
}

pub mod ok {

    ample::result!( Ok; "MUnMap Ok"; usize; [
        [0; OK; Default; usize; "Ok"; "All good"],
        [98; OPENAT;  OPENAT; usize; "OPENAT"; "WAITING"],
        [99; OPENAT4;  OPENAT4; usize; "OPENAT"; "WAITING"],
    ]);

    impl Ok {
        pub fn from_no(no: usize) -> Self {
            Ok::Default(no)
        }
    }
}

pub mod error {
    ample::result!(Error; "Open error"; usize; [
        [13; EACCES; PermissionDenied; usize; "EACCES"; "Requested access or pathname search permission is denied"],
        [16; EBUSY; Busy; usize; "EBUSY"; "Exclusive open requested for a block device that is in use"],
        [122; EDQUOT; QuotaExceeded; usize; "EDQUOT"; "Disk block or inode quota is exhausted"],
        [17; EEXIST; AlreadyExists; usize; "EEXIST"; "Path exists while O_CREAT and O_EXCL were requested"],
        [14; EFAULT; InvalidPathPointer; usize; "EFAULT"; "Path points outside the accessible address space"],
        [27; EFBIG; FileTooLarge; usize; "EFBIG"; "File is too large to be opened on this interface"],
        [4; EINTR; Interrupted; usize; "EINTR"; "Open was interrupted while waiting on a slow device"],
        [22; EINVAL; InvalidArgument; usize; "EINVAL"; "Flags or pathname component are invalid"],
        [21; EISDIR; IsDirectory; usize; "EISDIR"; "Path is a directory and write access was requested"],
        [40; ELOOP; TooManySymbolicLinks; usize; "ELOOP"; "Too many symbolic links were encountered"],
        [24; EMFILE; ProcessFileDescriptorLimit; usize; "EMFILE"; "Process file-descriptor limit was reached"],
        [36; ENAMETOOLONG; NameTooLong; usize; "ENAMETOOLONG"; "Pathname is too long"],
        [23; ENFILE; SystemFileDescriptorLimit; usize; "ENFILE"; "System-wide open-file limit was reached"],
        [19; ENODEV; NoSuchDevice; usize; "ENODEV"; "Device special file has no corresponding device"],
        [2; ENOENT; NotFound; usize; "ENOENT"; "Path or one of its components does not exist"],
        [12; ENOMEM; OutOfMemory; usize; "ENOMEM"; "Insufficient kernel memory was available"],
        [28; ENOSPC; NoSpaceLeft; usize; "ENOSPC"; "Device has no space for the file to be created"],
        [20; ENOTDIR; NotDirectory; usize; "ENOTDIR"; "A pathname component that must be a directory is not one"],
        [6; ENXIO; NoSuchDeviceOrAddress; usize; "ENXIO"; "Device, FIFO, or UNIX socket condition prevents opening"],
        [95; EOPNOTSUPP; OperationNotSupported; usize; "EOPNOTSUPP"; "Filesystem does not support the requested operation"],
        [75; EOVERFLOW; Overflow; usize; "EOVERFLOW"; "File metadata cannot be represented by this interface"],
        [1; EPERM; OperationNotPermitted; usize; "EPERM"; "Operation is not permitted, for example because of O_NOATIME or a file seal"],
        [30; EROFS; ReadOnlyFileSystem; usize; "EROFS"; "Write access was requested on a read-only filesystem"],
        [26; ETXTBSY; TextFileBusy; usize; "ETXTBSY"; "Executable, swap, or kernel-used file is busy"],
        [11; EWOULDBLOCK; WouldBlock; usize; "EWOULDBLOCK"; "Nonblocking open conflicts with an incompatible lease"],
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
            crate::target::os::Ok::Syscall(crate::target::os::syscall::Ok::Open(
                crate::target::os::syscall::open::Ok::Default(m),
            )),
        ))),
        _ => core::result::Result::Err(crate::Error::Target(crate::target::Error::Os(
            crate::target::os::Error::Syscall(crate::target::os::syscall::Error::Open(
                Error::Default(3),
            )),
        ))),
    }
}
