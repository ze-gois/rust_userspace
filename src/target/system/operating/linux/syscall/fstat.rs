use crate::target::architecture::x86::bit64::syscall;

pub mod stat;
pub use stat::Stat;

#[cfg(target_arch = "x86_64")]
pub const NUMBER: usize = super::number::x86::bit64::FSTAT;

/// # Safety
///
/// `status` must designate writable storage large enough for one Linux
/// `struct stat` value for the active ABI.
#[inline(always)]
pub unsafe fn fstat(file_descriptor: isize, status: *mut Stat) -> crate::Result {
    let raw_return = unsafe {
        syscall::syscall2(NUMBER, file_descriptor as usize, status as usize)
    };
    handle_result(raw_return)
}

pub mod ok {

    ample::result!( Ok; "Fstat Ok"; usize; [
        [0; OK; Default; usize; "Ok"; "All good"],
    ]);

    impl Ok {
        pub fn from_no(no: usize) -> Self {
            Ok::Default(no)
        }
    }
}

pub mod error {
    ample::result!(Error; "Fstat Error"; usize; [
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

pub fn handle_result(raw: usize) -> crate::Result {
    let result = super::Return::new(raw);

    if result.is_error() {
        core::result::Result::Err(crate::Error::Target(
            crate::target::Error::OperatingSystem(
                crate::target::system::operating::linux::Error::Syscall(
                    crate::target::system::operating::linux::syscall::Error::Fstat(
                        Error::Default(result.raw()),
                    ),
                ),
            ),
        ))
    } else {
        core::result::Result::Ok(crate::Ok::Target(
            crate::target::Ok::OperatingSystem(
                crate::target::system::operating::linux::Ok::Syscall(
                    crate::target::system::operating::linux::syscall::Ok::Fstat(
                        Ok::Default(result.raw()),
                    ),
                ),
            ),
        ))
    }
}
