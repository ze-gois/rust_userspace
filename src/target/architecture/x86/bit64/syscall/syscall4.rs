use super::result::*;

#[inline(always)]
pub fn syscall4(n: usize, a1: usize, a2: usize, a3: usize, a4: usize) -> crate::Result {
    let syscall_return: usize;

    unsafe {
        core::arch::asm!(
            "syscall",
            inlateout("rax") n => syscall_return,
            in("rdi") a1,
            in("rsi") a2,
            in("rdx") a3,
            in("r10") a4,
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
            [0; SYSCALL4_OK; Default; usize; "ZE"; "Entry to ze"],
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
            [0; SYSCALL4_ERROR; Default; usize; "ZE"; "Entry to ze"],
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
            crate::target::architecture::Error::Syscall(
                crate::target::architecture::x86::bit64::syscall::Error::Syscall4(Error::Default(result)),
            ),
        )))
    } else {
        core::result::Result::Ok(crate::Ok::Target(crate::target::Ok::Architecture(
            crate::target::architecture::Ok::Syscall(
                crate::target::architecture::x86::bit64::syscall::Ok::Syscall4(Ok::Default(result)),
            ),
        )))
    }
}
