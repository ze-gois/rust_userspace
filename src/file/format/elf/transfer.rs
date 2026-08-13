/// Transfer control using the Linux process-entry convention.
///
/// The target receives the newly prepared Linux initial stack in `%rsp`.
/// The dynamic interpreter uses that stack to relocate itself and the main
/// executable before transferring control to the program entry point.
pub unsafe fn jump_to_entry(entry: u64, initial_stack: crate::target::arch::PointerType) -> ! {
    unsafe {
        core::arch::asm!(
            "mov rsp, {stack}",
            "xor ebp, ebp",
            "jmp {entry}",
            stack = in(reg) initial_stack,
            entry = in(reg) entry,
            options(noreturn),
        );
    }
}
