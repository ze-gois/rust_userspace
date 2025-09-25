use super::super::super::dtype::class_32::*;

ample::r#struct!(
    pub struct Rel32 {
        pub r_offset: Addr,
        pub r_info: Word,
    }
);

ample::r#struct!(
    pub struct Rela32 {
        pub r_offset: Addr,
        pub r_info: Word,
        pub r_addend: SWord,
    }
);

// #define ELF32_R_SYM(i) ((i)>>8)
// #define ELF32_R_TYPE(i) ((unsigned char)(i))
// #define ELF32_R_INFO(s,t) (((s)<<8)+(unsigned char)(t))
