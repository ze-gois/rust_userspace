//! Symbol binding (`STB_*`).

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Binding {
    Local,
    Global,
    Weak,
    OperatingSystemSpecific(u8),
    ProcessorSpecific(u8),
    Reserved(u8),
}

impl Binding {
    pub const fn from_raw(raw: u8) -> Self {
        match raw {
            0 => Self::Local,
            1 => Self::Global,
            2 => Self::Weak,
            10..=12 => Self::OperatingSystemSpecific(raw),
            13..=15 => Self::ProcessorSpecific(raw),
            _ => Self::Reserved(raw),
        }
    }

    pub const fn raw(self) -> u8 {
        match self {
            Self::Local => 0,
            Self::Global => 1,
            Self::Weak => 2,
            Self::OperatingSystemSpecific(raw)
            | Self::ProcessorSpecific(raw)
            | Self::Reserved(raw) => raw,
        }
    }
}
