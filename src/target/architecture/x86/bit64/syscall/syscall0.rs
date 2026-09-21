#[inline(always)]
pub fn syscall0(number: usize) -> usize {
    let result: usize;
    unsafe {
        core::arch::asm!(
            "syscall",
            inlateout("rax") number => result,
            out("rcx") _,
            out("r11") _,
        );
    }
    result
}
