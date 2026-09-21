use super::result::*;

#[inline(always)]
pub fn syscall0(n: usize) -> crate::Result {
    let syscall_return: usize;
    unsafe {
        core::arch::asm!(
            "syscall",
            inlateout("rax") n => syscall_return,
            out("rcx") _,
            out("r11") _,
        );
    }
    handle_result(syscall_return)
}

pub mod ok {
    ample::result!(
        Ok;
        "Human Ok";
        usize;
        [
            [0; X86_64_SYSCALL0_OK; Default; usize; "ZE"; "Entry to ze"],
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
            [0; X86_64_SYSCALL0_ERROR; Default; usize; "ZE"; "Entry to ze"],
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

pub fn handle_result(result: usize) -> crate::Result {
    if (result as isize) < 0 {
        core::result::Result::Err(crate::Error::Target(crate::target::Error::Arch(
            crate::target::arch::Error::X86_64Syscall(
                crate::target::arch::syscall::Error::X86_64Syscall0(Error::Default(result)),
            ),
        )))
    } else {
        core::result::Result::Ok(crate::Ok::Target(crate::target::Ok::Arch(
            crate::target::arch::Ok::X86_64Syscall(
                crate::target::arch::syscall::Ok::X86_64Syscall0(Ok::Default(result)),
            ),
        )))
    }
}
