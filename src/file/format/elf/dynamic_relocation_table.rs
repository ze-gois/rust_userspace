//! Relocation table resolved from the dynamic array.

use ample::r#type::Vec;

use super::{
    dynamic_symbol_table::DynamicSymbolTable,
    relocation::Relocation,
    symbol::Symbol,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Addend {
    Implicit,
    Explicit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Purpose {
    General,
    ProcedureLinkageTable,
}

#[derive(Debug)]
pub struct DynamicRelocationTable<'file> {
    pub relocations: Vec<Relocation>,
    pub symbols: DynamicSymbolTable<'file>,
    pub addend: Addend,
    pub purpose: Purpose,
}

impl<'file> DynamicRelocationTable<'file> {
    pub const fn new(
        relocations: Vec<Relocation>,
        symbols: DynamicSymbolTable<'file>,
        addend: Addend,
        purpose: Purpose,
    ) -> Self {
        Self {
            relocations,
            symbols,
            addend,
            purpose,
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

    pub fn symbol(&self, relocation: &Relocation) -> Option<&Symbol> {
        self.symbols.get(relocation.symbol_index as usize)
    }

    pub fn symbol_name(&self, relocation: &Relocation) -> Option<&'file str> {
        self.symbols.name(relocation.symbol_index as usize)
    }

    pub fn iter(&self) -> core::slice::Iter<'_, Relocation> {
        self.relocations.iter()
    }
}
