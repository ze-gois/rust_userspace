//! ELF header.
//!
//! Serialized representations preserve the normative gABI field names.
//! `Header` exposes the same information using semantic Rust identifiers.

pub mod class_32;
pub mod class_64;
pub mod machine;
pub mod r#type;
pub mod section_header_count;
pub mod section_name_string_table_index;
pub mod version;
pub mod validation;

pub use machine::Machine;
pub use r#type::Type;
pub use section_header_count::SectionHeaderCount;
pub use section_name_string_table_index::SectionNameStringTableIndex;
pub use version::Version;
pub use validation::Error as ValidationError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Header {
    pub identification: super::identification::Identification,
    pub r#type: Type,
    pub machine: Machine,
    pub version: Version,
    pub entry: u64,
    pub program_header_offset: u64,
    pub section_header_offset: u64,
    pub flags: u32,
    pub header_size: u16,
    pub program_header_entry_size: u16,
    pub program_header_count: u16,
    pub section_header_entry_size: u16,
    pub section_header_count: SectionHeaderCount,
    pub section_name_string_table_index: SectionNameStringTableIndex,
}

impl TryFrom<class_32::Representation> for Header {
    type Error = ();

    fn try_from(representation: class_32::Representation) -> Result<Self, Self::Error> {
        let identification =
            super::identification::Identification::from_bytes(representation.e_ident)
                .ok_or(())?;

        Ok(Self {
            identification,
            r#type: Type::from_raw(representation.e_type),
            machine: Machine::from_raw(representation.e_machine),
            version: Version::from_raw(representation.e_version),
            entry: representation.e_entry as u64,
            program_header_offset: representation.e_phoff as u64,
            section_header_offset: representation.e_shoff as u64,
            flags: representation.e_flags,
            header_size: representation.e_ehsize,
            program_header_entry_size: representation.e_phentsize,
            program_header_count: representation.e_phnum,
            section_header_entry_size: representation.e_shentsize,
            section_header_count: SectionHeaderCount::from_raw(representation.e_shnum),
            section_name_string_table_index:
                SectionNameStringTableIndex::from_raw(representation.e_shstrndx),
        })
    }
}

impl TryFrom<class_64::Representation> for Header {
    type Error = ();

    fn try_from(representation: class_64::Representation) -> Result<Self, Self::Error> {
        let identification =
            super::identification::Identification::from_bytes(representation.e_ident)
                .ok_or(())?;

        Ok(Self {
            identification,
            r#type: Type::from_raw(representation.e_type),
            machine: Machine::from_raw(representation.e_machine),
            version: Version::from_raw(representation.e_version),
            entry: representation.e_entry,
            program_header_offset: representation.e_phoff,
            section_header_offset: representation.e_shoff,
            flags: representation.e_flags,
            header_size: representation.e_ehsize,
            program_header_entry_size: representation.e_phentsize,
            program_header_count: representation.e_phnum,
            section_header_entry_size: representation.e_shentsize,
            section_header_count: SectionHeaderCount::from_raw(representation.e_shnum),
            section_name_string_table_index:
                SectionNameStringTableIndex::from_raw(representation.e_shstrndx),
        })
    }
}
