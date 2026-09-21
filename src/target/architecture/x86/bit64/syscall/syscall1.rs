#[inline(always)]
pub fn syscall1(number: usize, argument1: usize) -> usize {
    let result: usize;
    unsafe {
        core::arch::asm!(
            "syscall",
            inlateout("rax") number => result,
            in("rdi") argument1,
            out("rcx") _,
            out("r11") _,
        );
    }
    result
}
