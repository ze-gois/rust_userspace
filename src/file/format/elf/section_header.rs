//! Section header table entries.
//!
//! A section header describes one section of an ELF object file. It is not
//! itself the section contents.

pub mod class_32;
pub mod class_64;
pub mod flags;
pub mod index;
pub mod r#type;

pub use flags::Flags;
pub use index::Index;
pub use r#type::Type;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SectionHeader {
    pub name_index: u32,
    pub r#type: Type,
    pub flags: Flags,
    pub address: u64,
    pub offset: u64,
    pub size: u64,
    pub link: u32,
    pub information: u32,
    pub alignment: u64,
    pub entry_size: u64,
}

impl From<class_32::Representation> for SectionHeader {
    fn from(representation: class_32::Representation) -> Self {
        Self {
            name_index: representation.sh_name,
            r#type: Type::from_raw(representation.sh_type),
            flags: Flags::from_raw(representation.sh_flags as u64),
            address: representation.sh_addr as u64,
            offset: representation.sh_offset as u64,
            size: representation.sh_size as u64,
            link: representation.sh_link,
            information: representation.sh_info,
            alignment: representation.sh_addralign as u64,
            entry_size: representation.sh_entsize as u64,
        }
    }
}

impl From<class_64::Representation> for SectionHeader {
    fn from(representation: class_64::Representation) -> Self {
        Self {
            name_index: representation.sh_name,
            r#type: Type::from_raw(representation.sh_type),
            flags: Flags::from_raw(representation.sh_flags),
            address: representation.sh_addr,
            offset: representation.sh_offset,
            size: representation.sh_size,
            link: representation.sh_link,
            information: representation.sh_info,
            alignment: representation.sh_addralign,
            entry_size: representation.sh_entsize,
        }
    }
}
