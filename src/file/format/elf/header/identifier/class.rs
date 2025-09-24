ample::enum_labeled!(
    #[derive(Debug)]
    pub enum Class,
    u8,
    "Class of pointer power",
    [
        [0; ClassNone; ELFCLASSNONE; "ELFCLASSNONE";  "Invalid class"],
        [1; Class32;   ELFCLASS32;   "ELFCLASS32";    "32-bit object class"],
        [2; Class64;   ELFCLASS64;   "ELFCLASS64";    "64-bit object class"],
    ]
);
