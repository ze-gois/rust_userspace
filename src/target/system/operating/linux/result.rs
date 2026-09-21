pub mod ok {
    ample::result!(
        Ok;
        "Linux success";
        usize;
        [
            [0; LINUX_SYSTEM_CALL_SUCCESS; Syscall; super::super::syscall::Ok; "syscall"; "Linux system call success"]
        ]
    );
}

pub mod error {
    ample::result!(
        Error;
        "Linux failure";
        usize;
        [
            [0; LINUX_SYSTEM_CALL_FAILURE; Syscall; super::super::syscall::Error; "syscall"; "Linux system call failure"]
        ]
    );
}

pub use error::Error;
pub use ok::Ok;

pub type Result = core::result::Result<Ok, Error>;
