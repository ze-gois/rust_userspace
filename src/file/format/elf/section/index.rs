ample::enum_labeled!(
    #[derive(Debug)]
    pub enum Index,
    u16,
    "Data endianness",
    [
        [0      ; UNDEF     ; SHN_UNDEF                 ; "SHN_UNDEF"     ; "SHN_UNDEF"],
        [0xff00 ; LORESERVE ; SHN_LOPROC, SHN_LORESERVE ; "SHN_LORESERVE" ; "SHN_LORESERVE"],
        // [0xff00 ; LOPROC    ; SHN_LOPROC                ; "SHN_LOPROC"    ; "SHN_LOPROC"],
        [0xff1f ; HIPROC    ; SHN_HIPROC                ; "SHN_HIPROC"    ; "SHN_HIPROC"],
        [0xff20 ; LOOS      ; SHN_LOOS                  ; "SHN_LOOS"      ; "SHN_LOOS"],
        [0xff3f ; HIOS      ; SHN_HIOS                  ; "SHN_HIOS"      ; "SHN_HIOS"],
        [0xfff1 ; ABS       ; SHN_ABS                   ; "SHN_ABS"       ; "SHN_ABS"],
        [0xfff2 ; COMMON    ; SHN_COMMON                ; "SHN_COMMON"    ; "SHN_COMMON"],
        // [0xffff ; XINDEX    ; SHN_XINDEX                ; "SHN_XINDEX"    ; "SHN_XINDEX"],
        [0xffff ; HIRESERVE ; SHN_XINDEX, SHN_HIRESERVE ; "SHN_HIRESERVE" ; "SHN_HIRESERVE"],
    ]
);
