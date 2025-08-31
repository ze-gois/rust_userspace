enum_labeled!(
    pub Class,
    u8,
    "Class of pointer power",
    [
        [0; NON; (); NON; "NON";"Invalid class"],
        [1; C32; (); C32; "C32";"32-bit objects class"],
        [2; C64; (); C64; "C64";"64-bit objects class"],
        [3; NUM; (); NUM; "NUM";"NUM class"],
    ]
);
