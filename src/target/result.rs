pub mod ok {
    ample::result!(
        Ok;
        "Target Ok";
        usize;
        [
            [1; TARGET_DEFAULT_OK; Default; usize; "Target"; "Target result"],
            [2; TARGET_INFO_OK; Info; usize; "Target"; "Target result"],
            [3; TARGET_OPERATING_SYSTEM_OK; OperatingSystem; crate::target::system::operating::Ok; "Target"; "Target result"],
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
        "Target Error";
        usize;
        [
            [1; TARGET_DEFAULT_ERROR; Default; usize; "Target"; "Target result"],
            [2; TARGET_INFO_ERROR; Info; usize; "Target"; "Target result"],
            [3; TARGET_OPERATING_SYSTEM_ERROR; OperatingSystem; crate::target::system::operating::Error; "Target"; "Target result"],
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
