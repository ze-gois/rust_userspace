use super::result::*;

#[inline(always)]
pub fn syscall1(n: usize, a1: usize) -> crate::Result {
    let syscall_return: usize;

    unsafe {
        core::arch::asm!(
            "syscall",
            inlateout("rax") n => syscall_return,
            in("rdi") a1,
            out("rcx") _,
            out("r11") _
        );
    }

    handle_result(syscall_return)
}

pub mod ok {
    ample::result!(
        Ok;
        "Architecture syscall Ok";
        usize;
        [
            [0; SYSCALL1_OK; Default; usize; "ZE"; "Entry to ze"],
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
        "Architecture syscall Error";
        usize;
        [
            [0; SYSCALL1_ERROR; Default; usize; "ZE"; "Entry to ze"],
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
        core::result::Result::Err(crate::Error::Target(crate::target::Error::Architecture(
            crate::target::arch::Error::Syscall(
                crate::target::arch::syscall::Error::Syscall1(Error::Default(result)),
            ),
        )))
    } else {
        core::result::Result::Ok(crate::Ok::Target(crate::target::Ok::Architecture(
            crate::target::arch::Ok::Syscall(
                crate::target::arch::syscall::Ok::Syscall1(Ok::Default(result)),
            ),
        )))
    }
}
