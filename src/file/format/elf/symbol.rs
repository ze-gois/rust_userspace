//! ELF symbol table entry.

pub mod binding;
pub mod class_32;
pub mod class_64;
pub mod r#type;
pub mod visibility;

pub use binding::Binding;
pub use r#type::Type;
pub use visibility::Visibility;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Symbol {
    pub name_index: u32,
    pub value: u64,
    pub size: u64,
    pub binding: Binding,
    pub r#type: Type,
    pub visibility: Visibility,
    pub section_index: super::section_header::Index,
}

impl Symbol {
    pub const fn information(binding: u8, r#type: u8) -> u8 {
        (binding << 4) + (r#type & 0x0f)
    }
}
