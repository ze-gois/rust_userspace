//! ELF program image described by `PT_LOAD` entries.
//!
//! This module represents what the ELF object requires in memory. It does not
//! choose operating-system mapping calls or translate `p_flags` into an OS
//! protection API.

use ample::r#type::Vec;

use super::{
    base_address::BaseAddress,
    memory_image::{OwnedMemoryImage, OwnedRegion},
    program_header::Flags,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    FileImageLargerThanMemoryImage { index: usize },
    FileImageUnavailable { index: usize },
    VirtualAddressRangeOverflow { index: usize },
    MemoryImageSizeUnsupported { index: usize, size: u64 },
    LoadTimeVirtualAddressOverflow { index: usize },
    NoLoadSegments,
    InvalidMaximumPageSize { size: u64 },
    BaseAddressUnderflow {
        memory_load_address: u64,
        lowest_link_time_virtual_address: u64,
        maximum_page_size: u64,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZeroFill {
    pub link_time_virtual_address: u64,
    pub size: u64,
}

impl ZeroFill {
    pub const fn new(link_time_virtual_address: u64, size: u64) -> Self {
        Self {
            link_time_virtual_address,
            size,
        }
    }

    pub const fn is_empty(self) -> bool {
        self.size == 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Segment<'file> {
    pub program_header_index: usize,
    pub link_time_virtual_address: u64,
    pub file_image: &'file [u8],
    pub zero_fill: ZeroFill,
    pub flags: Flags,
    pub alignment: u64,
    pub link_time_end_virtual_address: u64,
}

impl<'file> Segment<'file> {
    pub const fn memory_size(&self) -> u64 {
        self.file_image.len() as u64 + self.zero_fill.size
    }

    pub fn memory_image_bytes(&self) -> Result<Vec<u8>, Error> {
        let memory_size = self.memory_size();
        let capacity = usize::try_from(memory_size).map_err(|_| {
            Error::MemoryImageSizeUnsupported {
                index: self.program_header_index,
                size: memory_size,
            }
        })?;

        let mut bytes = Vec::with_capacity(capacity);
        bytes.extend(self.file_image.iter().copied());

        let zero_fill_size = capacity - self.file_image.len();
        bytes.extend(core::iter::repeat(0).take(zero_fill_size));

        Ok(bytes)
    }

    pub fn load_time_virtual_address(
        &self,
        base_address: BaseAddress,
    ) -> Result<u64, Error> {
        base_address
            .relocate_virtual_address(self.link_time_virtual_address)
            .ok_or(Error::LoadTimeVirtualAddressOverflow {
                index: self.program_header_index,
            })
    }

    pub fn zero_fill_load_time_virtual_address(
        &self,
        base_address: BaseAddress,
    ) -> Result<u64, Error> {
        base_address
            .relocate_virtual_address(self.zero_fill.link_time_virtual_address)
            .ok_or(Error::LoadTimeVirtualAddressOverflow {
                index: self.program_header_index,
            })
    }

    pub fn load_time_end_virtual_address(
        &self,
        base_address: BaseAddress,
    ) -> Result<u64, Error> {
        base_address
            .relocate_virtual_address(self.link_time_end_virtual_address)
            .ok_or(Error::LoadTimeVirtualAddressOverflow {
                index: self.program_header_index,
            })
    }
}

#[derive(Debug)]
pub struct ProgramImage<'file> {
    pub segments: Vec<Segment<'file>>,
}

impl<'file> ProgramImage<'file> {
    pub const fn new(segments: Vec<Segment<'file>>) -> Self {
        Self { segments }
    }

    pub fn len(&self) -> usize {
        self.segments.len()
    }

    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }

    pub fn iter(&self) -> core::slice::Iter<'_, Segment<'file>> {
        self.segments.iter()
    }

    pub fn lowest_link_time_virtual_address(&self) -> Option<u64> {
        self.segments
            .iter()
            .map(|segment| segment.link_time_virtual_address)
            .min()
    }

    pub fn base_address(
        &self,
        memory_load_address: u64,
        maximum_page_size: u64,
    ) -> Result<BaseAddress, Error> {
        if maximum_page_size == 0 || !maximum_page_size.is_power_of_two() {
            return Err(Error::InvalidMaximumPageSize {
                size: maximum_page_size,
            });
        }

        let lowest_link_time_virtual_address = self
            .lowest_link_time_virtual_address()
            .ok_or(Error::NoLoadSegments)?;

        BaseAddress::calculate(
            memory_load_address,
            lowest_link_time_virtual_address,
            maximum_page_size,
        )
        .ok_or(Error::BaseAddressUnderflow {
            memory_load_address,
            lowest_link_time_virtual_address,
            maximum_page_size,
        })
    }

    pub fn materialize_memory_image(
        &self,
        base_address: BaseAddress,
    ) -> Result<OwnedMemoryImage, Error> {
        let mut regions = Vec::with_capacity(self.segments.len());

        for segment in self.segments.iter() {
            let load_time_virtual_address =
                segment.load_time_virtual_address(base_address)?;
            let bytes = segment.memory_image_bytes()?;

            regions.push(OwnedRegion::new(
                load_time_virtual_address,
                bytes,
            ));
        }

        Ok(OwnedMemoryImage::new(regions))
    }
}
