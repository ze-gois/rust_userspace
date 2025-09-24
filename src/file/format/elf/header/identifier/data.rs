ample::enum_labeled_typed!(
    pub enum Data,
    u8,
    "Data endianness",
    [
        [0; Data0; super::T; ELFDATANONE; "Data 0"; "Del (0x7f)"],
        [1; Data1; super::T; ELFDATA2LSB; "Data 1"; "E (0x45)"],
        [2; Data2; super::T; ELFDATA2MSB; "Data 2"; "L (0x4c)"],
    ]
);
