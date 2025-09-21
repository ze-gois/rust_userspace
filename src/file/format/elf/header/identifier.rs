pub mod class;
pub use class::Class;

pub mod index;
pub use index::Index;

pub mod magic;
pub use magic::Magic;

pub mod os_abi;
pub use os_abi::OsABI;

use crate::file::format::elf::dtype;

use dtype::UChar as T;

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
    pub fn from_path(filepath: &str) -> crate::Result {
        let file_descriptor = crate::file::open(filepath);
        Self::from_file_descriptor(file_descriptor)
    }

    pub fn from_file_descriptor(file_descriptor: isize) -> crate::Result {
        crate::file::seek(file_descriptor, 0);

        core::result::Result::Ok(crate::Ok::File(crate::file::Ok::Format(
            crate::file::format::Ok::Elf(crate::file::format::elf::Ok::Header(
                crate::file::format::elf::header::Ok::Identifier(Identifier {
                    magic0: T::read(file_descriptor, true)?,
                    magic1: T::read(file_descriptor, true)?,
                    magic2: T::read(file_descriptor, true)?,
                    magic3: T::read(file_descriptor, true)?,
                    class: T::read(file_descriptor, true)?,
                    endianness: T::read(file_descriptor, true)?,
                    version: T::read(file_descriptor, true)?,
                    osabi: T::read(file_descriptor, true)?,
                    abiversion: T::read(file_descriptor, true)?,
                    padding: T::read(file_descriptor, true)?,
                    unassigned0: T::read(file_descriptor, true)?,
                    unassigned1: T::read(file_descriptor, true)?,
                    unassigned2: T::read(file_descriptor, true)?,
                    unassigned3: T::read(file_descriptor, true)?,
                    unassigned4: T::read(file_descriptor, true)?,
                    nident: T::read(file_descriptor, true)?,
                }),
            )),
        )))
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
        )
    }
}
