//! Sections contained in an ELF segment.

use super::super::section_header::SectionHeader;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageContribution {
    FileAndMemory,
    MemoryOnly,
}

#[derive(Debug, Clone, Copy)]
pub struct Section {
    pub section_index: usize,
    pub section_header: SectionHeader,
    pub contribution: ImageContribution,
}

impl Section {
    pub const fn new(
        section_index: usize,
        section_header: SectionHeader,
        contribution: ImageContribution,
    ) -> Self {
        Self {
            section_index,
            section_header,
            contribution,
        }
    }
}
