#[inline(always)]
pub unsafe fn syscall3(number: usize, argument1: usize, argument2: usize, argument3: usize) -> usize {
    let result: usize;
    unsafe {
        core::arch::asm!(
            "syscall",
            inlateout("rax") number => result,
            in("rdi") argument1,
            in("rsi") argument2,
            in("rdx") argument3,
            out("rcx") _,
            out("r11") _,
        );
    }
    result
}
