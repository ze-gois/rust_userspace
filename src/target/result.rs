pub mod ok {
    ample::result!(
        Ok;
        "Target success";
        usize;
        [
            [1; TARGET_DEFAULT_OK; Default; usize; "default"; "Default target success"],
            [2; TARGET_INFO_OK; Info; usize; "information"; "Target information success"],
            [3; TARGET_OS_OK; Os; crate::target::os::Ok; "operating system"; "Operating-system target success"],
            [4; TARGET_ARCH_OK; Arch; crate::target::arch::Ok; "architecture"; "Architecture target success"],
        ]
    );

    impl Ok {
        pub fn from_no(no: usize) -> Self {
            Ok::Default(no)
        }
    }
}

pub mod error {
    ample::result!(
        Error;
        "Target failure";
        usize;
        [
            [1; TARGET_DEFAULT_ERROR; Default; usize; "default"; "Default target failure"],
            [2; TARGET_INFO_ERROR; Info; usize; "information"; "Target information failure"],
            [3; TARGET_OS_ERROR; Os; crate::target::os::Error; "operating system"; "Operating-system target failure"],
            [4; TARGET_ARCH_ERROR; Arch; crate::target::arch::Error; "architecture"; "Architecture target failure"],
        ]
    );

    impl Error {
        pub fn from_no(no: usize) -> Self {
            Error::Default(no)
        }
    }
}

pub use error::Error;
pub use ok::Ok;

pub type Result = core::result::Result<Ok, Error>;
