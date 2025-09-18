pub mod ok {
    ample::result!(
        Ok;
        "Human Ok";
        usize;
        [
            [0; X86_64_SYSCALL_DEFAULT_OK; X86_64Syscall; usize; "ZE"; "Entry to ze"],
            [1; X86_64_SYSCALL0_OK; X86_64Syscall0; super::super::syscall0::Ok; "ZE"; "Entry to ze"],
            [2; X86_64_SYSCALL1_OK; X86_64Syscall1; super::super::syscall1::Ok; "ZE"; "Entry to ze"],
            [3; X86_64_SYSCALL2_OK; X86_64Syscall2; super::super::syscall2::Ok; "ZE"; "Entry to ze"],
            [4; X86_64_SYSCALL3_OK; X86_64Syscall3; super::super::syscall3::Ok; "ZE"; "Entry to ze"],
            [5; X86_64_SYSCALL4_OK; X86_64Syscall4; super::super::syscall4::Ok; "ZE"; "Entry to ze"],
            [6; X86_64_SYSCALL5_OK; X86_64Syscall5; super::super::syscall5::Ok; "ZE"; "Entry to ze"],
            [7; X86_64_SYSCALL6_OK; X86_64Syscall6; super::super::syscall6::Ok; "ZE"; "Entry to ze"],
        ]
    );

    impl Ok {
        pub fn from_no(no: usize) -> Self {
            Ok::X86_64Syscall(no)
        }
    }
}

pub mod error {
    ample::result!(
        Error;
        "Human error";
        usize;
        [
            [0; X86_64_SYSCALL_DEFAULT_ERROR; X86_64Syscall; usize; "ZE"; "Entry to ze"],
            [1; X86_64_SYSCALL0_ERROR; X86_64Syscall0; super::super::syscall0::Error; "ZE"; "Entry to ze"],
            [2; X86_64_SYSCALL1_ERROR; X86_64Syscall1; super::super::syscall1::Error; "ZE"; "Entry to ze"],
            [3; X86_64_SYSCALL2_ERROR; X86_64Syscall2; super::super::syscall2::Error; "ZE"; "Entry to ze"],
            [4; X86_64_SYSCALL3_ERROR; X86_64Syscall3; super::super::syscall3::Error; "ZE"; "Entry to ze"],
            [5; X86_64_SYSCALL4_ERROR; X86_64Syscall4; super::super::syscall4::Error; "ZE"; "Entry to ze"],
            [6; X86_64_SYSCALL5_ERROR; X86_64Syscall5; super::super::syscall5::Error; "ZE"; "Entry to ze"],
            [7; X86_64_SYSCALL6_ERROR; X86_64Syscall6; super::super::syscall6::Error; "ZE"; "Entry to ze"],
        ]
    );

    impl Error {
        pub fn from_no(no: usize) -> Self {
            Error::X86_64Syscall(no)
        }
    }
}

pub use error::Error;
pub use ok::Ok;

pub type Result = core::result::Result<Ok, Error>;

pub fn handle_result(result: usize) -> crate::Result {
    if (result as isize) < 0 {
        core::result::Result::Err(crate::Error::Target(crate::target::Error::Arch(
            crate::target::arch::Error::X86_64Syscall(Error::from_no(result)),
        )))
    } else {
        core::result::Result::Ok(crate::Ok::Target(crate::target::Ok::Arch(
            crate::target::arch::Ok::X86_64Syscall(Ok::from_no(result)),
        )))
    }
}
