use super::super::super::dtype::class_64::*;

ample::r#struct!(
    pub struct Header64 {
        pub sh_name: Word,
        pub sh_type: Word,
        pub sh_flags: XWord,
        pub sh_addr: Addr,
        pub sh_offset: Off,
        pub sh_size: XWord,
        pub sh_link: Word,
        pub sh_info: Word,
        pub sh_addralign: XWord,
        pub sh_entsize: XWord,
    }
);

impl Header64 {
    pub fn flags(&self) -> Option<super::Flag> {
        let flag = super::Flag::from(self.sh_flags.0);
        match flag {
            super::Flag::TODO => None,
            _ => Some(flag),
        }
    }

    pub fn r#type(&self) -> Option<super::Type> {
        super::Type::from_discriminant(self.sh_type.0 as u32)
    }
}

use super::r#type::constants::SHT_NULL;

pub const FORMER_ENTRY: Header64 = Header64 {
    sh_name: Word(0),        // No name
    sh_type: Word(SHT_NULL), // Inactive
    sh_flags: XWord(0),      // No flags
    sh_addr: Addr(0),        // No address
    sh_offset: Off(0),       // No offset
    sh_size: XWord(0),       // If non-zero, the actual number of section header entries
    sh_link: Word(0),        // If non-zero, the index of the section header string table section
    sh_info: Word(0),        // No auxiliary information
    sh_addralign: XWord(0),  // No alignment
    sh_entsize: XWord(0),    // No entries
};

ample::r#struct!(
    pub struct Compression64 {
        pub ch_type: Word,
        pub ch_reserved: Word,
        pub ch_size: XWord,
        pub ch_addralign: XWord,
    }
);
