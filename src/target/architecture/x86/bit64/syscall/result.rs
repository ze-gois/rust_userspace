pub mod ok {
    ample::result!(
        Ok;
        "Architecture syscall Ok";
        usize;
        [
            [0; SYSCALL_DEFAULT_OK; Syscall; usize; "ZE"; "Entry to ze"],
            [1; SYSCALL0_OK; Syscall0; super::super::syscall0::Ok; "ZE"; "Entry to ze"],
            [2; SYSCALL1_OK; Syscall1; super::super::syscall1::Ok; "ZE"; "Entry to ze"],
            [3; SYSCALL2_OK; Syscall2; super::super::syscall2::Ok; "ZE"; "Entry to ze"],
            [4; SYSCALL3_OK; Syscall3; super::super::syscall3::Ok; "ZE"; "Entry to ze"],
            [5; SYSCALL4_OK; Syscall4; super::super::syscall4::Ok; "ZE"; "Entry to ze"],
            [6; SYSCALL5_OK; Syscall5; super::super::syscall5::Ok; "ZE"; "Entry to ze"],
            [7; SYSCALL6_OK; Syscall6; super::super::syscall6::Ok; "ZE"; "Entry to ze"],
        ]
    );

    impl Ok {
        pub fn from_no(no: usize) -> Self {
            Ok::Syscall(no)
        }
    }
}

pub mod error {
    ample::result!(
        Error;
        "Architecture syscall Error";
        usize;
        [
            [0; SYSCALL_DEFAULT_ERROR; Syscall; usize; "ZE"; "Entry to ze"],
            [1; SYSCALL0_ERROR; Syscall0; super::super::syscall0::Error; "ZE"; "Entry to ze"],
            [2; SYSCALL1_ERROR; Syscall1; super::super::syscall1::Error; "ZE"; "Entry to ze"],
            [3; SYSCALL2_ERROR; Syscall2; super::super::syscall2::Error; "ZE"; "Entry to ze"],
            [4; SYSCALL3_ERROR; Syscall3; super::super::syscall3::Error; "ZE"; "Entry to ze"],
            [5; SYSCALL4_ERROR; Syscall4; super::super::syscall4::Error; "ZE"; "Entry to ze"],
            [6; SYSCALL5_ERROR; Syscall5; super::super::syscall5::Error; "ZE"; "Entry to ze"],
            [7; SYSCALL6_ERROR; Syscall6; super::super::syscall6::Error; "ZE"; "Entry to ze"],
        ]
    );

    impl Error {
        pub fn from_no(no: usize) -> Self {
            Error::Syscall(no)
        }
    }
}

pub use error::Error;
pub use ok::Ok;

pub type Result = core::result::Result<Ok, Error>;

pub fn handle_result(result: usize) -> crate::Result {
    if (result as isize) < 0 {
        core::result::Result::Err(crate::Error::Target(crate::target::Error::Architecture(
            crate::target::architecture::Error::Syscall(Error::from_no(result)),
        )))
    } else {
        core::result::Result::Ok(crate::Ok::Target(crate::target::Ok::Architecture(
            crate::target::architecture::Ok::Syscall(Ok::from_no(result)),
        )))
    }
}
