pub mod ok {
    ample::result!( Ok; "Linux syscall Ok"; usize; [
        [0; OK;         Default; usize; "Ok"; "All good"],
        [2; ERR_CLOSE;  Close;    super::super::Success;        "close";     "E_CLOSE" ],
        [3; ERR_LSEEK;  Lseek;    super::super::Success;        "lseek";     "E_LSEEK"],
        [4; ERR_MMAP;  Mmap;     super::super::Success;          "mmap";      "E_MMAP"],
        [5; ERR_MPROTECT;  Mprotect; super::super::Success;  "mprotect";  "E_MPROTECT"],
        [6; ERR_MUNMAP;  Munmap;   super::super::Success;      "munmap";    "E_MUNMAP"],
        [7; ERR_OPEN;  Open;     super::super::Success;          "open";      "E_OPEN"],
        [14; ERR_OPENAT; Openat; super::super::Success; "openat"; "E_OPENAT"],
        [8; ERR_READ;  Read;     super::super::Success;          "read";      "E_READ"],
        [9; ERR_WRITE;  Write;    super::super::Success;        "write";     "E_WRITE" ],
        [10; ERR_FSTAT; Fstat;    super::super::Success;        "fstat";     "E_FSTAT"],
        [11; ERR_FORK;  Fork;     super::super::Success;         "fork";      "E_FORK" ],
        [12; ERR_EXECVE; Execve;    super::super::Success;       "execve";    "E_EXECVE"],
        [13; ERR_GETRANDOM; Getrandom; super::super::Success; "getrandom"; "E_GETRANDOM"]
    ]);

    impl Ok {
        pub fn from_no(no: usize) -> Self {
            Ok::Default(no)
        }
    }
}

pub mod error {
    ample::result!(Error; "Linux syscall Error"; usize; [
        [1; ERROR;      Default; usize; "Error"; "Something wicked this way comes"],
        [2; ERR_CLOSE;  Close;    super::super::Failure;        "close";     "E_CLOSE" ],
        [3; ERR_LSEEK;  Lseek;    super::super::Failure;        "lseek";     "E_LSEEK"],
        [4; ERR_MMAP;  Mmap;     super::super::Failure;          "mmap";      "E_MMAP"],
        [5; ERR_MPROTECT;  Mprotect; super::super::Failure;  "mprotect";  "E_MPROTECT"],
        [6; ERR_MUNMAP;  Munmap;   super::super::Failure;      "munmap";    "E_MUNMAP"],
        [7; ERR_OPEN;  Open;     super::super::Failure;          "open";      "E_OPEN"],
        [14; ERR_OPENAT; Openat; super::super::Failure; "openat"; "E_OPENAT"],
        [8; ERR_READ;  Read;     super::super::Failure;          "read";      "E_READ"],
        [9; ERR_WRITE;  Write;    super::super::Failure;        "write";     "E_WRITE"],
        [10; ERR_FSTAT; Fstat;    super::super::Failure;        "fstat";     "E_FSTAT"],
        [11; ERR_FORK;  Fork;     super::super::Failure;         "fork";      "E_FORK" ],
        [12; ERR_EXECVE;  Execve;    super::super::Failure;       "execve";    "E_EXECVE" ],
        [13; ERR_GETRANDOM; Getrandom; super::super::Failure; "getrandom"; "E_GETRANDOM"]
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
