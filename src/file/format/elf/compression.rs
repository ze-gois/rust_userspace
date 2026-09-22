//! ELF compressed-section header.

pub mod class_32;
pub mod class_64;
pub mod r#type;

pub use r#type::Type;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompressionHeader {
    pub r#type: Type,
    pub uncompressed_size: u64,
    pub alignment: u64,
}
