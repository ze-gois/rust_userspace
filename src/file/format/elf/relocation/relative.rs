//! Compact relative relocation entries (`Elf32_Relr` / `Elf64_Relr`).

use super::super::{
    identification::Data,
    representation::{class_32 as representation_32, class_64 as representation_64, Decoder},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Entry {
    Address(u64),
    Bitmap(u64),
}

impl Entry {
    pub const fn from_raw(raw: u64) -> Self {
        if raw & 1 == 0 {
            Self::Address(raw)
        } else {
            Self::Bitmap(raw)
        }
    }

    pub const fn raw(self) -> u64 {
        match self {
            Self::Address(raw) | Self::Bitmap(raw) => raw,
        }
    }
}

pub mod class_32 {
    use super::{Data, Decoder, Entry, representation_32};

    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Representation(pub representation_32::Word);

    impl Representation {
        pub fn decode(bytes: &[u8], offset: usize, data: Data) -> Option<Self> {
            let mut decoder = Decoder::new(bytes, offset, data)?;
            Some(Self(decoder.word()?))
        }
    }

    impl From<Representation> for Entry {
        fn from(representation: Representation) -> Self {
            Self::from_raw(u64::from(representation.0))
        }
    }
}

pub mod class_64 {
    use super::{Data, Decoder, Entry, representation_64};

    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Representation(pub representation_64::Xword);

    impl Representation {
        pub fn decode(bytes: &[u8], offset: usize, data: Data) -> Option<Self> {
            let mut decoder = Decoder::new(bytes, offset, data)?;
            Some(Self(decoder.xword()?))
        }
    }

    impl From<Representation> for Entry {
        fn from(representation: Representation) -> Self {
            Self::from_raw(representation.0)
        }
    }
}
