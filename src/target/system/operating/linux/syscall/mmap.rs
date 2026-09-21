use crate::target::architecture::Architecture;

pub mod flags;
pub mod protection;

pub use flags::Flag;
pub use protection::Protection;

#[cfg(target_arch = "x86_64")]
pub const NUMBER: usize = super::number::x86::bit64::MMAP;

#[inline(always)]
#[rustfmt::skip]
pub fn mmap(addr: *mut u8, length: usize, protection: i32, flags: i32, fd: i32, offset: i64) -> crate::Result {
    let arch_result = Architecture::syscall6(
        NUMBER,
        addr as usize,
        length,
        protection as usize,
        flags as usize,
        fd as usize,
        offset as usize,
    );

    handle_result(arch_result)
}

pub mod ok {

    ample::result!( Ok; "Linux syscall Ok"; usize; [
        [0; OK; Default; usize; "Ok"; "All good"],
    ]);

    impl Ok {
        pub fn from_no(no: usize) -> Self {
            Ok::Default(no)
        }
    }
}

pub mod error {
    ample::result!(Error; "Mmap Error"; usize; [
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
        crate::Result::Ok(crate::Ok::Target(crate::target::Ok::Architecture(
            crate::target::architecture::Ok::Syscall(
                crate::target::architecture::x86::bit64::syscall::Ok::Syscall6(
                    crate::target::architecture::x86::bit64::syscall::syscall6::Ok::Default(m),
                ),
            ),
        ))) => core::result::Result::Ok(crate::Ok::Target(crate::target::Ok::OperatingSystem(
            crate::target::system::operating::linux::Ok::Syscall(crate::target::system::operating::linux::syscall::Ok::Mmap(
                crate::target::system::operating::linux::syscall::mmap::Ok::Default(m),
            )),
        ))),
        crate::Result::Err(crate::Error::Target(crate::target::Error::Architecture(
            crate::target::architecture::Error::Syscall(
                crate::target::architecture::x86::bit64::syscall::Error::Syscall6(
                    crate::target::architecture::x86::bit64::syscall::syscall6::Error::Default(errno),
                ),
            ),
        ))) => core::result::Result::Err(crate::Error::Target(crate::target::Error::OperatingSystem(
            crate::target::system::operating::linux::Error::Syscall(crate::target::system::operating::linux::syscall::Error::Mmap(
                Error::Default(errno),
            )),
        ))),
        _ => core::result::Result::Err(crate::Error::Target(crate::target::Error::OperatingSystem(
            crate::target::system::operating::linux::Error::Syscall(crate::target::system::operating::linux::syscall::Error::Mmap(
                Error::Default(3),
            )),
        ))),
    }
}
