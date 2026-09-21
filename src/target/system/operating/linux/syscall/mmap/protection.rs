#[rustfmt::skip]
ample::enum_flag!(
    usize;
    "Memory map protection";
    pub enum Protection {
        [0; None;    PROT_NONE;  "None";    "Pages may not be accessed"],
        [1; Read;    PROT_READ;  "Read";    "Pages may be read"],
        [2; Write;   PROT_WRITE; "Write";   "Pages may be written"],
        [4; Execute; PROT_EXEC;  "Execute"; "Pages may be executed"],
    }
);
