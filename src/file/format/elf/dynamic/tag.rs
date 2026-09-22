//! Dynamic-array tag (`DT_*`).

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PayloadKind {
    Ignored,
    Value,
    Pointer,
    Unspecified,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tag {
    Null,
    Needed,
    ProcedureLinkageTableRelocationSize,
    ProcedureLinkageTableGlobalOffsetTable,
    Hash,
    StringTable,
    SymbolTable,
    RelocationWithAddend,
    RelocationWithAddendSize,
    RelocationWithAddendEntrySize,
    StringTableSize,
    SymbolEntrySize,
    Initialization,
    Termination,
    SharedObjectName,
    RuntimeSearchPath,
    Symbolic,
    Relocation,
    RelocationSize,
    RelocationEntrySize,
    ProcedureLinkageTableRelocation,
    Debug,
    TextRelocation,
    JumpRelocation,
    BindNow,
    InitializationArray,
    TerminationArray,
    InitializationArraySize,
    TerminationArraySize,
    RunPath,
    Flags,
    PreInitializationArray,
    PreInitializationArraySize,
    SymbolTableSectionIndex,
    RelativeRelocationSize,
    RelativeRelocation,
    RelativeRelocationEntrySize,
    OperatingSystemSpecific(i64),
    ProcessorSpecific(i64),
    Reserved(i64),
}

impl Tag {
    pub const fn from_raw(raw: i64) -> Self {
        match raw {
            0 => Self::Null,
            1 => Self::Needed,
            2 => Self::ProcedureLinkageTableRelocationSize,
            3 => Self::ProcedureLinkageTableGlobalOffsetTable,
            4 => Self::Hash,
            5 => Self::StringTable,
            6 => Self::SymbolTable,
            7 => Self::RelocationWithAddend,
            8 => Self::RelocationWithAddendSize,
            9 => Self::RelocationWithAddendEntrySize,
            10 => Self::StringTableSize,
            11 => Self::SymbolEntrySize,
            12 => Self::Initialization,
            13 => Self::Termination,
            14 => Self::SharedObjectName,
            15 => Self::RuntimeSearchPath,
            16 => Self::Symbolic,
            17 => Self::Relocation,
            18 => Self::RelocationSize,
            19 => Self::RelocationEntrySize,
            20 => Self::ProcedureLinkageTableRelocation,
            21 => Self::Debug,
            22 => Self::TextRelocation,
            23 => Self::JumpRelocation,
            24 => Self::BindNow,
            25 => Self::InitializationArray,
            26 => Self::TerminationArray,
            27 => Self::InitializationArraySize,
            28 => Self::TerminationArraySize,
            29 => Self::RunPath,
            30 => Self::Flags,
            32 => Self::PreInitializationArray,
            33 => Self::PreInitializationArraySize,
            34 => Self::SymbolTableSectionIndex,
            35 => Self::RelativeRelocationSize,
            36 => Self::RelativeRelocation,
            37 => Self::RelativeRelocationEntrySize,
            0x6000_000d..=0x6fff_f000 => Self::OperatingSystemSpecific(raw),
            0x7000_0000..=0x7fff_ffff => Self::ProcessorSpecific(raw),
            _ => Self::Reserved(raw),
        }
    }

    pub const fn payload_kind(self) -> PayloadKind {
        match self {
            Self::Null | Self::Symbolic | Self::TextRelocation | Self::BindNow => {
                PayloadKind::Ignored
            }
            Self::ProcedureLinkageTableGlobalOffsetTable
            | Self::Hash
            | Self::StringTable
            | Self::SymbolTable
            | Self::RelocationWithAddend
            | Self::Initialization
            | Self::Termination
            | Self::Relocation
            | Self::Debug
            | Self::JumpRelocation
            | Self::InitializationArray
            | Self::TerminationArray
            | Self::PreInitializationArray
            | Self::SymbolTableSectionIndex
            | Self::RelativeRelocation => PayloadKind::Pointer,
            Self::Needed
            | Self::ProcedureLinkageTableRelocationSize
            | Self::RelocationWithAddendSize
            | Self::RelocationWithAddendEntrySize
            | Self::StringTableSize
            | Self::SymbolEntrySize
            | Self::SharedObjectName
            | Self::RuntimeSearchPath
            | Self::RelocationSize
            | Self::RelocationEntrySize
            | Self::ProcedureLinkageTableRelocation
            | Self::InitializationArraySize
            | Self::TerminationArraySize
            | Self::RunPath
            | Self::Flags
            | Self::PreInitializationArraySize
            | Self::RelativeRelocationSize
            | Self::RelativeRelocationEntrySize => PayloadKind::Value,
            Self::OperatingSystemSpecific(_)
            | Self::ProcessorSpecific(_)
            | Self::Reserved(_) => PayloadKind::Unspecified,
        }
    }
}
