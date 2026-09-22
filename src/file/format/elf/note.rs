//! ELF note information.
//!
//! A note is identified jointly by its originator name and type. The gABI does
//! not assign a universal meaning to the descriptor bytes.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Note<'file> {
    pub name: &'file [u8],
    pub r#type: u64,
    pub descriptor: &'file [u8],
}

impl<'file> Note<'file> {
    pub const fn new(name: &'file [u8], r#type: u64, descriptor: &'file [u8]) -> Self {
        Self {
            name,
            r#type,
            descriptor,
        }
    }
}
