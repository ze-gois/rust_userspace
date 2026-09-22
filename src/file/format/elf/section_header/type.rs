//! Section type (`sh_type`).

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Null,
    ProgramBits,
    SymbolTable,
    StringTable,
    RelocationWithAddend,
    Hash,
    Dynamic,
    Note,
    NoBits,
    Relocation,
    SharedLibrary,
    DynamicSymbolTable,
    InitializationArray,
    TerminationArray,
    PreInitializationArray,
    Group,
    SymbolTableSectionIndex,
    OperatingSystemSpecific(u32),
    ProcessorSpecific(u32),
    ApplicationSpecific(u32),
    Reserved(u32),
}

impl Type {
    pub const fn from_raw(raw: u32) -> Self {
        match raw {
            0 => Self::Null,
            1 => Self::ProgramBits,
            2 => Self::SymbolTable,
            3 => Self::StringTable,
            4 => Self::RelocationWithAddend,
            5 => Self::Hash,
            6 => Self::Dynamic,
            7 => Self::Note,
            8 => Self::NoBits,
            9 => Self::Relocation,
            10 => Self::SharedLibrary,
            11 => Self::DynamicSymbolTable,
            14 => Self::InitializationArray,
            15 => Self::TerminationArray,
            16 => Self::PreInitializationArray,
            17 => Self::Group,
            18 => Self::SymbolTableSectionIndex,
            0x6000_0000..=0x6fff_ffff => Self::OperatingSystemSpecific(raw),
            0x7000_0000..=0x7fff_ffff => Self::ProcessorSpecific(raw),
            0x8000_0000..=0xffff_ffff => Self::ApplicationSpecific(raw),
            _ => Self::Reserved(raw),
        }
    }

    pub const fn raw(self) -> u32 {
        match self {
            Self::Null => 0,
            Self::ProgramBits => 1,
            Self::SymbolTable => 2,
            Self::StringTable => 3,
            Self::RelocationWithAddend => 4,
            Self::Hash => 5,
            Self::Dynamic => 6,
            Self::Note => 7,
            Self::NoBits => 8,
            Self::Relocation => 9,
            Self::SharedLibrary => 10,
            Self::DynamicSymbolTable => 11,
            Self::InitializationArray => 14,
            Self::TerminationArray => 15,
            Self::PreInitializationArray => 16,
            Self::Group => 17,
            Self::SymbolTableSectionIndex => 18,
            Self::OperatingSystemSpecific(raw)
            | Self::ProcessorSpecific(raw)
            | Self::ApplicationSpecific(raw)
            | Self::Reserved(raw) => raw,
        }
    }
}
