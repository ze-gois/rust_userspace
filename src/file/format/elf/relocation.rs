//! ELF relocation entries.

pub mod class_32;
pub mod class_64;
pub mod r#type;
pub mod relative;

pub use r#type::Type;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationError {
    EntrySizeMismatch { expected: u64, actual: u64 },
    SizeNotEntryMultiple,
    MissingTargetSectionInRelocatableObject,
    SymbolIndexOutOfBounds {
        entry_index: usize,
        symbol_index: u32,
    },
    OffsetOutsideTargetSection {
        entry_index: usize,
        offset: u64,
        target_section_index: usize,
        target_size: u64,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Relocation {
    pub offset: u64,
    pub symbol_index: u32,
    pub r#type: Type,
    pub addend: Option<i64>,
}
