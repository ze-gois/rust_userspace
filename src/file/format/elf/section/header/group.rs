ample::enum_flag!(
    u32;
    "Data endianness";
    pub enum Group {
        [0x1; COMDAT; GRP_COMDAT; "GRP_COMDAT"; "GRP_COMDAT"],
        [0x0ff00000; MASKOS; GRP_MASKOS; "GRP_MASKOS"; "GRP_MASKOS"],
        [0xf0000000; MASKPROC; GRP_MASKPROC; "GRP_MASKPROC"; "GRP_MASKPROC"],
    }
);
