//! ELF section contents.

use super::section_header::SectionHeader;

#[derive(Debug, Clone, Copy)]
pub struct Section<'file> {
    pub header: SectionHeader,
    pub contents: &'file [u8],
}

impl<'file> Section<'file> {
    pub const fn new(header: SectionHeader, contents: &'file [u8]) -> Self {
        Self { header, contents }
    }

    pub fn is_empty(&self) -> bool {
        self.contents.is_empty()
    }

    pub fn len(&self) -> usize {
        self.contents.len()
    }
}
