#[rustfmt::skip]
ample::enum_flag!(
    usize;
    "Open Mode";
    pub enum Mode {
        [0o400; ReadOwner;  READ_OWNER;  "Read owner";  "Readable by owner"],
        [0o200; WriteOwner; WRITE_OWNER; "Write owner"; "Writable by owner"],
        [0o100; ExecOwner;  EXEC_OWNER;  "Exec owner";  "Executable by owner"],
        [0o40;  ReadGroup;  READ_GROUP;  "Read group";  "Readable by group"],
        [0o20;  WriteGroup; WRITE_GROUP; "Write group"; "Writable by group"],
        [0o10;  ExecGroup;  EXEC_GROUP;  "Exec group";  "Executable by group"],
        [0o4;   ReadOther;  READ_OTHER;  "Read other";  "Readable by others"],
        [0o2;   WriteOther; WRITE_OTHER; "Write other"; "Writable by others"],
        [0o1;   ExecOther;  EXEC_OTHER;  "Exec other";  "Executable by others"]
    }
);
