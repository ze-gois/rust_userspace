ample::enum_labeled_typed!(
    pub enum Magic,
    u8,
    "Dope",
    [
        [0; Magic0; super::T; ELFMAG0; "Magic 0"; "Del (0x7f)"],
        [1; Magic1; super::T; ELFMAG1; "Magic 1"; "E (0x45)"],
        [2; Magic2; super::T; ELFMAG2; "Magic 2"; "L (0x4c)"],
        [3; Magic3; super::T; ELFMAG3; "Magic 3"; "F (0x46)"],
    ]
);
