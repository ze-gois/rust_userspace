#[cfg(not(feature = "with_std"))]
use core::panic::PanicInfo;

pub use crate::info;

#[panic_handler]
#[cfg(not(feature = "with_std"))]
pub fn panic(info: &PanicInfo) -> ! {
    hook();
    if let Some(location) = info.location() {
        // Example: send to UART or RTT instead of println
        let filename = location.file();
        let fileline = location.line() as u32;
        let filecolumn = location.column() as u32;
        info!(
            "\npanic at file: {}:{}:{}\n",
            filename, fileline, filecolumn
        );
    }
    let mut count = 5;
    loop {
        count -= 1;
        info!("x.");
        if count == 0 {
            info!("..:");
            // unsafe { core::arch::asm!("call flag_license") };
            crate::target::os::syscall::exit(23);
        }
    }
}

pub fn hook() -> () {
    // #[cfg(feature = "with_std")]
    // std::panic::set_hook(std::boxed::Box::new(|info| {
    //     info!("Custom panic: {}", info);
    //     unsafe { core::arch::asm!("call flag_license") };
    // }));
}
