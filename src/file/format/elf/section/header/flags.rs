ample::enum_flag!(
    u64;
    "Data endianness";
    pub enum Flag {
        // [0x0; None; SHF_NONE; "SHF_NONE"; "SHF_NONE"],
        [0x1; Write; HF_WRITE; "HF_WRITE"; "HF_WRITE"],
        [0x2; Alloc; SHF_ALLOC; "SHF_ALLOC"; "SHF_ALLOC"],
        [0x4; Execinstr; SHF_EXECINSTR; "SHF_EXECINSTR"; "SHF_EXECINSTR"],
        [0x10; Merge; SHF_MERGE; "SHF_MERGE"; "SHF_MERGE"],
        [0x20; Strings; SHF_STRINGS; "SHF_STRINGS"; "SHF_STRINGS"],
        [0x40; InfoLink; SHF_INFO_LINK; "SHF_INFO_LINK"; "SHF_INFO_LINK"],
        [0x80; LinkOrder; SHF_LINK_ORDER; "SHF_LINK_ORDER"; "SHF_LINK_ORDER"],
        [0x100; OsNonconforming; SHF_OS_NONCONFORMING; "SHF_OS_NONCONFORMING"; "SHF_OS_NONCONFORMING"],
        [0x200; Group; SHF_GROUP; "SHF_GROUP"; "SHF_GROUP"],
        [0x400; Tls; SHF_TLS; "SHF_TLS"; "SHF_TLS"],
        [0x800; Compressed; SHF_COMPRESSED; "SHF_COMPRESSED"; "SHF_COMPRESSED"],
        [0x0ff00000; Maskos; SHF_MASKOS; "SHF_MASKOS"; "SHF_MASKOS"],
        [0xf0000000; Maskproc; SHF_MASKPROC; "SHF_MASKPROC"; "SHF_MASKPROC"],
    }
);
