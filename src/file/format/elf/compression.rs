//! ELF compressed section.
//!
//! A compressed section starts with a class-specific compression header.
//! The bytes following that header are encoded according to `ch_type`.

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

#[derive(Debug, Clone, Copy)]
pub struct CompressedSection<'file> {
    pub header: CompressionHeader,
    pub data: &'file [u8],
}

impl<'file> CompressedSection<'file> {
    pub const fn new(header: CompressionHeader, data: &'file [u8]) -> Self {
        Self { header, data }
    }
}
