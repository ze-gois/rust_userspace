//! ELF object-file type (`e_type`).

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    None,
    Relocatable,
    Executable,
    SharedObject,
    Core,
    OperatingSystemSpecific(u16),
    ProcessorSpecific(u16),
    Reserved(u16),
}

impl Type {
    pub const fn from_raw(raw: u16) -> Self {
        match raw {
            0 => Self::None,
            1 => Self::Relocatable,
            2 => Self::Executable,
            3 => Self::SharedObject,
            4 => Self::Core,
            0xfe00..=0xfeff => Self::OperatingSystemSpecific(raw),
            0xff00..=0xffff => Self::ProcessorSpecific(raw),
            _ => Self::Reserved(raw),
        }
    }

    pub const fn raw(self) -> u16 {
        match self {
            Self::None => 0,
            Self::Relocatable => 1,
            Self::Executable => 2,
            Self::SharedObject => 3,
            Self::Core => 4,
            Self::OperatingSystemSpecific(raw)
            | Self::ProcessorSpecific(raw)
            | Self::Reserved(raw) => raw,
        }
    }
}
