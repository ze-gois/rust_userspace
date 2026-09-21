use crate::target::architecture::x86::bit64::syscall;

#[cfg(target_arch = "x86_64")]
pub const NUMBER: usize = super::number::x86::bit64::READ;

/// # Safety
///
/// `byte_buffer` must designate writable storage for at least `byte_count`
/// bytes.
pub unsafe fn read(
    file_descriptor: isize,
    byte_buffer: *mut u8,
    byte_count: usize,
) -> crate::Result {
    let raw_return = unsafe {
        syscall::syscall3(
            NUMBER,
            file_descriptor as usize,
            byte_buffer as usize,
            byte_count,
        )
    };

    handle_result(raw_return)
}

pub fn handle_result(raw: usize) -> crate::Result {
    match super::Return::new(raw).classify() {
        core::result::Result::Ok(success) => core::result::Result::Ok(
            crate::Ok::Target(crate::target::Ok::OperatingSystem(
                crate::target::system::operating::linux::Ok::Syscall(
                    crate::target::system::operating::linux::syscall::Ok::Read(success),
                ),
            )),
        ),
        core::result::Result::Err(failure) => core::result::Result::Err(
            crate::Error::Target(crate::target::Error::OperatingSystem(
                crate::target::system::operating::linux::Error::Syscall(
                    crate::target::system::operating::linux::syscall::Error::Read(failure),
                ),
            )),
        ),
    }
}
