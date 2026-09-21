use crate::target::architecture::x86::bit64::syscall;

pub mod flags;
pub use flags::Flag;

pub mod mode;
pub use mode::Mode;

#[cfg(target_arch = "x86_64")]
pub const NUMBER: usize = super::number::x86::bit64::OPEN;

/// # Safety
///
/// `file_pathname` must point to a readable NUL-terminated pathname.
pub unsafe fn open(file_pathname: *const u8, flags: i32, mode: i32) -> crate::Result {
    let raw_return = unsafe {
        syscall::syscall3(
            NUMBER,
            file_pathname as usize,
            flags as usize,
            mode as usize,
        )
    };

    handle_result(raw_return)
}

pub mod ok {

    ample::result!( Ok; "Open Ok"; usize; [
        [0; OK; Default; usize; "Ok"; "All good"],
    ]);

    impl Ok {
        pub fn from_no(no: usize) -> Self {
            Ok::Default(no)
        }
    }
}

pub mod error {
    ample::result!(Error; "Open Error"; usize; [
        [1;  ERROR;         Default;              usize;  "Error"; "Something wicked this way comes"],
        [2;  ENOENT;        FileNotFound;       usize;  "ENOENT";       "File not found"],
        [13; EACCES;        PermissionDenied;   usize;  "EACCES";       "Permission denied"],
        [22; EINVAL;        InvalidPath;        usize;  "EINVAL";       "Invalid path"],
        [20; ENOTDIR;       DirectoryNotFound;  usize;  "ENOTDIR";      "Directory not found"],
        [40; ELOOP;         TooManySymlinks;    usize;  "ELOOP";        "Too many levels of symbolic links"],
        [36; ENAMETOOLONG;  PathnameTooLong;    usize;  "ENAMETOOLONG"; "Pathname too long"],
        [17; EEXIST;        FileExists;         usize;  "EEXIST";       "File exists"],
        [24; EMFILE;        TooManyOpenFiles;   usize;  "EMFILE";       "Too many open files"],
        [28; ENOSPC;        NoSpace;            usize;  "ENOSPC";       "No space left on device"],
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

pub fn handle_result(raw: usize) -> crate::Result {
    let result = super::Return::new(raw);

    if result.is_error() {
        core::result::Result::Err(crate::Error::Target(
            crate::target::Error::OperatingSystem(
                crate::target::system::operating::linux::Error::Syscall(
                    crate::target::system::operating::linux::syscall::Error::Open(
                        Error::Default(result.raw()),
                    ),
                ),
            ),
        ))
    } else {
        core::result::Result::Ok(crate::Ok::Target(
            crate::target::Ok::OperatingSystem(
                crate::target::system::operating::linux::Ok::Syscall(
                    crate::target::system::operating::linux::syscall::Ok::Open(
                        Ok::Default(result.raw()),
                    ),
                ),
            ),
        ))
    }
}
