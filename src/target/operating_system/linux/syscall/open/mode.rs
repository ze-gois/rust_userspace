ample::flags!(
    usize;
    "Open mode";
    pub struct Mode {
        [0o400; OWNER_READ;    S_IRUSR; "S_IRUSR"; "Owner may read"],
        [0o200; OWNER_WRITE;   S_IWUSR; "S_IWUSR"; "Owner may write"],
        [0o100; OWNER_EXECUTE; S_IXUSR; "S_IXUSR"; "Owner may execute"],
        [0o040; GROUP_READ;    S_IRGRP; "S_IRGRP"; "Group may read"],
        [0o020; GROUP_WRITE;   S_IWGRP; "S_IWGRP"; "Group may write"],
        [0o010; GROUP_EXECUTE; S_IXGRP; "S_IXGRP"; "Group may execute"],
        [0o004; OTHER_READ;    S_IROTH; "S_IROTH"; "Others may read"],
        [0o002; OTHER_WRITE;   S_IWOTH; "S_IWOTH"; "Others may write"],
        [0o001; OTHER_EXECUTE; S_IXOTH; "S_IXOTH"; "Others may execute"]
    }
);
