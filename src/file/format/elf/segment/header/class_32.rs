use super::super::super::dtype::class_32::*;

ample::r#struct!(
    pub struct Header32 {
        pub p_type: Word,
        pub p_offset: Off,
        pub p_vaddr: Addr,
        pub p_paddr: Addr,
        pub p_filesz: Word,
        pub p_memsz: Word,
        pub p_flags: Word,
        pub p_align: Word,
    }
);
