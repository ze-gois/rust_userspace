//! ELF symbol table.
//!
//! A symbol table is a section whose entries are `Symbol` values and whose
//! section-header `sh_link` identifies the associated string table.

use ample::r#type::Vec;

use super::{string_table::StringTable, symbol::Symbol};

#[derive(Debug)]
pub struct SymbolTable<'file> {
    pub symbols: Vec<Symbol>,
    pub strings: StringTable<'file>,
}

impl<'file> SymbolTable<'file> {
    pub const fn new(symbols: Vec<Symbol>, strings: StringTable<'file>) -> Self {
        Self { symbols, strings }
    }

    pub fn name(&self, index: usize) -> Option<&'file str> {
        let symbol = self.symbols.get(index)?;
        self.strings.get_str(symbol.name_index as usize)
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

    pub fn iter(&self) -> core::slice::Iter<'_, Symbol> {
        self.symbols.iter()
    }
}
