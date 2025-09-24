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

pub mod version;
pub use version::Version;

// use super::super::dtype::Trait;
use super::super::dtype::class_32::UChar as T;

pub const MAGIC: [<T as super::super::dtype::Trait>::Inner; 4] = [0x7f, 0x45, 0x4c, 0x46];

ample::r#struct!(
    #[derive(Debug)]
    pub struct Identifier {
        pub magic0: T,     //EI_MAG0 0 File identification
        pub magic1: T,     //EI_MAG1 1 File identification
        pub magic2: T,     //EI_MAG2 2 File identification
        pub magic3: T,     //EI_MAG3 3 File identification
        pub class: T,      //EI_CLASS 4 File class
        pub data: T,       //EI_DATA 5 Data encoding
        pub version: T,    //EI_VERSION 6 File version
        pub osabi: T,      //EI_OSABI 7 Operating system/ABI identification
        pub abiversion: T, //EI_ABIVERSION 8 ABI version
        pub padding: T,    //EI_PAD 9 Start of padding bytes
        pub unassigned0: T,
        pub unassigned1: T,
        pub unassigned2: T,
        pub unassigned3: T,
        pub unassigned4: T,
        pub nident: T, //EI_NIDENT 16 Size of e_ident[]
    }
);

impl Identifier {
    pub fn magic(&self) -> [Option<Magic>; 4] {
        [
            Magic::from_discriminant(self.magic0.0),
            Magic::from_discriminant(self.magic1.0),
            Magic::from_discriminant(self.magic2.0),
            Magic::from_discriminant(self.magic3.0),
        ]
    }

    pub fn is_magical(&self) -> bool {
        for (m, magic) in self.magic().iter().enumerate() {
            let Some(magic) = magic else {
                return false;
            };
            if magic.discriminant() != MAGIC[m] {
                return false;
            }
        }
        true
    }

    pub fn class(&self) -> Class {
        match self.class.0 {
            0x0 => Class::ClassNone,
            0x1 => Class::Class32,
            0x2 => Class::Class64,
            _ => Class::ClassNone,
        }
    }

    pub fn data(&self) -> Data {
        match self.data.0 {
            0x0 => Data::DataNone,
            0x1 => Data::DataLSB,
            0x2 => Data::DataMSB,
            _ => Data::DataNone,
        }
    }

    pub fn version(&self) -> Version {
        match self.version.0 {
            0x0 => Version::Invalid,
            0x1 => Version::Current,
            _ => Version::Invalid,
        }
    }
    pub fn endianness(&self) -> bool {
        self.data.0 == 0x1
    }

    pub fn as_array(&self) -> [T; 16] {
        [
            self.magic0,
            self.magic1,
            self.magic2,
            self.magic3,
            self.class,
            self.data,
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
                Magic : {:?},
                Class : {:?},
                Data : {:?},
                Version : {:?},
                OsABI : {:?},
                ABIVersion : {:?},
                Padding : {:?},
                Unassigned0 : {:?},
                Unassigned1 : {:?},
                Unassigned2 : {:?},
                Unassigned3 : {:?},
                Unassigned4 : {:?},
                Nident : {:?},
            \t}}
            ",
            self.magic(),
            self.class(),
            self.data(),
            self.version(),
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
