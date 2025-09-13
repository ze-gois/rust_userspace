pub mod result;
pub use result::*;

syscall_modules!(
    [0;   read;     READ;     Syscall3;  "Read"],
    [1;   write;    WRITE;    Syscall3;  "Write"],
    [2;   open;     OPEN;     Syscall3;  "Open"],
    [3;   close;    CLOSE;    Syscall1;  "Close"],
    [5;   fstat;    FSTAT;    Syscall2;  "FStat"],
    [8;   lseek;    LSEEK;    Syscall3;  "LSeek"],
    [9;   mmap;     MMAP;     Syscall6;  "MMap"],
    [10;  mprotect; MPROTECT; Syscall3;  "MProtect"],
    [11;  munmap;   MUNMAP;   Syscall2;  "MUnMap"],
    [60;  exit;     EXIT;     Syscall1;  "Exit"],
    [257; openat;   OPENAT;   Syscall3;  "OpenAt"],
    [258; openat4;  OPENAT4;  Syscall4;  "OpenAt4"]
);
