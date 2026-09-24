//! Symbol type (`STT_*`).

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    None,
    Object,
    Function,
    Section,
    File,
    Common,
    ThreadLocalStorage,
    OperatingSystemSpecific(u8),
    ProcessorSpecific(u8),
    Reserved(u8),
}

impl Type {
    pub const fn from_raw(raw: u8) -> Self {
        match raw {
            0 => Self::None,
            1 => Self::Object,
            2 => Self::Function,
            3 => Self::Section,
            4 => Self::File,
            5 => Self::Common,
            6 => Self::ThreadLocalStorage,
            10..=12 => Self::OperatingSystemSpecific(raw),
            13..=15 => Self::ProcessorSpecific(raw),
            _ => Self::Reserved(raw),
        }
    }

    pub const fn raw(self) -> u8 {
        match self {
            Self::None => 0,
            Self::Object => 1,
            Self::Function => 2,
            Self::Section => 3,
            Self::File => 4,
            Self::Common => 5,
            Self::ThreadLocalStorage => 6,
            Self::OperatingSystemSpecific(raw)
            | Self::ProcessorSpecific(raw)
            | Self::Reserved(raw) => raw,
        }
    }
}
