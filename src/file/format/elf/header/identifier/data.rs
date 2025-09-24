ample::enum_labeled!(
    #[derive(Debug)]
    pub enum Data,
    u8,
    "Data endianness",
    [
        [0; DataNone; ELFDATANONE; "Data 0";   "No data encoding?"],
        [1; DataLSB;  ELFDATA2LSB; "Data LSB"; "Least-Significant Byte stored in byte 0"],
        [2; DataMSB;  ELFDATA2MSB; "Data MSB"; "Most-Significant Byte stored in byte 0"],
    ]
);
