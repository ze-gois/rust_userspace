// #[rustfmt::skip]
// ample::enum_flag!(Flag, usize, "Map Flags",[
//     [0o0;        RDONLY;         RDONLY;          "Shared";         "Shared"],
//     [0o1;        WRONLY;         WRONLY;         "Private";        "Private"],
//     [0o2;        RDWR;           RDWR;  "SharedValidate"; "SharedValidate"],
//     [0o100;      CREAT;          CREAT;           "Fixed";          "Fixed"],
//     [0o200;      EXCL;           EXCL;       "Anonymous";      "Anonymous"],
//     [0o400;      NOCTTY;         NOCTTY;       "GrowsDown";      "GrowsDown"],
//     [0o1000;     TRUNC;          TRUNC;       "DenyWrite";      "DenyWrite"],
//     [0o2000;     APPEND;         APPEND;      "Executable";     "Executable"],
//     [0o4000;     NONBLOCK;       NONBLOCK;          "Locked";         "Locked"],
//     [0o10000;    DSYNC;          DSYNC;       "NoReserve";      "NoReserve"],
//     [0o4010000;  SYNC;           SYNC;        "Populate";       "Populate"],
//     [0o200000;   DIRECTORY;      DIRECTORY;        "NonBlock";       "NonBlock"],
//     [0o400000;   NOFOLLOW;       NOFOLLOW;           "Stack";          "Stack"],
//     [0o2000000;  CLOEXEC;        CLOEXEC;         "HugeTlb";        "HugeTlb"],
// ]);

#[rustfmt::skip]
ample::enum_flag!(
    i32;
    "Open Flags";
    pub enum Flag {
        [0o0; RDONLY; RDONLY; "Read only"; "Read only"],
        [0o1; WRONLY; WRONLY; "Write only"; "Write only"],
        [0o2; RDWR;   RDWR;   "Read/Write"; "Read/Write"],
        [0o100; CREAT; CREAT; "Create"; "Create file if not exists"],
        [0o200; EXCL;  EXCL;  "Exclusive"; "Fail if exists"],
        [0o400; NOCTTY; NOCTTY; "No controlling TTY"; "Do not assign controlling TTY"],
        [0o1000; TRUNC; TRUNC; "Truncate"; "Truncate file if exists"],
        [0o2000; APPEND; APPEND; "Append"; "Append mode"],
        [0o4000; NONBLOCK; NONBLOCK; "Non-blocking"; "Non-blocking I/O"],
        [0o10000; DSYNC; DSYNC; "DSync"; "Synchronized I/O"],
        [0o4010000; SYNC; SYNC; "Sync"; "Synchronous writes"],
        [0o200000; DIRECTORY; DIRECTORY; "Directory"; "Must be a directory"],
        [0o400000; NOFOLLOW; NOFOLLOW; "No Follow"; "Do not follow symlinks"],
        [0o2000000; CLOEXEC; CLOEXEC; "Close on exec"; "Close on exec"]
    }
);
