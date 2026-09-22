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

#[derive(Debug, Clone, Copy)]
pub struct LinkOrder<'file> {
    pub metadata_index: usize,
    pub metadata: Section<'file>,
    pub referenced_index: usize,
    pub referenced: Section<'file>,
}

impl<'file> LinkOrder<'file> {
    pub const fn new(
        metadata_index: usize,
        metadata: Section<'file>,
        referenced_index: usize,
        referenced: Section<'file>,
    ) -> Self {
        Self {
            metadata_index,
            metadata,
            referenced_index,
            referenced,
        }
    }
}
