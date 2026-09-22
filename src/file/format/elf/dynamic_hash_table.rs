//! System V hash table resolved through the dynamic array.

use super::{
    dynamic_symbol_table::DynamicSymbolTable,
    hash::HashTable,
};

#[derive(Debug)]
pub struct DynamicHashTable<'file> {
    pub table: HashTable,
    pub symbols: DynamicSymbolTable<'file>,
}

impl<'file> DynamicHashTable<'file> {
    pub const fn new(table: HashTable, symbols: DynamicSymbolTable<'file>) -> Self {
        Self { table, symbols }
    }

    pub fn find(&self, name: &[u8]) -> Option<usize> {
        self.table.find_index(name, |index, expected| {
            self.symbols
                .name(index)
                .is_some_and(|actual| actual.as_bytes() == expected)
        })
    }
}
