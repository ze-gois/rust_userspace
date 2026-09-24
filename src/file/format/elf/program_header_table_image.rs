//! Program-header table image described by `PT_PHDR`.

use super::program_header::ProgramHeader;

#[derive(Debug, Clone, Copy)]
pub struct ProgramHeaderTableImage<'file> {
    pub program_header: ProgramHeader,
    pub bytes: &'file [u8],
}

impl<'file> ProgramHeaderTableImage<'file> {
    pub const fn new(program_header: ProgramHeader, bytes: &'file [u8]) -> Self {
        Self {
            program_header,
            bytes,
        }
    }
}
