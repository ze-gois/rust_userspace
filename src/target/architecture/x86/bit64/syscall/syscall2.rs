#[inline(always)]
pub fn syscall2(number: usize, argument1: usize, argument2: usize) -> usize {
    let result: usize;
    unsafe {
        core::arch::asm!(
            "syscall",
            inlateout("rax") number => result,
            in("rdi") argument1,
            in("rsi") argument2,
            out("rcx") _,
            out("r11") _,
        );
    }
    result
}
