use super::super::super::dtype::class_32::*;

ample::r#struct!(
    pub struct Symbol32 {
        pub st_name: Word,
        pub st_value: Addr,
        pub st_size: Word,
        pub st_info: UChar,
        pub st_other: UChar,
        pub st_shndx: Half,
    }
);

pub const FORMER: Symbol32 = Symbol32 {
    st_name: Word(0),
    st_value: Addr(0),
    st_size: Word(0),
    st_info: UChar(0),
    st_other: UChar(0),
    st_shndx: Half(super::super::index::constants::SHN_UNDEF),
};

// st_name 0 No name
// st_value 0 Zero value
// st_size 0 No size
// st_info 0 No type, local binding
// st_other 0 Default visibility
// st_shndx SHN_UNDEF No section
