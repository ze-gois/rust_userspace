//! Read-only ELF memory image.
//!
//! A memory image exposes bytes at load-time virtual addresses. Construction
//! and mutation belong to later loading stages; this view only provides the
//! bytes already present in mapped regions.

use ample::r#type::Vec;

#[derive(Debug, Clone, Copy)]
pub struct Region<'memory> {
    pub virtual_address: u64,
    pub bytes: &'memory [u8],
}

impl<'memory> Region<'memory> {
    pub const fn new(virtual_address: u64, bytes: &'memory [u8]) -> Self {
        Self {
            virtual_address,
            bytes,
        }
    }

    pub fn bytes(&self, virtual_address: u64, size: usize) -> Option<&'memory [u8]> {
        let displacement = virtual_address.checked_sub(self.virtual_address)?;
        let displacement = usize::try_from(displacement).ok()?;
        let end = displacement.checked_add(size)?;
        self.bytes.get(displacement..end)
    }
}

#[derive(Debug)]
pub struct MemoryImage<'memory> {
    pub regions: Vec<Region<'memory>>,
}

impl<'memory> MemoryImage<'memory> {
    pub const fn new(regions: Vec<Region<'memory>>) -> Self {
        Self { regions }
    }

    pub fn bytes(&self, virtual_address: u64, size: usize) -> Option<&'memory [u8]> {
        self.regions
            .iter()
            .find_map(|region| region.bytes(virtual_address, size))
    }
}
