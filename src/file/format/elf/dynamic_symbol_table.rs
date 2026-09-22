//! Dynamic linking symbol table resolved from the dynamic array.
//!
//! Unlike a section-based symbol table, this view does not depend on
//! `SHT_DYNSYM`, `sh_link`, or `sh_info`.

use ample::r#type::Vec;

use super::{
    string_table::StringTable,
    symbol::{ResolvedSectionIndex, Symbol},
};

#[derive(Debug)]
pub struct DynamicSymbolTable<'file> {
    pub symbols: Vec<Symbol>,
    pub strings: StringTable<'file>,
    pub section_indices: Vec<ResolvedSectionIndex>,
}

impl<'file> DynamicSymbolTable<'file> {
    pub const fn new(
        symbols: Vec<Symbol>,
        strings: StringTable<'file>,
        section_indices: Vec<ResolvedSectionIndex>,
    ) -> Self {
        Self {
            symbols,
            strings,
            section_indices,
        }
    }

    pub fn len(&self) -> usize {
        self.symbols.len()
    }

    pub fn is_empty(&self) -> bool {
        self.symbols.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<&Symbol> {
        self.symbols.get(index)
    }

    pub fn name(&self, index: usize) -> Option<&'file str> {
        let symbol = self.get(index)?;
        self.strings.get_str(symbol.name_index as usize)
    }

    pub fn section_index(&self, index: usize) -> Option<ResolvedSectionIndex> {
        self.section_indices.get(index).copied()
    }

    pub fn iter(&self) -> core::slice::Iter<'_, Symbol> {
        self.symbols.iter()
    }
}
