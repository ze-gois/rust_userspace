//! ELF object-file version (`e_version`).

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Version {
    None,
    Current,
    Reserved(u32),
}

impl Version {
    pub const fn from_raw(raw: u32) -> Self {
        match raw {
            0 => Self::None,
            1 => Self::Current,
            _ => Self::Reserved(raw),
        }
    }

    pub const fn raw(self) -> u32 {
        match self {
            Self::None => 0,
            Self::Current => 1,
            Self::Reserved(raw) => raw,
        }
    }
}
