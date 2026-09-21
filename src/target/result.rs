pub mod ok {
    ample::result!(
        Ok;
        "Target success";
        usize;
        [
            [0; TARGET_OPERATING_SYSTEM_SUCCESS; OperatingSystem; crate::target::system::operating::Ok; "operating system"; "Operating-system target success"]
        ]
    );
}

pub mod error {
    ample::result!(
        Error;
        "Target failure";
        usize;
        [
            [0; TARGET_OPERATING_SYSTEM_FAILURE; OperatingSystem; crate::target::system::operating::Error; "operating system"; "Operating-system target failure"]
        ]
    );
}

pub use error::Error;
pub use ok::Ok;

pub type Result = core::result::Result<Ok, Error>;
