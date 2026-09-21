use super::result::*;

#[inline(always)]
pub fn syscall2(n: usize, a1: usize, a2: usize) -> crate::Result {
    let syscall_return: usize;

    unsafe {
        core::arch::asm!(
            "syscall",
            inlateout("rax") n => syscall_return,
            in("rdi") a1,
            in("rsi") a2,
            out("rcx") _,
            out("r11") _,
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
            [0; SYSCALL2_OK; Default; usize; "ZE"; "Entry to ze"],
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
            [0; SYSCALL2_ERROR; Default; usize; "ZE"; "Entry to ze"],
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
                crate::target::arch::syscall::Error::Syscall2(Error::Default(result)),
            ),
        )))
    } else {
        core::result::Result::Ok(crate::Ok::Target(crate::target::Ok::Architecture(
            crate::target::arch::Ok::Syscall(
                crate::target::arch::syscall::Ok::Syscall2(Ok::Default(result)),
            ),
        )))
    }
}
