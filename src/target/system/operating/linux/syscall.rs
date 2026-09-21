pub mod result;
pub use result::*;

syscall_modules!(
    [0;   read;     READ;     Syscall3;  "Read"],
    [1;   write;    WRITE;    Syscall3;  "Write"],
    [2;   open;     OPEN;     Syscall3;  "Open"],
    [3;   close;    CLOSE;    Syscall1;  "Close"],
    [5;   fstat;    FSTAT;    Syscall2;  "Fstat"],
    [8;   lseek;    LSEEK;    Syscall3;  "Lseek"],
    [57;  fork;     FORK;     Syscall0;  "Fork"],
    [59;  execve;   EXECVE;   Syscall3;  "Execve"],
    [9;   mmap;     MMAP;     Syscall6;  "Mmap"],
    [10;  mprotect; MPROTECT; Syscall3;  "Mprotect"],
    [11;  munmap;   MUNMAP;   Syscall2;  "Munmap"],
    [60;  exit;     EXIT;     Syscall1;  "Exit"],
    [257; openat;   OPENAT;   Syscall4;  "Openat"],
    [318; getrandom; GETRANDOM; Syscall3; "Getrandom"]
);
