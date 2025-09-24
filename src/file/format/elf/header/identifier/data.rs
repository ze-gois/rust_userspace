ample::enum_labeled!(
    pub enum Data,
    u8,
    "Data endianness",
    [
        [0; Data0; u8; ELFDATANONE; "Data 0"; "Del (0x7f)"],
        [1; Data1; u8; ELFDATA2LSB; "Data 1"; "E (0x45)"],
        [2; Data2; u8; ELFDATA2MSB; "Data 2"; "L (0x4c)"],
    ]
);
