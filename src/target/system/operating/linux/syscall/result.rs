pub mod ok {
    ample::result!(
        Ok;
        "Linux system call success";
        usize;
        [
            [0;  UNKNOWN_SUCCESS;   Unknown;   super::super::Success; "unknown";   "Success from an unclassified Linux system call"],
            [2;  CLOSE_SUCCESS;     Close;     super::super::Success; "close";     "Successful close system call"],
            [3;  LSEEK_SUCCESS;     Lseek;     super::super::Success; "lseek";     "Successful lseek system call"],
            [4;  MMAP_SUCCESS;      Mmap;      super::super::Success; "mmap";      "Successful mmap system call"],
            [5;  MPROTECT_SUCCESS;  Mprotect;  super::super::Success; "mprotect";  "Successful mprotect system call"],
            [6;  MUNMAP_SUCCESS;    Munmap;    super::super::Success; "munmap";    "Successful munmap system call"],
            [7;  OPEN_SUCCESS;      Open;      super::super::Success; "open";      "Successful open system call"],
            [8;  READ_SUCCESS;      Read;      super::super::Success; "read";      "Successful read system call"],
            [9;  WRITE_SUCCESS;     Write;     super::super::Success; "write";     "Successful write system call"],
            [10; FSTAT_SUCCESS;     Fstat;     super::super::Success; "fstat";     "Successful fstat system call"],
            [11; FORK_SUCCESS;      Fork;      super::super::Success; "fork";      "Successful fork system call"],
            [12; EXECVE_SUCCESS;    Execve;    super::super::Success; "execve";    "Successful execve system call"],
            [13; GETRANDOM_SUCCESS; Getrandom; super::super::Success; "getrandom"; "Successful getrandom system call"],
            [14; OPENAT_SUCCESS;    Openat;    super::super::Success; "openat";    "Successful openat system call"]
        ]
    );
}

pub mod error {
    ample::result!(
        Error;
        "Linux system call failure";
        usize;
        [
            [0;  UNKNOWN_FAILURE;   Unknown;   super::super::Failure; "unknown";   "Failure from an unclassified Linux system call"],
            [2;  CLOSE_FAILURE;     Close;     super::super::Failure; "close";     "Failed close system call"],
            [3;  LSEEK_FAILURE;     Lseek;     super::super::Failure; "lseek";     "Failed lseek system call"],
            [4;  MMAP_FAILURE;      Mmap;      super::super::Failure; "mmap";      "Failed mmap system call"],
            [5;  MPROTECT_FAILURE;  Mprotect;  super::super::Failure; "mprotect";  "Failed mprotect system call"],
            [6;  MUNMAP_FAILURE;    Munmap;    super::super::Failure; "munmap";    "Failed munmap system call"],
            [7;  OPEN_FAILURE;      Open;      super::super::Failure; "open";      "Failed open system call"],
            [8;  READ_FAILURE;      Read;      super::super::Failure; "read";      "Failed read system call"],
            [9;  WRITE_FAILURE;     Write;     super::super::Failure; "write";     "Failed write system call"],
            [10; FSTAT_FAILURE;     Fstat;     super::super::Failure; "fstat";     "Failed fstat system call"],
            [11; FORK_FAILURE;      Fork;      super::super::Failure; "fork";      "Failed fork system call"],
            [12; EXECVE_FAILURE;    Execve;    super::super::Failure; "execve";    "Failed execve system call"],
            [13; GETRANDOM_FAILURE; Getrandom; super::super::Failure; "getrandom"; "Failed getrandom system call"],
            [14; OPENAT_FAILURE;    Openat;    super::super::Failure; "openat";    "Failed openat system call"]
        ]
    );
}

pub use error::Error;
pub use ok::Ok;

pub type Result = core::result::Result<Ok, Error>;
