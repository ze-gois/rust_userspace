#[inline(always)]
pub fn syscall4(
    number: usize,
    argument1: usize,
    argument2: usize,
    argument3: usize,
    argument4: usize,
) -> usize {
    let result: usize;
    unsafe {
        core::arch::asm!(
            "syscall",
            inlateout("rax") number => result,
            in("rdi") argument1,
            in("rsi") argument2,
            in("rdx") argument3,
            in("r10") argument4,
            out("rcx") _,
            out("r11") _,
        );
    }
    result
}
