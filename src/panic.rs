#[cfg(not(feature = "with_std"))]
use core::panic::PanicInfo;

#[panic_handler]
#[cfg(not(feature = "with_std"))]
pub fn panic(_info: &PanicInfo) -> ! {
    #[cfg(target_os = "linux")]
    {
        crate::target::system::operating::linux::syscall::exit(101)
    }

    #[cfg(not(target_os = "linux"))]
    loop {
        core::hint::spin_loop();
    }
}

pub fn hook() {}
