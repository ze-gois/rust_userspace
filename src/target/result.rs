pub mod ok {
    ample::result!(
        Ok;
        "Human Ok";
        usize;
        [
            [1; TARGET_DEFAULT_OK; Default; usize; "ZE"; "Entry to ze"],
            [2; TARGET_INFO_OK; Info; usize; "ZE"; "Entry to ze"],
            [3; TARGET_OPERATING_SYSTEM_OK; OperatingSystem; crate::target::os::Ok; "ZE"; "Entry to ze"],
            [4; TARGET_ARCHITECTURE_OK; Architecture; crate::target::arch::Ok; "ZE"; "Entry to ze"],
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
        "Human error";
        usize;
        [
            [1; TARGET_DEFAULT_ERROR; Default; usize; "ZE"; "Entry to ze"],
            [2; TARGET_INFO_ERROR; Info; usize; "ZE"; "Entry to ze"],
            [3; TARGET_OPERATING_SYSTEM_ERROR; OperatingSystem; crate::target::os::Error; "ZE"; "Entry to ze"],
            [4; TARGET_ARCHITECTURE_ERROR; Architecture; crate::target::arch::Error; "ZE"; "Entry to ze"],
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

// pub fn handle_result(result: usize) -> Result {
//     if (result as isize) < 0 {
//         Err(Error::from_no(result))
//     } else {
//         Ok(Ok::from_no(result))
//     }
// }
