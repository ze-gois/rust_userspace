//! ELF section group.

use ample::r#type::Vec;

use super::{
    section::Section,
    symbol_table::SymbolTable,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationError {
    GroupOutsideRelocatableObject { index: usize },
    GroupSectionFlagsNotZero { index: usize },
    MemberOutsideRelocatableObject { index: usize },
    MemberNotAfterGroup { group_index: usize, member_index: usize },
    MemberMissingGroupFlag { group_index: usize, member_index: usize },
    MemberInMultipleGroups { member_index: usize },
    GroupFlagWithoutGroup { member_index: usize },
    InvalidGroup { index: usize },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Flags(u32);

impl Flags {
    pub const COMDAT: u32 = 0x1;
    pub const OPERATING_SYSTEM_MASK: u32 = 0x0ff0_0000;
    pub const PROCESSOR_MASK: u32 = 0xf000_0000;

    pub const fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    pub const fn raw(self) -> u32 {
        self.0
    }

    pub const fn comdat(self) -> bool {
        self.0 & Self::COMDAT != 0
    }
}


#[derive(Debug, Clone, Copy)]
pub struct Member<'file> {
    pub section_index: usize,
    pub section: Section<'file>,
}

impl<'file> Member<'file> {
    pub const fn new(section_index: usize, section: Section<'file>) -> Self {
        Self {
            section_index,
            section,
        }
    }
}

#[derive(Debug)]
pub struct SectionGroup<'file> {
    pub flags: Flags,
    pub members: Vec<usize>,
    pub symbols: SymbolTable<'file>,
    pub signature_symbol_index: usize,
}

impl<'file> SectionGroup<'file> {
    pub const fn new(
        flags: Flags,
        members: Vec<usize>,
        symbols: SymbolTable<'file>,
        signature_symbol_index: usize,
    ) -> Self {
        Self {
            flags,
            members,
            symbols,
            signature_symbol_index,
        }
    }

    pub fn signature_symbol(&self) -> Option<&super::symbol::Symbol> {
        self.symbols.get(self.signature_symbol_index)
    }

    pub fn signature_name(&self) -> Option<&'file str> {
        self.symbols.name(self.signature_symbol_index)
    }
}
