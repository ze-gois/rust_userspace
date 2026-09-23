//! Dynamic linking symbol table resolved from the dynamic array.
//!
//! Unlike a section-based symbol table, this view does not depend on
//! `SHT_DYNSYM`, `sh_link`, or `sh_info`.

use ample::r#type::Vec;

use super::{
    string_table::StringTable,
    symbol::{Binding, ResolvedSectionIndex, Symbol, Type, Visibility},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationError {
    MissingUndefinedSymbol,
    InvalidUndefinedSymbol,
    UndefinedOtherBits { index: usize, bits: u8 },
    ReservedBinding { index: usize, raw: u8 },
    ReservedType { index: usize, raw: u8 },
    ReservedVisibility { index: usize, raw: u8 },
    InvalidNameIndex { index: usize, name_index: u32 },
    LocalProtectedVisibility { index: usize },
    FileSymbolNotLocal { index: usize },
    FileSymbolNotAbsolute { index: usize },
    ReservedSectionIndex { index: usize, raw: u16 },
}

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

    pub fn validate(&self) -> Result<(), ValidationError> {
        let undefined = self
            .symbols
            .first()
            .ok_or(ValidationError::MissingUndefinedSymbol)?;

        if undefined.name_index != 0
            || undefined.value != 0
            || undefined.size != 0
            || !matches!(undefined.binding, Binding::Local)
            || !matches!(undefined.r#type, Type::None)
            || !matches!(undefined.visibility, Visibility::Default)
            || undefined.other_bits != 0
            || !matches!(
                self.section_indices.first(),
                Some(ResolvedSectionIndex::Undefined)
            )
        {
            return Err(ValidationError::InvalidUndefinedSymbol);
        }

        for (index, symbol) in self.symbols.iter().enumerate() {
            if symbol.other_bits != 0 {
                return Err(ValidationError::UndefinedOtherBits {
                    index,
                    bits: symbol.other_bits,
                });
            }

            if let Binding::Reserved(raw) = symbol.binding {
                return Err(ValidationError::ReservedBinding { index, raw });
            }

            if let Type::Reserved(raw) = symbol.r#type {
                return Err(ValidationError::ReservedType { index, raw });
            }

            if let Visibility::Reserved(raw) = symbol.visibility {
                return Err(ValidationError::ReservedVisibility { index, raw });
            }

            if symbol.name_index != 0
                && self.strings.get(symbol.name_index as usize).is_none()
            {
                return Err(ValidationError::InvalidNameIndex {
                    index,
                    name_index: symbol.name_index,
                });
            }

            if matches!(symbol.binding, Binding::Local)
                && matches!(symbol.visibility, Visibility::Protected)
            {
                return Err(ValidationError::LocalProtectedVisibility { index });
            }

            if matches!(symbol.r#type, Type::File) {
                if !matches!(symbol.binding, Binding::Local) {
                    return Err(ValidationError::FileSymbolNotLocal { index });
                }

                if !matches!(
                    self.section_indices.get(index),
                    Some(ResolvedSectionIndex::Absolute)
                ) {
                    return Err(ValidationError::FileSymbolNotAbsolute { index });
                }
            }

            if let Some(ResolvedSectionIndex::Reserved(raw)) =
                self.section_indices.get(index).copied()
            {
                return Err(ValidationError::ReservedSectionIndex { index, raw });
            }
        }

        Ok(())
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
