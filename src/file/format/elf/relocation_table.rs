//! ELF relocation table.
//!
//! A relocation section contains `Relocation` entries. Its section header
//! links to the associated symbol table through `sh_link` and identifies the
//! section to which the relocations apply through `sh_info`.

use ample::r#type::Vec;

use super::{
    relocation::Relocation,
    section::Section,
    symbol_table::SymbolTable,
};


#[derive(Debug, Clone, Copy)]
pub struct TargetSection<'file> {
    pub section_index: usize,
    pub section: Section<'file>,
}

impl<'file> TargetSection<'file> {
    pub const fn new(section_index: usize, section: Section<'file>) -> Self {
        Self {
            section_index,
            section,
        }
    }
}

#[derive(Debug)]
pub struct RelocationTable<'file> {
    pub relocations: Vec<Relocation>,
    pub symbols: SymbolTable<'file>,
    pub target_section_index: usize,
}

impl<'file> RelocationTable<'file> {
    pub const fn new(
        relocations: Vec<Relocation>,
        symbols: SymbolTable<'file>,
        target_section_index: usize,
    ) -> Self {
        Self {
            relocations,
            symbols,
            target_section_index,
        }
    }

    pub fn len(&self) -> usize {
        self.relocations.len()
    }

    pub fn is_empty(&self) -> bool {
        self.relocations.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<&Relocation> {
        self.relocations.get(index)
    }

    pub fn symbol(&self, relocation: &Relocation) -> Option<&super::symbol::Symbol> {
        self.symbols.get(relocation.symbol_index as usize)
    }

    pub fn symbol_name(&self, relocation: &Relocation) -> Option<&'file str> {
        self.symbols.name(relocation.symbol_index as usize)
    }

    pub fn iter(&self) -> core::slice::Iter<'_, Relocation> {
        self.relocations.iter()
    }
}
