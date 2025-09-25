ample::enum_flag!(
    u64;
    "Segment Flags";
    pub enum Flag {
        [0          ; Null    ; PT_NULL    ; "PT_NULL"    ; "PT_NULL"],
        [1          ; Load    ; PT_LOAD    ; "PT_LOAD"    ; "PT_LOAD"],
        [2          ; Dynamic ; PT_DYNAMIC ; "PT_DYNAMIC" ; "PT_DYNAMIC"],
        [3          ; Interp  ; PT_INTERP  ; "PT_INTERP"  ; "PT_INTERP"],
        [4          ; Note    ; PT_NOTE    ; "PT_NOTE"    ; "PT_NOTE"],
        [5          ; Shlib   ; PT_SHLIB   ; "PT_SHLIB"   ; "PT_SHLIB"],
        [6          ; Phdr    ; PT_PHDR    ; "PT_PHDR"    ; "PT_PHDR"],
        [7          ; Tls     ; PT_TLS     ; "PT_TLS"     ; "PT_TLS"],
        [0x60000000 ; Loos    ; PT_LOOS    ; "PT_LOOS"    ; "PT_LOOS"],
        [0x6fffffff ; Hios    ; PT_HIOS    ; "PT_HIOS"    ; "PT_HIOS"],
        [0x70000000 ; Loproc  ; PT_LOPROC  ; "PT_LOPROC"  ; "PT_LOPROC"],
        [0x7fffffff ; Hiproc  ; PT_HIPROC  ; "PT_HIPROC"  ; "PT_HIPROC"],
    }
);
