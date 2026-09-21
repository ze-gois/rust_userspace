use super::syscall::*;

impl crate::target::architecture::Architecture {
    pub fn syscall0(number: usize) -> crate::Result {
        syscall0(number)
    }

    pub fn syscall1(number: usize, argument1: usize) -> crate::Result {
        syscall1(number, argument1)
    }

    pub fn syscall2(number: usize, argument1: usize, argument2: usize) -> crate::Result {
        syscall2(number, argument1, argument2)
    }

    pub fn syscall3(
        number: usize,
        argument1: usize,
        argument2: usize,
        argument3: usize,
    ) -> crate::Result {
        syscall3(number, argument1, argument2, argument3)
    }

    pub fn syscall4(
        number: usize,
        argument1: usize,
        argument2: usize,
        argument3: usize,
        argument4: usize,
    ) -> crate::Result {
        syscall4(number, argument1, argument2, argument3, argument4)
    }

    pub fn syscall5(
        number: usize,
        argument1: usize,
        argument2: usize,
        argument3: usize,
        argument4: usize,
        argument5: usize,
    ) -> crate::Result {
        syscall5(
            number,
            argument1,
            argument2,
            argument3,
            argument4,
            argument5,
        )
    }

    pub fn syscall6(
        number: usize,
        argument1: usize,
        argument2: usize,
        argument3: usize,
        argument4: usize,
        argument5: usize,
        argument6: usize,
    ) -> crate::Result {
        syscall6(
            number,
            argument1,
            argument2,
            argument3,
            argument4,
            argument5,
            argument6,
        )
    }
}
