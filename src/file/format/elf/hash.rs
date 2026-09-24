//! System V ELF symbol hash table.

use ample::r#type::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationError {
    BucketIndexOutOfBounds {
        bucket_index: usize,
        symbol_index: u32,
    },
    ChainIndexOutOfBounds {
        chain_index: usize,
        symbol_index: u32,
    },
    ChainCycle {
        bucket_index: usize,
        symbol_index: u32,
    },
}

#[derive(Debug)]
pub struct HashTable {
    pub buckets: Vec<u32>,
    pub chains: Vec<u32>,
}

impl HashTable {
    pub const fn new(buckets: Vec<u32>, chains: Vec<u32>) -> Self {
        Self { buckets, chains }
    }

    pub fn validate(&self, symbol_count: usize) -> Result<(), ValidationError> {
        for (bucket_index, symbol_index) in self.buckets.iter().copied().enumerate() {
            if symbol_index != 0
                && (symbol_index as usize >= symbol_count
                    || symbol_index as usize >= self.chains.len())
            {
                return Err(ValidationError::BucketIndexOutOfBounds {
                    bucket_index,
                    symbol_index,
                });
            }
        }

        for (chain_index, symbol_index) in self.chains.iter().copied().enumerate() {
            if symbol_index != 0
                && (symbol_index as usize >= symbol_count
                    || symbol_index as usize >= self.chains.len())
            {
                return Err(ValidationError::ChainIndexOutOfBounds {
                    chain_index,
                    symbol_index,
                });
            }
        }

        for (bucket_index, start) in self.buckets.iter().copied().enumerate() {
            if start == 0 {
                continue;
            }

            let mut slow = start;
            let mut fast = start;

            loop {
                slow = *self
                    .chains
                    .get(slow as usize)
                    .ok_or(ValidationError::BucketIndexOutOfBounds {
                        bucket_index,
                        symbol_index: slow,
                    })?;
                if slow == 0 {
                    break;
                }

                for _ in 0..2 {
                    fast = *self
                        .chains
                        .get(fast as usize)
                        .ok_or(ValidationError::BucketIndexOutOfBounds {
                            bucket_index,
                            symbol_index: fast,
                        })?;
                    if fast == 0 {
                        break;
                    }
                }

                if fast == 0 {
                    break;
                }

                if slow == fast {
                    return Err(ValidationError::ChainCycle {
                        bucket_index,
                        symbol_index: slow,
                    });
                }
            }
        }

        Ok(())
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
