pub mod class;
pub use class::Class;

pub mod index;
pub use index::Index;

pub mod magic;
pub use magic::Magic;

pub mod os_abi;
pub use os_abi::OsABI;

use crate::file::format::elf::dtype;
use dtype::Endianness;

use dtype::UChar as T;

// /// # Identifier to file as an ELF object file
// ///
// /// Provide information about the data representation of the object file structures.
// /// The bytes of this array that have defined meanings are detailed below.
// /// The remaining bytes are reserved for future use, and should be set to zero.
// #[repr(C)]
// #[derive(Copy, Debug)]
crate::r#struct!(pub Identifier {
    /// # Array with “magic numbers” identifying the file as an ELF object file.
    ///
    /// They contain the characters ‘\x7f’, ‘E’, ‘L’, and ‘F’, respectively.
    /// pub magic: [T; 4],
    pub magic0: T,
    pub magic1: T,
    pub magic2: T,
    pub magic3: T,
    /// # Identifies the class of the object file, or its capacity.
    ///
    /// |Name|Value|Meaning|
    /// |:-:|:-:|:-:|
    /// |C32|1|32-bit objects|
    /// |C64|2|64-bit objects|
    ///
    /// The class of the ELF file is independent of the data model assumed by the
    /// object code. The EI_CLASS field identifies the file format; a processor-
    /// specific flag in the e_flags field, described below, may be used to identify
    /// the application’s data model if the processory supports multiple models.
    pub class: T,
    /// # Data encoding of the object file data structures
    ///
    /// *endianness was originaly identified with "data"*
    ///
    /// |Name|Value|Meaning|
    /// |:-:|:-:|:-:|
    /// |None|0|Invalid endianness|
    /// |LSB|1|Little-endian|
    /// |MSB|2|Big-endian|
    ///
    /// For the convenience of code that examines ELF object files at run time
    /// (e.g., the dynamic loader), it is intended that the data encoding of the
    /// object file will match that of the running program. For environments that
    /// support both byte orders, a processor-specific flag in the e_flags field,
    /// described below, may be used to identify the application’s operating mode.
    pub endianness: T,
    /// # Version of the object file format.
    ///
    /// Currently, this field has the value
    /// CURRENT, which is defined with the value 1.
    pub version: T,
    /// # Operating system and ABI for which the object is prepared.
    ///
    /// Some fields in other ELF structures have flags and values
    /// that have environment-specific meanings; the interpretation of
    /// those fields is determined by the value of this field.
    pub osabi: T,
    /// # ABI version for which the object is prepared.
    ///
    /// Used to distinguish incompatible versions of an ABI.
    /// Interpretation of this version number is dependent on the ABI identified
    /// by the EI_OSABI field.
    /// For applications conforming to the System V ABI, third edition, this field
    /// should contain 0.
    pub abiversion: T,
    /// # Padding bytes
    pub padding: T,
    /// # Five unsassigned bytes
    pub unassigned0: T,
    pub unassigned1: T,
    pub unassigned2: T,
    pub unassigned3: T,
    pub unassigned4: T,
    /// # Number of bytes of identifier
    pub nident: T,
});

impl Identifier {
    /// Loads ELF Identifier from a compliant path
    ///
    /// ```rust
    /// let path : &str = "./data/symver.powerpc64.so";
    /// let identifier = lib::header::elf::Identifier::from_filepath(&path);
    /// println!("{}",identifier);
    /// ```
    pub fn from_filepath(filepath: &str) -> crate::Result {
        let file_descriptor = crate::file::open(filepath);
        Self::from_file_descriptor(file_descriptor)
    }

    /// Loads ELF Identifier from a filemap
    ///
    /// ```rust
    /// let path : &str = "./data/symver.powerpc64.so";
    /// let file = core::fs::File::open(path).unwrap();
    /// let map = unsafe { memmap2::Mmap::map(&file).unwrap() };
    /// let identifier = lib::header::elf::Identifier::from_memmap(&map);
    /// println!("{}",identifier);
    /// ```
    pub fn from_file_descriptor(file_descriptor: isize) -> crate::Result {
        let endianness = Endianness::LSB;

        crate::file::seek(file_descriptor, 0);

        core::result::Result::Ok(crate::Ok::File(crate::file::Ok::Format(
            crate::file::format::Ok::Elf(crate::file::format::elf::Ok::Header(
                crate::file::format::elf::header::Ok::Identifier(Identifier {
                    magic0: T::read(file_descriptor, true).unwrap(),
                    magic1: T::read(file_descriptor, true).unwrap(),
                    magic2: T::read(file_descriptor, true).unwrap(),
                    magic3: T::read(file_descriptor, true).unwrap(),
                    class: T::read(file_descriptor, true).unwrap(),
                    endianness: T::read(file_descriptor, true).unwrap(),
                    version: T::read(file_descriptor, true).unwrap(),
                    osabi: T::read(file_descriptor, true).unwrap(),
                    abiversion: T::read(file_descriptor, true).unwrap(),
                    padding: T::read(file_descriptor, true).unwrap(),
                    unassigned0: T::read(file_descriptor, true).unwrap(),
                    unassigned1: T::read(file_descriptor, true).unwrap(),
                    unassigned2: T::read(file_descriptor, true).unwrap(),
                    unassigned3: T::read(file_descriptor, true).unwrap(),
                    unassigned4: T::read(file_descriptor, true).unwrap(),
                    nident: T::read(file_descriptor, true).unwrap(),
                }),
            )),
        )))
    }

    /// Each byte of the array is indexed symbolically using the names in the Table
    /// |Name|Value|Purpose|
    /// |:-:|:-:|:-:|
    /// |Mag0|0|File identification|
    /// |Mag1|1|File identification|
    /// |Mag2|2|File identification|
    /// |Mag3|3|File identification|
    /// |Class|4|File class|
    /// |Data|5|Data encoding|
    /// |Version|6|File version|
    /// |OsABI|7|OS/ABI identification|
    /// |AbiVersion|8|ABI version|
    /// |Pad|9|Start of paddingdbytes|
    /// |Unassigned|10|Meaningless bytes|
    /// |Unassigned|11|Meaningless bytes|
    /// |Unassigned|12|Meaningless bytes|
    /// |Unassigned|13|Meaningless bytes|
    /// |Unassigned|14|Meaningless bytes|
    /// |Unassigned|15|Meaningless bytes|
    /// |NIdent|16|Size of e_ident[]|
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

// // impl core::fmt::Debug for Identifier {
// //     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
// //         f.debug_struct("ELF Header Identifier")
// //             .field(
// //                 "Class",
// //                 &format!("{} ({})", self.class, self.get_class_str()),
// //             )
// //             .field(
// //                 "Data",
// //                 &format!("{} ({})", self.endianness, self.get_endianness_str()),
// //             )
// //             .field("Version", &self.copy_version())
// //             .field(
// //                 "OS ABI",
// //                 &format!("{} ({})", self.osabi, self.get_osabi_str()),
// //             )
// //             .field("ABI version", &self.copy_abi_version())
// //             .field("Padding", &self.copy_padding())
// //             .field("Unused", &self.get_unassigned_str())
// //             .field("NIdent", &self.nident)
// //             .finish()
// //     }
// // }
