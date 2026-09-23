#[cfg(not(feature = "with_std"))]
use core::panic::PanicInfo;

#[panic_handler]
#[cfg(not(feature = "with_std"))]
pub fn panic(_info: &PanicInfo) -> ! {
    crate::target::operating_system::syscall::exit(101)
}

pub fn hook() {}
