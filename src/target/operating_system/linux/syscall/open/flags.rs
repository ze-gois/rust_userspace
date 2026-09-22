ample::flags!(
    i32;
    "Open flags";
    pub struct Flag {
        [0o0;       READ_ONLY;               O_RDONLY;    "O_RDONLY";    "Open for reading only"],
        [0o1;       WRITE_ONLY;              O_WRONLY;    "O_WRONLY";    "Open for writing only"],
        [0o2;       READ_WRITE;              O_RDWR;      "O_RDWR";      "Open for reading and writing"],
        [0o100;     CREATE;                  O_CREAT;     "O_CREAT";     "Create file if it does not exist"],
        [0o200;     EXCLUSIVE;               O_EXCL;      "O_EXCL";      "Require exclusive creation"],
        [0o400;     NO_CONTROLLING_TERMINAL; O_NOCTTY;    "O_NOCTTY";    "Do not assign a controlling terminal"],
        [0o1000;    TRUNCATE;                O_TRUNC;     "O_TRUNC";     "Truncate an existing regular file"],
        [0o2000;    APPEND;                  O_APPEND;    "O_APPEND";    "Append each write"],
        [0o4000;    NON_BLOCKING;            O_NONBLOCK;  "O_NONBLOCK";  "Use non-blocking I/O"],
        [0o10000;   DATA_SYNCHRONIZED;       O_DSYNC;     "O_DSYNC";     "Synchronized data I/O"],
        [0o4010000; SYNCHRONIZED;            O_SYNC;      "O_SYNC";      "Synchronized file I/O"],
        [0o200000;  DIRECTORY;               O_DIRECTORY; "O_DIRECTORY"; "Require a directory"],
        [0o400000;  NO_FOLLOW;               O_NOFOLLOW;  "O_NOFOLLOW";  "Do not follow the final symbolic link"],
        [0o2000000; CLOSE_ON_EXECUTE;        O_CLOEXEC;   "O_CLOEXEC";   "Close on successful exec"]
    }
);

impl Flag {
    pub const RDONLY: Self = Self::READ_ONLY;
    pub const WRONLY: Self = Self::WRITE_ONLY;
    pub const RDWR: Self = Self::READ_WRITE;
}
