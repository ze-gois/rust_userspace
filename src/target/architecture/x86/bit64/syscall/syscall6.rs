#[inline(always)]
pub fn syscall6(
    number: usize,
    argument1: usize,
    argument2: usize,
    argument3: usize,
    argument4: usize,
    argument5: usize,
    argument6: usize,
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
            in("r8") argument5,
            in("r9") argument6,
            out("rcx") _,
            out("r11") _,
        );
    }
    result
}
