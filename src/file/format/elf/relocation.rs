//! ELF relocation entries.

pub mod class_32;
pub mod class_64;
pub mod r#type;

pub use r#type::Type;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Relocation {
    pub offset: u64,
    pub symbol_index: u32,
    pub r#type: Type,
    pub addend: Option<i64>,
}
