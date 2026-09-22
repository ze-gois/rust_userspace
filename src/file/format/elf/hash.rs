//! System V ELF symbol hash table.

use ample::r#type::Vec;

use super::symbol_table::SymbolTable;

#[derive(Debug)]
pub struct HashTable<'file> {
    pub buckets: Vec<u32>,
    pub chains: Vec<u32>,
    pub symbols: SymbolTable<'file>,
}

impl<'file> HashTable<'file> {
    pub const fn new(
        buckets: Vec<u32>,
        chains: Vec<u32>,
        symbols: SymbolTable<'file>,
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

        let mut index = *self.buckets.get(hash(name) as usize % self.buckets.len())? as usize;

        while index != 0 {
            let symbol_name = self.symbols.name(index)?.as_bytes();
            if symbol_name == name {
                return Some(index);
            }
            index = *self.chains.get(index)? as usize;
        }

        None
    }
}

/// System V ELF hash function.
pub fn hash(name: &[u8]) -> u32 {
    let mut hash = 0u32;

    for byte in name {
        if *byte == 0 {
            break;
        }

        hash = hash.wrapping_shl(4).wrapping_add(*byte as u32);
        let high = hash & 0xf000_0000;
        if high != 0 {
            hash ^= high >> 24;
        }
        hash &= !high;
    }

    hash
}
