//! Encoding of the ELF header's `e_shstrndx` field.
//!
//! `SHN_XINDEX` means the actual index is stored in section-header entry
//! zero's `sh_link`.

use super::super::section_header::Index;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SectionNameStringTableIndex {
    Undefined,
    Direct(Index),
    Extended,
}

impl SectionNameStringTableIndex {
    pub const fn from_raw(raw: u16) -> Self {
        match raw {
            0 => Self::Undefined,
            0xffff => Self::Extended,
            value => Self::Direct(Index::from_raw(value)),
        }
    }

    pub const fn raw(self) -> u16 {
        match self {
            Self::Undefined => 0,
            Self::Direct(index) => index.raw(),
            Self::Extended => 0xffff,
        }
    }
}
