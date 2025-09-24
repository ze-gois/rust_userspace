ample::enum_labeled_typed!(
    #[derive(Debug)]
    pub enum Class,
    u8,
    "Class of pointer power",
    [
        [0; ClassNone; super::T; ELFCLASSNONE; "ELFCLASSNONE";  "Invalid class"],
        [1; Class32;   super::T; ELFCLASS32;   "ELFCLASS32";    "32-bit object class"],
        [2; Class64;   super::T; ELFCLASS64;   "ELFCLASS64";    "64-bit object class"],
    ]
);
