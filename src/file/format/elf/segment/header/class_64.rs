use super::super::super::dtype::class_64::*;

ample::r#struct!(
    pub struct Header32 {
        pub p_type: Word,
        pub p_flags: Word,
        pub p_offset: Off,
        pub p_vaddr: Addr,
        pub p_paddr: Addr,
        pub p_filesz: XWord,
        pub p_memsz: XWord,
        pub p_align: XWord,
    }
);
