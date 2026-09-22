//! Program header table entries.
//!
//! A program header entry describes a segment or other information required
//! to prepare a program for execution. It is not itself the segment.

pub mod class_32;
pub mod class_64;
pub mod flags;
pub mod r#type;
pub mod validation;

pub use flags::Flags;
pub use r#type::Type;
pub use validation::Error as ValidationError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProgramHeader {
    pub r#type: Type,
    pub flags: Flags,
    pub offset: u64,
    pub virtual_address: u64,
    pub physical_address: u64,
    pub file_size: u64,
    pub memory_size: u64,
    pub alignment: u64,
}

impl From<class_32::Representation> for ProgramHeader {
    fn from(representation: class_32::Representation) -> Self {
        Self {
            r#type: Type::from_raw(representation.p_type),
            flags: Flags::from_raw(representation.p_flags),
            offset: representation.p_offset as u64,
            virtual_address: representation.p_vaddr as u64,
            physical_address: representation.p_paddr as u64,
            file_size: representation.p_filesz as u64,
            memory_size: representation.p_memsz as u64,
            alignment: representation.p_align as u64,
        }
    }
}

impl From<class_64::Representation> for ProgramHeader {
    fn from(representation: class_64::Representation) -> Self {
        Self {
            r#type: Type::from_raw(representation.p_type),
            flags: Flags::from_raw(representation.p_flags),
            offset: representation.p_offset,
            virtual_address: representation.p_vaddr,
            physical_address: representation.p_paddr,
            file_size: representation.p_filesz,
            memory_size: representation.p_memsz,
            alignment: representation.p_align,
        }
    }
}
