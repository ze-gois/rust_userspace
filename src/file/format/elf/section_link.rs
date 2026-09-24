//! Section relationships expressed through `sh_link`.
//!
//! The meaning of `sh_link` depends on `sh_type`. This module models only
//! the generic relationships assigned by the System V ABI table for
//! `sh_link`; flag-specific relationships such as `SHF_LINK_ORDER` remain
//! separate.

use super::section::Section;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationError {
    LinkOutOfBounds {
        section_index: usize,
        linked_section_index: usize,
    },
    LinkNotStringTable {
        section_index: usize,
        linked_section_index: usize,
    },
    LinkNotSymbolTable {
        section_index: usize,
        linked_section_index: usize,
    },
    InformationMustBeZero {
        section_index: usize,
        information: u32,
    },
    RelocationTargetOutOfBounds {
        section_index: usize,
        target_section_index: usize,
    },
    GroupSignatureOutOfBounds {
        section_index: usize,
        symbol_index: usize,
    },
    LinkOrderTargetOutOfBounds {
        section_index: usize,
        target_section_index: usize,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Meaning {
    StringTable,
    SymbolTable,
}

#[derive(Debug, Clone, Copy)]
pub struct SectionLink<'file> {
    pub section_index: usize,
    pub linked_section_index: usize,
    pub linked_section: Section<'file>,
    pub meaning: Meaning,
}

impl<'file> SectionLink<'file> {
    pub const fn new(
        section_index: usize,
        linked_section_index: usize,
        linked_section: Section<'file>,
        meaning: Meaning,
    ) -> Self {
        Self {
            section_index,
            linked_section_index,
            linked_section,
            meaning,
        }
    }
}
