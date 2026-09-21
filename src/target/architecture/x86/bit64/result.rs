pub mod ok {
    ample::result!( Ok; "Architecture syscall Ok"; usize; [
        [0; OK; Default; usize; "Ok"; "All good"],
        [1; X86_64SYSCALL_OK; Syscall; super::super::syscall::Ok; "Ok"; "All good"],
    ]);

    impl Ok {
        pub fn from_no(no: usize) -> Self {
            Ok::Default(no)
        }
    }
}

// diferente
pub mod error {
    ample::result!(Error; "Architecture syscall Error"; usize; [
        [1; ERROR; Default; usize; "Error"; "Something wicked this way comes"],
        [0; OK; Syscall; super::super::syscall::Error; "Ok"; "All good"],
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

pub fn handle_result(result: usize) -> crate::Result {
    if (result as isize) < 0 {
        core::result::Result::Err(crate::Error::Target(crate::target::Error::Architecture(
            Error::from_no(result),
        )))
    } else {
        core::result::Result::Ok(crate::Ok::Target(crate::target::Ok::Architecture(Ok::from_no(
            result,
        ))))
    }
}
