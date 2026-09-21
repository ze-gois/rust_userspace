use crate::target::architecture::x86::bit64::syscall;

pub mod flags;
pub mod protection;
pub mod sharing;

pub use flags::Flags;
pub use protection::Protection;
pub use sharing::Sharing;

#[cfg(target_arch = "x86_64")]
pub const NUMBER: usize = super::number::x86::bit64::MMAP;

/// # Safety
///
/// The requested mapping must not invalidate live Rust references or otherwise
/// violate Rust's aliasing and lifetime rules. This is especially important for
/// fixed-address mappings.
#[inline(always)]
pub unsafe fn mmap(
    address: *mut u8,
    length: usize,
    protection: Protection,
    sharing: Sharing,
    flags: Flags,
    file_descriptor: i32,
    offset: i64,
) -> crate::Result {
    let raw_return = unsafe {
        syscall::syscall6(
            NUMBER,
            address as usize,
            length,
            protection.bits() as usize,
            (sharing.raw() | flags.bits()) as usize,
            file_descriptor as usize,
            offset as usize,
        )
    };

    handle_result(raw_return)
}

pub fn handle_result(raw: usize) -> crate::Result {
    match super::Return::new(raw).classify() {
        core::result::Result::Ok(success) => core::result::Result::Ok(
            crate::Ok::Target(crate::target::Ok::OperatingSystem(
                crate::target::system::operating::linux::Ok::Syscall(
                    crate::target::system::operating::linux::syscall::Ok::Mmap(success),
                ),
            )),
        ),
        core::result::Result::Err(failure) => core::result::Result::Err(
            crate::Error::Target(crate::target::Error::OperatingSystem(
                crate::target::system::operating::linux::Error::Syscall(
                    crate::target::system::operating::linux::syscall::Error::Mmap(failure),
                ),
            )),
        ),
    }
}
