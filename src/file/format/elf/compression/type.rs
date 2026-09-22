//! ELF compression algorithm (`ELFCOMPRESS_*`).

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Zlib,
    OperatingSystemSpecific(u32),
    ProcessorSpecific(u32),
    Reserved(u32),
}

impl Type {
    pub const fn from_raw(raw: u32) -> Self {
        match raw {
            1 => Self::Zlib,
            0x6000_0000..=0x6fff_ffff => Self::OperatingSystemSpecific(raw),
            0x7000_0000..=0x7fff_ffff => Self::ProcessorSpecific(raw),
            _ => Self::Reserved(raw),
        }
    }

    pub const fn raw(self) -> u32 {
        match self {
            Self::Zlib => 1,
            Self::OperatingSystemSpecific(raw)
            | Self::ProcessorSpecific(raw)
            | Self::Reserved(raw) => raw,
        }
    }
}
