//! ELF Thread-Local Storage template.
//!
//! A `PT_TLS` entry describes the TLS initialization image and the total
//! template size. Bytes beyond the initialization image up to `p_memsz`
//! belong to the zero-initialized portion of the template.

use super::program_header::ProgramHeader;

#[derive(Debug, Clone, Copy)]
pub struct Template<'file> {
    pub program_header: ProgramHeader,
    pub initialization_image: &'file [u8],
}

impl<'file> Template<'file> {
    pub const fn new(
        program_header: ProgramHeader,
        initialization_image: &'file [u8],
    ) -> Self {
        Self {
            program_header,
            initialization_image,
        }
    }

    pub const fn total_size(&self) -> u64 {
        self.program_header.memory_size
    }

    pub const fn initialization_size(&self) -> u64 {
        self.program_header.file_size
    }

    pub const fn zero_fill_size(&self) -> u64 {
        self.program_header.memory_size - self.program_header.file_size
    }

    pub const fn alignment(&self) -> u64 {
        self.program_header.alignment
    }
}
