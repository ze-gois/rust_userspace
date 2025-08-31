pub mod result;
pub use result::*;

publish_syscalls!(
    [Syscall3; 0;   READ;     read;     "Read"],
    [Syscall3; 1;   WRITE;    write;    "Write"],
    [Syscall3; 2;   OPEN;     open;     "Open"],
    [Syscall1; 3;   CLOSE;    close;    "Close"],
    [Syscall2; 5;   FSTAT;    fstat;    "FStat"],
    [Syscall3; 8;   LSEEK;    lseek;    "LSeek"],
    [Syscall6; 9;   MMAP;     mmap;     "MMap"],
    [Syscall3; 10;  MPROTECT; mprotect; "MProtect"],
    [Syscall2; 11;  MUNMAP;   munmap;   "MUnMap"],
    [Syscall1; 60;  EXIT;     exit;     "Exit"],
    [Syscall3; 257; OPENAT;   openat;   "OpenAt"],
    [Syscall4; 258; OPENAT4;  openat4;  "OpenAt4"]
);
