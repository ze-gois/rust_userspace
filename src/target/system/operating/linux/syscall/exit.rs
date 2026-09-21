use crate::target::architecture::x86::bit64::syscall;

#[cfg(target_arch = "x86_64")]
pub const NUMBER: usize = super::number::x86::bit64::EXIT;

pub fn exit(status_code: i32) -> ! {
    let _ = unsafe { syscall::syscall1(NUMBER, status_code as usize) };

    loop {
        core::hint::spin_loop();
    }
}
