#![no_std]
#![no_main]

#[unsafe(no_mangle)]
pub extern "C" fn entry(_stack_pointer: userspace::target::arch::StackPointer) -> ! {
    userspace::target::os::linux::syscall::exit(0)
}
