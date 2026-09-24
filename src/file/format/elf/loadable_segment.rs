//! ELF loadable segment.
//!
//! A `PT_LOAD` entry describes a file image and a memory image. When
//! `p_memsz` exceeds `p_filesz`, the remaining memory-image bytes are
//! defined to be zero.

use super::program_header::ProgramHeader;

#[derive(Debug, Clone, Copy)]
pub struct LoadableSegment<'file> {
    pub program_header: ProgramHeader,
    pub file_image: &'file [u8],
}

impl<'file> LoadableSegment<'file> {
    pub const fn new(program_header: ProgramHeader, file_image: &'file [u8]) -> Self {
        Self {
            program_header,
            file_image,
        }
    }

    pub const fn memory_size(&self) -> u64 {
        self.program_header.memory_size
    }

    pub const fn file_size(&self) -> u64 {
        self.program_header.file_size
    }

    pub const fn zero_fill_size(&self) -> u64 {
        self.program_header.memory_size - self.program_header.file_size
    }

    pub const fn alignment(&self) -> u64 {
        self.program_header.alignment
    }
}
