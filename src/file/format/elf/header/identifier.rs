pub mod index;
pub use index::Index;

pub mod magic;
pub use magic::Magic;

pub mod class;
pub use class::Class;

pub mod data;
pub use data::Data;

pub mod os_abi;
pub use os_abi::OsABI;

use crate::file::format::elf::dtype;

use dtype::class_64::UChar as T;

ample::r#struct!(
    #[derive(Debug)]
    pub struct Identifier {
        pub magic0: T,
        pub magic1: T,
        pub magic2: T,
        pub magic3: T,
        pub class: T,
        pub endianness: T,
        pub version: T,
        pub osabi: T,
        pub abiversion: T,
        pub padding: T,
        pub unassigned0: T,
        pub unassigned1: T,
        pub unassigned2: T,
        pub unassigned3: T,
        pub unassigned4: T,
        pub nident: T,
    }
);

impl Identifier {
    pub fn class(&self) -> Class {
        match self.class.0 {
            0x0 => Class::ClassNone(self.class),
            0x1 => Class::Class32(self.class),
            0x2 => Class::Class64(self.class),
            _ => Class::ClassNone(self.class),
        }
    }

    pub fn as_array(&self) -> [T; 16] {
        [
            self.magic0,
            self.magic1,
            self.magic2,
            self.magic3,
            self.class,
            self.endianness,
            self.version,
            self.osabi,
            self.abiversion,
            self.padding,
            self.unassigned0,
            self.unassigned1,
            self.unassigned2,
            self.unassigned3,
            self.unassigned4,
            self.nident,
        ]
    }
}

impl core::fmt::Display for Identifier {
    fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            formatter,
            "
            \tELF Identifier {{
                Magic0 : {:?},
                Magic1 : {:?},
                Magic2 : {:?},
                Magic3 : {:?},
                Class : {:?},
                Endianness : {:?},
                Version : {:?},
                Osabi : {:?},
                Abiversion : {:?},
                Padding : {:?},
                Unassigned0 : {:?},
                Unassigned1 : {:?},
                Unassigned2 : {:?},
                Unassigned3 : {:?},
                Unassigned4 : {:?},
                Nident : {:?},
            \t}}
            ",
            self.magic0.0 as char,
            self.magic1.0 as char,
            self.magic2.0 as char,
            self.magic3.0 as char,
            self.class(),
            self.endianness,
            self.version,
            self.osabi,
            self.abiversion,
            self.padding,
            self.unassigned0,
            self.unassigned1,
            self.unassigned2,
            self.unassigned3,
            self.unassigned4,
            self.nident,
        )
    }
}
