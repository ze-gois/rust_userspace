#[rustfmt::skip]
ample::enum_flag!(
    isize;
    "Map Flags";
    pub enum AtFlag {
        [ -100; FDCWD;           RDONLY;          "Shared";         "Shared"],
        [0x200; REMOVEDIR;       WRONLY;         "Private";        "Private"],
        [0x400; SymlinkFollow;   RDWR;    "SharedValidate"; "SharedValidate"],
        [0x100; SymlinkNoFollow; CREAT;            "Fixed";          "Fixed"],
    }
);
