//! Linux system call numbers for the x86-64 system-call ABI.
//!
//! These values are architecture-specific. They are kept out of the generic
//! Linux syscall namespace so another architecture cannot accidentally inherit
//! the x86-64 numbering.

pub const READ: usize = 0;
pub const WRITE: usize = 1;
pub const OPEN: usize = 2;
pub const CLOSE: usize = 3;
pub const FSTAT: usize = 5;
pub const LSEEK: usize = 8;
pub const MMAP: usize = 9;
pub const MPROTECT: usize = 10;
pub const MUNMAP: usize = 11;
pub const FORK: usize = 57;
pub const EXECVE: usize = 59;
pub const EXIT: usize = 60;
pub const OPENAT: usize = 257;
pub const GETRANDOM: usize = 318;
