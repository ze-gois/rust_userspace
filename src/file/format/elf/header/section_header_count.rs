//! Encoding of the ELF header's `e_shnum` field.
//!
//! A zero field means either that no section-header table exists or that the
//! actual count is stored in section-header entry zero's `sh_size`.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SectionHeaderCount {
    ZeroOrExtended,
    Direct(u16),
}

impl SectionHeaderCount {
    pub const fn from_raw(raw: u16) -> Self {
        if raw == 0 {
            Self::ZeroOrExtended
        } else {
            Self::Direct(raw)
        }
    }

    pub const fn raw(self) -> u16 {
        match self {
            Self::ZeroOrExtended => 0,
            Self::Direct(raw) => raw,
        }
    }
}
