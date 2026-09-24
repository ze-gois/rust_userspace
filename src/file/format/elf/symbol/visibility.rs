//! Symbol visibility (`STV_*`).

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Visibility {
    Default,
    Internal,
    Hidden,
    Protected,
    Exported,
    Singleton,
    Eliminate,
    Reserved(u8),
}

impl Visibility {
    pub const fn from_raw(raw: u8) -> Self {
        match raw & 0x07 {
            0 => Self::Default,
            1 => Self::Internal,
            2 => Self::Hidden,
            3 => Self::Protected,
            4 => Self::Exported,
            5 => Self::Singleton,
            6 => Self::Eliminate,
            raw => Self::Reserved(raw),
        }
    }

    pub const fn raw(self) -> u8 {
        match self {
            Self::Default => 0,
            Self::Internal => 1,
            Self::Hidden => 2,
            Self::Protected => 3,
            Self::Exported => 4,
            Self::Singleton => 5,
            Self::Eliminate => 6,
            Self::Reserved(raw) => raw,
        }
    }
}
