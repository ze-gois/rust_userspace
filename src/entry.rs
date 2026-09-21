#![no_std]
#![no_main]

#[unsafe(no_mangle)]
pub extern "C" fn entry(_stack_pointer: userspace::target::arch::PointerType) -> ! {
    userspace::target::os::syscall::exit(0)
}
