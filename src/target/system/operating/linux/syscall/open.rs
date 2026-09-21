use crate::target::architecture::{Architecture, traits::Callable};

pub mod flags;
pub use flags::Flag;

pub mod mode;
pub use mode::Mode;

hooking!(OPEN);

pub fn open(file_pathname: *const u8, flags: i32, mode: i32) -> crate::Result {
    let syscall_result = Architecture::syscall3(
        NUMBER,
        file_pathname as usize,
        flags as usize,
        mode as usize,
    );

    handle_result(syscall_result)
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
            crate::target::system::operating::linux::Ok::Syscall(crate::target::system::operating::linux::syscall::Ok::Open(
                crate::target::system::operating::linux::syscall::open::Ok::Default(m),
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
                    crate::target::system::operating::linux::syscall::Error::Open(
                        Error::Default(raw),
                    ),
                ),
            ),
        )),
        _ => core::result::Result::Err(crate::Error::Target(
            crate::target::Error::OperatingSystem(
                crate::target::system::operating::linux::Error::Syscall(
                    crate::target::system::operating::linux::syscall::Error::Open(
                        Error::Default(usize::MAX),
                    ),
                ),
            ),
        )),
    }
}
