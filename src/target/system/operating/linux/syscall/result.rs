pub mod ok {
    ample::result!( Ok; "Linux syscall Ok"; usize; [
        [0; OK;         Default; usize; "Ok"; "All good"],
        [2; ERR_CLOSE;  Close;    super::super::close::Ok;        "close";     "E_CLOSE" ],
        [3; ERR_LSEEK;  Lseek;    super::super::lseek::Ok;        "lseek";     "E_LSEEK"],
        [4; ERR_MMAP;  Mmap;     super::super::mmap::Ok;          "mmap";      "E_MMAP"],
        [5; ERR_MPROTECT;  Mprotect; super::super::mprotect::Ok;  "mprotect";  "E_MPROTECT"],
        [6; ERR_MUNMAP;  Munmap;   super::super::munmap::Ok;      "munmap";    "E_MUNMAP"],
        [7; ERR_OPEN;  Open;     super::super::open::Ok;          "open";      "E_OPEN"],
        [14; ERR_OPENAT; Openat; super::super::openat::Ok; "openat"; "E_OPENAT"],
        [8; ERR_READ;  Read;     super::super::read::Ok;          "read";      "E_READ"],
        [9; ERR_WRITE;  Write;    super::super::write::Ok;        "write";     "E_WRITE" ],
        [10; ERR_FSTAT; Fstat;    super::super::fstat::Ok;        "fstat";     "E_FSTAT"],
        [11; ERR_FORK;  Fork;     super::super::fork::Ok;         "fork";      "E_FORK" ],
        [12; ERR_EXECVE; Execve;    super::super::execve::Ok;       "execve";    "E_EXECVE"],
        [13; ERR_GETRANDOM; Getrandom; super::super::getrandom::Ok; "getrandom"; "E_GETRANDOM"]
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
        [2; ERR_CLOSE;  Close;    super::super::close::Error;        "close";     "E_CLOSE" ],
        [3; ERR_LSEEK;  Lseek;    super::super::lseek::Error;        "lseek";     "E_LSEEK"],
        [4; ERR_MMAP;  Mmap;     super::super::mmap::Error;          "mmap";      "E_MMAP"],
        [5; ERR_MPROTECT;  Mprotect; super::super::mprotect::Error;  "mprotect";  "E_MPROTECT"],
        [6; ERR_MUNMAP;  Munmap;   super::super::munmap::Error;      "munmap";    "E_MUNMAP"],
        [7; ERR_OPEN;  Open;     super::super::open::Error;          "open";      "E_OPEN"],
        [14; ERR_OPENAT; Openat; super::super::openat::Error; "openat"; "E_OPENAT"],
        [8; ERR_READ;  Read;     super::super::read::Error;          "read";      "E_READ"],
        [9; ERR_WRITE;  Write;    super::super::write::Error;        "write";     "E_WRITE"],
        [10; ERR_FSTAT; Fstat;    super::super::fstat::Error;        "fstat";     "E_FSTAT"],
        [11; ERR_FORK;  Fork;     super::super::fork::Error;         "fork";      "E_FORK" ],
        [12; ERR_EXECVE;  Execve;    super::super::execve::Error;       "execve";    "E_EXECVE" ],
        [13; ERR_GETRANDOM; Getrandom; super::super::getrandom::Error; "getrandom"; "E_GETRANDOM"]
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
