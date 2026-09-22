//! System V ELF symbol hash table.

#[derive(Debug, Clone, Copy)]
pub struct HashTable<'file> {
    pub buckets: &'file [u32],
    pub chains: &'file [u32],
}

impl<'file> HashTable<'file> {
    pub const fn new(buckets: &'file [u32], chains: &'file [u32]) -> Self {
        Self { buckets, chains }
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
