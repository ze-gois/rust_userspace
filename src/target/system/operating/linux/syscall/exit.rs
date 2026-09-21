use crate::target::architecture::{Architecture, traits::Callable};

#[cfg(target_arch = "x86_64")]
pub const NUMBER: usize = super::number::x86::bit64::EXIT;

pub fn exit(status_code: i32) -> ! {
    let status_code = status_code as usize;

    unsafe {
        let _ = Architecture::syscall1(NUMBER, status_code);
        core::hint::unreachable_unchecked()
    }
}
