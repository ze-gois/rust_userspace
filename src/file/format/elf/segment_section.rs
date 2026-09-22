//! Relationship between an allocated section and a loadable segment.

use super::section_header::SectionHeader;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageContribution {
    FileAndMemory,
    MemoryOnly,
}

#[derive(Debug, Clone, Copy)]
pub struct SegmentSection {
    pub section_index: usize,
    pub section_header: SectionHeader,
    pub contribution: ImageContribution,
}

impl SegmentSection {
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
