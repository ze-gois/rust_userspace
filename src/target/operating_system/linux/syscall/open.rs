use crate::target::arch::{Arch, traits::Callable};

pub mod flags;
pub use flags::{AtFlag, Flag};

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
    r#struct!(OkSyscallMUnMap { value: usize });

    result!( Ok; "MUnMap Ok"; usize; [
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
    result!(Error; "MUnMap error"; usize; [
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
        [98; OPENAT;  OPENAT; usize; "OPENAT"; "WAITING"],
        [99; OPENAT4;  OPENAT4; usize; "OPENAT"; "WAITING"],
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
