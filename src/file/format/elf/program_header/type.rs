//! Program-header entry type (`p_type`).

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Null,
    Load,
    Dynamic,
    Interpreter,
    Note,
    SharedLibrary,
    ProgramHeader,
    ThreadLocalStorage,
    OperatingSystemSpecific(u32),
    ProcessorSpecific(u32),
    Reserved(u32),
}

impl Type {
    pub const OPERATING_SYSTEM_LOWER: u32 = 0x6000_0000;
    pub const OPERATING_SYSTEM_UPPER: u32 = 0x6fff_ffff;
    pub const PROCESSOR_LOWER: u32 = 0x7000_0000;
    pub const PROCESSOR_UPPER: u32 = 0x7fff_ffff;

    pub const fn from_raw(raw: u32) -> Self {
        match raw {
            0 => Self::Null,
            1 => Self::Load,
            2 => Self::Dynamic,
            3 => Self::Interpreter,
            4 => Self::Note,
            5 => Self::SharedLibrary,
            6 => Self::ProgramHeader,
            7 => Self::ThreadLocalStorage,
            Self::OPERATING_SYSTEM_LOWER..=Self::OPERATING_SYSTEM_UPPER => {
                Self::OperatingSystemSpecific(raw)
            }
            Self::PROCESSOR_LOWER..=Self::PROCESSOR_UPPER => Self::ProcessorSpecific(raw),
            _ => Self::Reserved(raw),
        }
    }

    pub const fn raw(self) -> u32 {
        match self {
            Self::Null => 0,
            Self::Load => 1,
            Self::Dynamic => 2,
            Self::Interpreter => 3,
            Self::Note => 4,
            Self::SharedLibrary => 5,
            Self::ProgramHeader => 6,
            Self::ThreadLocalStorage => 7,
            Self::OperatingSystemSpecific(raw)
            | Self::ProcessorSpecific(raw)
            | Self::Reserved(raw) => raw,
        }
    }
}
