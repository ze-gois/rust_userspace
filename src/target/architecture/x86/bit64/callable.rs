use super::syscall::*;

impl crate::target::architecture::traits::Callable for crate::target::architecture::Architecture {
    fn _syscall0(n: usize) -> crate::Result {
        syscall0(n)
    }

    fn _syscall1(n: usize, a1: usize) -> crate::Result {
        syscall1(n, a1)
    }

    fn _syscall2(n: usize, a1: usize, a2: usize) -> crate::Result {
        syscall2(n, a1, a2)
    }

    fn _syscall3(n: usize, a1: usize, a2: usize, a3: usize) -> crate::Result {
        syscall3(n, a1, a2, a3)
    }

    fn _syscall4(n: usize, a1: usize, a2: usize, a3: usize, a4: usize) -> crate::Result {
        syscall4(n, a1, a2, a3, a4)
    }

    fn _syscall5(n: usize, a1: usize, a2: usize, a3: usize, a4: usize, a5: usize) -> crate::Result {
        syscall5(n, a1, a2, a3, a4, a5)
    }

    fn _syscall6(
        n: usize,
        a1: usize,
        a2: usize,
        a3: usize,
        a4: usize,
        a5: usize,
        a6: usize,
    ) -> crate::Result {
        syscall6(n, a1, a2, a3, a4, a5, a6)
    }
}
