//! ELF identification bytes (`e_ident`).

pub const SIZE: usize = 16;

pub const MAGIC_0_INDEX: usize = 0;
pub const MAGIC_1_INDEX: usize = 1;
pub const MAGIC_2_INDEX: usize = 2;
pub const MAGIC_3_INDEX: usize = 3;
pub const CLASS_INDEX: usize = 4;
pub const DATA_INDEX: usize = 5;
pub const VERSION_INDEX: usize = 6;
pub const OS_ABI_INDEX: usize = 7;
pub const ABI_VERSION_INDEX: usize = 8;
pub const PADDING_INDEX: usize = 9;
pub const PADDING_SIZE: usize = SIZE - PADDING_INDEX;

pub const MAGIC: [u8; 4] = [0x7f, b'E', b'L', b'F'];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Class {
    None,
    Class32,
    Class64,
    Reserved(u8),
}

impl Class {
    pub const fn from_raw(raw: u8) -> Self {
        match raw {
            0 => Self::None,
            1 => Self::Class32,
            2 => Self::Class64,
            _ => Self::Reserved(raw),
        }
    }

    pub const fn raw(self) -> u8 {
        match self {
            Self::None => 0,
            Self::Class32 => 1,
            Self::Class64 => 2,
            Self::Reserved(raw) => raw,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Data {
    None,
    LeastSignificantByteFirst,
    MostSignificantByteFirst,
    Reserved(u8),
}

impl Data {
    pub const fn from_raw(raw: u8) -> Self {
        match raw {
            0 => Self::None,
            1 => Self::LeastSignificantByteFirst,
            2 => Self::MostSignificantByteFirst,
            _ => Self::Reserved(raw),
        }
    }

    pub const fn raw(self) -> u8 {
        match self {
            Self::None => 0,
            Self::LeastSignificantByteFirst => 1,
            Self::MostSignificantByteFirst => 2,
            Self::Reserved(raw) => raw,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OperatingSystemAbi(u8);

impl OperatingSystemAbi {
    pub const fn from_raw(raw: u8) -> Self {
        Self(raw)
    }

    pub const fn raw(self) -> u8 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Identification {
    pub class: Class,
    pub data: Data,
    pub version: u8,
    pub operating_system_abi: OperatingSystemAbi,
    pub abi_version: u8,
    pub padding: [u8; PADDING_SIZE],
}

impl Identification {
    pub fn from_bytes(bytes: [u8; SIZE]) -> Option<Self> {
        if bytes[..4] != MAGIC {
            return None;
        }

        let padding = bytes[PADDING_INDEX..].try_into().ok()?;

        Some(Self {
            class: Class::from_raw(bytes[CLASS_INDEX]),
            data: Data::from_raw(bytes[DATA_INDEX]),
            version: bytes[VERSION_INDEX],
            operating_system_abi: OperatingSystemAbi::from_raw(bytes[OS_ABI_INDEX]),
            abi_version: bytes[ABI_VERSION_INDEX],
            padding,
        })
    }
}
