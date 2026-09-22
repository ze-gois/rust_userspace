//! Symbol visibility (`STV_*`).

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Visibility {
    Default,
    Internal,
    Hidden,
    Protected,
}

impl Visibility {
    pub const fn from_raw(raw: u8) -> Self {
        match raw & 0x03 {
            0 => Self::Default,
            1 => Self::Internal,
            2 => Self::Hidden,
            _ => Self::Protected,
        }
    }

    pub const fn raw(self) -> u8 {
        match self {
            Self::Default => 0,
            Self::Internal => 1,
            Self::Hidden => 2,
            Self::Protected => 3,
        }
    }
}
