use super::super::super::dtype::class_64::*;

ample::r#struct!(
    pub struct Symbol64 {
        pub st_name: Word,
        pub st_info: UChar,
        pub st_other: UChar,
        pub st_shndx: Half,
        pub st_value: Addr,
        pub st_size: XWord,
    }
);

pub const FORMER: Symbol64 = Symbol64 {
    st_name: Word(0),
    st_info: UChar(0),
    st_other: UChar(0),
    st_shndx: Half(super::super::index::constants::SHN_UNDEF),
    st_value: Addr(0),
    st_size: XWord(0),
};
