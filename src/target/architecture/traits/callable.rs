type C = usize;

pub type Syscall0 = fn(C) -> crate::Result;
pub type Syscall1 = fn(C, C) -> crate::Result;
pub type Syscall2 = fn(C, C, C) -> crate::Result;
pub type Syscall3 = fn(C, C, C, C) -> crate::Result;
pub type Syscall4 = fn(C, C, C, C, C) -> crate::Result;
pub type Syscall5 = fn(C, C, C, C, C, C) -> crate::Result;
pub type Syscall6 = fn(C, C, C, C, C, C, C) -> crate::Result;

enum Syscall {
    Syscall0,
    Syscall1,
    Syscall2,
    Syscall3,
    Syscall4,
    Syscall5,
    Syscall6,
}

pub trait Callable {
    fn _syscall0(n: C) -> crate::Result;
    fn _syscall1(n: C, a1: C) -> crate::Result;
    fn _syscall2(n: C, a1: C, a2: C) -> crate::Result;
    fn _syscall3(n: C, a1: C, a2: C, a3: C) -> crate::Result;
    fn _syscall4(n: C, a1: C, a2: C, a3: C, a4: C) -> crate::Result;
    fn _syscall5(n: C, a1: C, a2: C, a3: C, a4: C, a5: C) -> crate::Result;
    fn _syscall6(n: C, a1: C, a2: C, a3: C, a4: C, a5: C, a6: C) -> crate::Result;

    crate::wrap_syscall!(syscall0, _syscall0, n:C);
    crate::wrap_syscall!(syscall1, _syscall1, n:C, a1: C);
    crate::wrap_syscall!(syscall2, _syscall2, n:C, a1: C, a2: C);
    crate::wrap_syscall!(syscall3, _syscall3, n:C, a1: C, a2: C, a3: C);
    crate::wrap_syscall!(syscall4, _syscall4, n:C, a1: C, a2: C, a3: C, a4: C);
    crate::wrap_syscall!(syscall5, _syscall5, n:C, a1: C, a2: C, a3: C, a4: C, a5: C);
    crate::wrap_syscall!(syscall6, _syscall6, n:C, a1: C, a2: C, a3: C, a4: C, a5: C, a6: C);
}
