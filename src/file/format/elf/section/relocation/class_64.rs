use super::super::super::dtype::class_64::*;

ample::r#struct!(
    pub struct Rel64 {
        pub r_offset: Addr,
        pub r_info: XWord,
    }
);
ample::r#struct!(
    pub struct Rela64 {
        pub r_offset: Addr,
        pub r_info: XWord,
        pub r_addend: SXWord,
    }
);

// #define ELF64_R_SYM(i) ((i)>>32)
// #define ELF64_R_TYPE(i) ((i)&0xffffffffL)
// #define ELF64_R_INFO(s,t) (((s)<<32)+((t)&0xffffffffL))
