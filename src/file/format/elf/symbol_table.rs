//! ELF symbol table.
//!
//! A symbol table is a section whose entries are `Symbol` values.
//! The section header's `sh_link` identifies the associated string table;
//! `sh_info` identifies the first non-local symbol.

use ample::r#type::Vec;

use super::{
    string_table::StringTable,
    symbol::{Binding, ResolvedSectionIndex, Symbol, Type, Visibility},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationError {
    MissingUndefinedSymbol,
    InvalidUndefinedSymbol,
    FirstNonLocalIndexOutOfBounds { index: usize },
    NonLocalSymbolBeforeFirstNonLocal { index: usize },
    LocalSymbolAtOrAfterFirstNonLocal { index: usize },
}

#[derive(Debug)]
pub struct SymbolTable<'file> {
    pub symbols: Vec<Symbol>,
    pub strings: StringTable<'file>,
    pub section_indices: Vec<ResolvedSectionIndex>,
    pub first_non_local_index: usize,
}

impl<'file> SymbolTable<'file> {
    pub const fn new(
        symbols: Vec<Symbol>,
        strings: StringTable<'file>,
        section_indices: Vec<ResolvedSectionIndex>,
        first_non_local_index: usize,
    ) -> Self {
        Self {
            symbols,
            strings,
            section_indices,
            first_non_local_index,
        }
    }

    pub fn validate(&self) -> Result<(), ValidationError> {
        let undefined = self.symbols.first().ok_or(ValidationError::MissingUndefinedSymbol)?;

        if undefined.name_index != 0
            || undefined.value != 0
            || undefined.size != 0
            || !matches!(undefined.binding, Binding::Local)
            || !matches!(undefined.r#type, Type::None)
            || !matches!(undefined.visibility, Visibility::Default)
            || !matches!(self.section_indices.first(), Some(ResolvedSectionIndex::Undefined))
        {
            return Err(ValidationError::InvalidUndefinedSymbol);
        }

        if self.first_non_local_index > self.symbols.len() {
            return Err(ValidationError::FirstNonLocalIndexOutOfBounds {
                index: self.first_non_local_index,
            });
        }

        for (index, symbol) in self.symbols.iter().enumerate() {
            if index < self.first_non_local_index {
                if !matches!(symbol.binding, Binding::Local) {
                    return Err(ValidationError::NonLocalSymbolBeforeFirstNonLocal { index });
                }
            } else if matches!(symbol.binding, Binding::Local) {
                return Err(ValidationError::LocalSymbolAtOrAfterFirstNonLocal { index });
            }
        }

        Ok(())
    }

    pub fn section_index(&self, index: usize) -> Option<ResolvedSectionIndex> {
        self.section_indices.get(index).copied()
    }

    pub fn name(&self, index: usize) -> Option<&'file str> {
        let symbol = self.symbols.get(index)?;
        self.strings.get_str(symbol.name_index as usize)
    }

    pub fn local_symbols(&self) -> &[Symbol] {
        let end = self.first_non_local_index.min(self.symbols.len());
        &self.symbols[..end]
    }

    pub fn non_local_symbols(&self) -> &[Symbol] {
        let start = self.first_non_local_index.min(self.symbols.len());
        &self.symbols[start..]
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
