//! System V ELF symbol hash table.

use ample::r#type::Vec;

#[derive(Debug)]
pub struct HashTable {
    pub buckets: Vec<u32>,
    pub chains: Vec<u32>,
}

impl HashTable {
    pub const fn new(buckets: Vec<u32>, chains: Vec<u32>) -> Self {
        Self { buckets, chains }
    }

    pub fn find_index<F>(&self, name: &[u8], mut matches: F) -> Option<usize>
    where
        F: FnMut(usize, &[u8]) -> bool,
    {
        if self.buckets.is_empty() {
            return None;
        }

        let mut index = *self.buckets.get(hash(name) as usize % self.buckets.len())? as usize;

        while index != 0 {
            if matches(index, name) {
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
