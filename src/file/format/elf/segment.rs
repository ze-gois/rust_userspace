//! ELF segment contents.

use super::program_header::ProgramHeader;

#[derive(Debug, Clone, Copy)]
pub struct Segment<'file> {
    pub program_header: ProgramHeader,
    pub file_image: &'file [u8],
}

impl<'file> Segment<'file> {
    pub const fn new(program_header: ProgramHeader, file_image: &'file [u8]) -> Self {
        Self {
            program_header,
            file_image,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.file_image.is_empty()
    }

    pub fn len(&self) -> usize {
        self.file_image.len()
    }

    pub const fn memory_size(&self) -> u64 {
        self.program_header.memory_size
    }
}
