//! Read-only ELF memory image.
//!
//! A memory image exposes bytes at load-time virtual addresses. The read-only
//! and writing views are explicit and separate so that relocation planning does
//! not itself imply mutation.

use ample::r#type::Vec;

#[derive(Debug)]
pub struct OwnedRegion {
    pub virtual_address: u64,
    pub bytes: Vec<u8>,
}

impl OwnedRegion {
    pub const fn new(virtual_address: u64, bytes: Vec<u8>) -> Self {
        Self {
            virtual_address,
            bytes,
        }
    }
}

#[derive(Debug)]
pub struct OwnedMemoryImage {
    pub regions: Vec<OwnedRegion>,
}

impl OwnedMemoryImage {
    pub const fn new(regions: Vec<OwnedRegion>) -> Self {
        Self { regions }
    }

    pub fn as_memory_image(&self) -> MemoryImage<'_> {
        let mut regions = Vec::with_capacity(self.regions.len());

        for region in self.regions.iter() {
            regions.push(Region::new(
                region.virtual_address,
                region.bytes.as_slice(),
            ));
        }

        MemoryImage::new(regions)
    }

    pub fn as_memory_image_writer(&mut self) -> MemoryImageWriter<'_> {
        let mut regions = Vec::with_capacity(self.regions.len());

        for region in self.regions.iter_mut() {
            regions.push(RegionWriter::new(
                region.virtual_address,
                region.bytes.as_mut_slice(),
            ));
        }

        MemoryImageWriter::new(regions)
    }
}

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


#[derive(Debug)]
pub struct RegionWriter<'memory> {
    pub virtual_address: u64,
    pub bytes: &'memory mut [u8],
}

impl<'memory> RegionWriter<'memory> {
    pub fn new(virtual_address: u64, bytes: &'memory mut [u8]) -> Self {
        Self {
            virtual_address,
            bytes,
        }
    }

    pub fn contains_range(&self, virtual_address: u64, size: usize) -> bool {
        let Some(displacement) = virtual_address.checked_sub(self.virtual_address) else {
            return false;
        };
        let Ok(displacement) = usize::try_from(displacement) else {
            return false;
        };
        let Some(end) = displacement.checked_add(size) else {
            return false;
        };

        end <= self.bytes.len()
    }

    pub fn bytes_mut(
        &mut self,
        virtual_address: u64,
        size: usize,
    ) -> Option<&mut [u8]> {
        let displacement = virtual_address.checked_sub(self.virtual_address)?;
        let displacement = usize::try_from(displacement).ok()?;
        let end = displacement.checked_add(size)?;
        self.bytes.get_mut(displacement..end)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WriteError {
    Unavailable {
        virtual_address: u64,
        size: usize,
    },
}

#[derive(Debug)]
pub struct MemoryImageWriter<'memory> {
    pub regions: Vec<RegionWriter<'memory>>,
}

impl<'memory> MemoryImageWriter<'memory> {
    pub const fn new(regions: Vec<RegionWriter<'memory>>) -> Self {
        Self { regions }
    }

    pub fn validate_write(
        &self,
        virtual_address: u64,
        size: usize,
    ) -> Result<(), WriteError> {
        if self
            .regions
            .iter()
            .any(|region| region.contains_range(virtual_address, size))
        {
            return Ok(());
        }

        Err(WriteError::Unavailable {
            virtual_address,
            size,
        })
    }

    pub fn bytes_mut(
        &mut self,
        virtual_address: u64,
        size: usize,
    ) -> Option<&mut [u8]> {
        self.regions
            .iter_mut()
            .find_map(|region| region.bytes_mut(virtual_address, size))
    }

    pub fn write(
        &mut self,
        virtual_address: u64,
        bytes: &[u8],
    ) -> Result<(), WriteError> {
        self.validate_write(virtual_address, bytes.len())?;

        let destination = self
            .bytes_mut(virtual_address, bytes.len())
            .ok_or(WriteError::Unavailable {
                virtual_address,
                size: bytes.len(),
            })?;

        destination.copy_from_slice(bytes);
        Ok(())
    }
}
