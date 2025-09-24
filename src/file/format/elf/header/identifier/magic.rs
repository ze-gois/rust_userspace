ample::enum_labeled!(
    pub enum Magic,
    u8,
    "Dope",
    [
        [0; Magic0; u8; ELFMAG0; "Magic 0"; "Del (0x7f)"],
        [1; Magic1; u8; ELFMAG1; "Magic 1"; "E (0x45)"],
        [2; Magic2; u8; ELFMAG2; "Magic 2"; "L (0x4c)"],
        [3; Magic3; u8; ELFMAG3; "Magic 3"; "F (0x46)"],
    ]
);
