ample::flags!(
    usize;
    "Memory protection";
    pub struct Prot {
        [0; NONE;    PROT_NONE;  "PROT_NONE";  "Pages may not be accessed"],
        [1; READ;    PROT_READ;  "PROT_READ";  "Pages may be read"],
        [2; WRITE;   PROT_WRITE; "PROT_WRITE"; "Pages may be written"],
        [4; EXECUTE; PROT_EXEC;  "PROT_EXEC";  "Pages may be executed"]
    }
);
