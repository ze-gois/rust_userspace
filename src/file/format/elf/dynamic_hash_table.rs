//! System V hash table resolved through the dynamic array.

use ample::r#type::Vec;

use super::dynamic_symbol_table::DynamicSymbolTable;

#[derive(Debug)]
pub struct DynamicHashTable<'file> {
    pub buckets: Vec<u32>,
    pub chains: Vec<u32>,
    pub symbols: DynamicSymbolTable<'file>,
}

impl<'file> DynamicHashTable<'file> {
    pub const fn new(
        buckets: Vec<u32>,
        chains: Vec<u32>,
        symbols: DynamicSymbolTable<'file>,
    ) -> Self {
        Self {
            buckets,
            chains,
            symbols,
        }
    }

    pub fn find(&self, name: &[u8]) -> Option<usize> {
        if self.buckets.is_empty() {
            return None;
        }

        let mut index =
            *self.buckets.get(super::hash::hash(name) as usize % self.buckets.len())? as usize;

        while index != 0 {
            if self.symbols.name(index)?.as_bytes() == name {
                return Some(index);
            }
            index = *self.chains.get(index)? as usize;
        }

        None
    }
}
