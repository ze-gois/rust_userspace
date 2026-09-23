//! Compact relative relocation entries (`Elf32_Relr` / `Elf64_Relr`).

use ample::r#type::Vec;

use super::super::{
    identification::{Class, Data},
    representation::{class_32 as representation_32, class_64 as representation_64, Decoder},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SectionValidationError {
    ObjectTypeNotExecutableOrSharedObject,
    EntrySizeMismatch,
    SizeNotEntryMultiple,
    FirstEntryMustBeAddress,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageUnit {
    Class32(u32),
    Class64(u64),
}

impl StorageUnit {
    pub const fn value(self) -> u64 {
        match self {
            Self::Class32(value) => value as u64,
            Self::Class64(value) => value,
        }
    }

    pub const fn relocate(self, factor: RelocationFactor) -> Self {
        match self {
            Self::Class32(value) => {
                Self::Class32(value.wrapping_add(factor.value() as u32))
            }
            Self::Class64(value) => {
                Self::Class64(value.wrapping_add(factor.value() as u64))
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RelocationFactor(i128);

impl RelocationFactor {
    pub const fn from_virtual_addresses(
        actual_load_time_virtual_address: u64,
        link_time_virtual_address: u64,
    ) -> Self {
        Self(
            actual_load_time_virtual_address as i128
                - link_time_virtual_address as i128,
        )
    }

    pub const fn value(self) -> i128 {
        self.0
    }

    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpansionError {
    UnsupportedClass,
    BitmapWithoutAddress,
    VirtualAddressOverflow,
}

#[derive(Debug)]
pub struct Table {
    pub entries: Vec<Entry>,
    pub class: Class,
}

impl Table {
    pub const fn new(entries: Vec<Entry>, class: Class) -> Self {
        Self { entries, class }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn iter(&self) -> core::slice::Iter<'_, Entry> {
        self.entries.iter()
    }

    pub fn virtual_addresses(&self) -> Result<Vec<u64>, ExpansionError> {
        let (address_size, bitmap_storage_units) = match self.class {
            Class::Class32 => (4u64, 31u32),
            Class::Class64 => (8u64, 63u32),
            Class::None | Class::Reserved(_) => return Err(ExpansionError::UnsupportedClass),
        };

        let mut addresses = Vec::new();
        let mut next_address = None;

        for entry in self.entries.iter().copied() {
            match entry {
                Entry::Address(address) => {
                    addresses.push(address);
                    next_address = Some(
                        address
                            .checked_add(address_size)
                            .ok_or(ExpansionError::VirtualAddressOverflow)?,
                    );
                }
                Entry::Bitmap(bitmap) => {
                    let block_address =
                        next_address.ok_or(ExpansionError::BitmapWithoutAddress)?;

                    for bitmap_bit in 1..=bitmap_storage_units {
                        if bitmap & (1u64 << bitmap_bit) == 0 {
                            continue;
                        }

                        let storage_unit_index = u64::from(bitmap_bit - 1);
                        let displacement = storage_unit_index
                            .checked_mul(address_size)
                            .ok_or(ExpansionError::VirtualAddressOverflow)?;
                        let address = block_address
                            .checked_add(displacement)
                            .ok_or(ExpansionError::VirtualAddressOverflow)?;
                        addresses.push(address);
                    }

                    let block_size = u64::from(bitmap_storage_units)
                        .checked_mul(address_size)
                        .ok_or(ExpansionError::VirtualAddressOverflow)?;
                    next_address = Some(
                        block_address
                            .checked_add(block_size)
                            .ok_or(ExpansionError::VirtualAddressOverflow)?,
                    );
                }
            }
        }

        Ok(addresses)
    }
}

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
