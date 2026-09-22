#![no_std]
#![no_main]

#[unsafe(no_mangle)]
pub extern "C" fn entry(
    stack_pointer: userspace::target::architecture::StackPointer,
) -> ! {
    let stack = unsafe { userspace::memory::Stack::from_pointer(stack_pointer) };
    stack.print();

    userspace::file::print("LICENSE");
    userspace::target::os::syscall::exit(0)
}
