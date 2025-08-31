use crate::target::arch::{Arch, traits::Callable};

pub mod stat;
pub use stat::Stat;

hooking!(FSTAT);

#[inline(always)]
pub fn fstat(fd: isize, stat: *const Stat) -> crate::Result {
    let arch_result = Arch::syscall2(NUMBER, fd as usize, stat as usize);
    handle_result(arch_result)
}

pub mod ok {
    r#struct!(OkSyscallMUnMap { value: usize });

    result!( Ok; "MUnMap Ok"; usize; [
        [0; OK; Default; usize; "Ok"; "All good"],
    ]);

    impl Ok {
        pub fn from_no(no: usize) -> Self {
            Ok::Default(no)
        }
    }
}

pub mod error {
    result!(Error; "MUnMap error"; usize; [
        [1; ERROR; Default; usize; "Error"; "Something wicked this way comes"],
    ]);

    impl Error {
        pub fn from_no(no: usize) -> Self {
            Error::Default(no)
        }
    }
}

pub use error::Error;
pub use ok::Ok;

pub type Result = core::result::Result<Ok, Error>;

pub fn handle_result(result: crate::Result) -> crate::Result {
    // Err(crate::Error::Default(1))
    match result {
        crate::Result::Ok(crate::Ok::Target(crate::target::Ok::Arch(
            crate::target::arch::Ok::X86_64Syscall(
                crate::target::arch::syscall::Ok::X86_64Syscall2(
                    crate::target::arch::syscall::syscall2::Ok::Default(m),
                ),
            ),
        ))) => core::result::Result::Ok(crate::Ok::Target(crate::target::Ok::Os(
            crate::target::os::Ok::Syscall(crate::target::os::syscall::Ok::FStat(
                crate::target::os::syscall::fstat::Ok::Default(m),
            )),
        ))),
        _ => core::result::Result::Err(crate::Error::Target(crate::target::Error::Os(
            crate::target::os::Error::Syscall(crate::target::os::syscall::Error::FStat(
                Error::Default(3),
            )),
        ))),
    }
}
