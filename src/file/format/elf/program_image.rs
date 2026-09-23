//! ELF program image described by `PT_LOAD` entries.
//!
//! This module represents what the ELF object requires in memory. It does not
//! choose operating-system mapping calls or translate `p_flags` into an OS
//! protection API.

use ample::r#type::Vec;

use super::program_header::Flags;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    FileImageLargerThanMemoryImage { index: usize },
    FileImageUnavailable { index: usize },
    VirtualAddressRangeOverflow { index: usize },
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
}
