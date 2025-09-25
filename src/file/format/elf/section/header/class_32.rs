use super::super::super::dtype::class_32::*;

ample::r#struct!(
    pub struct Header32 {
        pub sh_name: Word,
        pub sh_type: Word,
        pub sh_flags: Word,
        pub sh_addr: Addr,
        pub sh_offset: Off,
        pub sh_size: Word,
        pub sh_link: Word,
        pub sh_info: Word,
        pub sh_addralign: Word,
        pub sh_entsize: Word,
    }
);

use super::r#type::constants::SHT_NULL;

pub const FORMER_ENTRY: Header32 = Header32 {
    sh_name: Word(0),        // No name
    sh_type: Word(SHT_NULL), // Inactive
    sh_flags: Word(0),       // No flags
    sh_addr: Addr(0),        // No address
    sh_offset: Off(0),       // No offset
    sh_size: Word(0),        // If non-zero, the actual number of section header entries
    sh_link: Word(0),        // If non-zero, the index of the section header string table section
    sh_info: Word(0),        // No auxiliary information
    sh_addralign: Word(0),   // No alignment
    sh_entsize: Word(0),     // No entries
};

ample::r#struct!(
    pub struct Compression32 {
        pub ch_type: Word,
        pub ch_size: Word,
        pub ch_addralign: Word,
    }
);
