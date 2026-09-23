//! Compact relative relocation entries (`Elf32_Relr` / `Elf64_Relr`).

use ample::r#type::Vec;

use super::super::{
    base_address::BaseAddress,
    identification::{Class, Data},
    memory_image::{self, MemoryImage, MemoryImageWriter},
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
pub enum RepresentationError {
    ValueOutOfRange { value: i128 },
    UnsupportedData(Data),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplicationError {
    MemoryImage(memory_image::WriteError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StorageUnitWrite {
    pub link_time_virtual_address: u64,
    pub load_time_virtual_address: u64,
    pub representation: StorageUnitRepresentation,
}

impl StorageUnitWrite {
    pub fn apply(
        self,
        memory_image: &mut MemoryImageWriter<'_>,
    ) -> Result<(), ApplicationError> {
        memory_image
            .write(
                self.load_time_virtual_address,
                self.representation.bytes(),
            )
            .map_err(ApplicationError::MemoryImage)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BatchApplicationError {
    pub index: usize,
    pub error: ApplicationError,
}

pub fn apply_storage_unit_writes(
    writes: &[StorageUnitWrite],
    memory_image: &mut MemoryImageWriter<'_>,
) -> Result<(), BatchApplicationError> {
    for (index, write) in writes.iter().copied().enumerate() {
        memory_image
            .validate_write(
                write.load_time_virtual_address,
                write.representation.bytes().len(),
            )
            .map_err(|error| BatchApplicationError {
                index,
                error: ApplicationError::MemoryImage(error),
            })?;
    }

    for (index, write) in writes.iter().copied().enumerate() {
        write
            .apply(memory_image)
            .map_err(|error| BatchApplicationError { index, error })?;
    }

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageUnitRepresentation {
    Class32([u8; 4]),
    Class64([u8; 8]),
}

impl StorageUnitRepresentation {
    pub const fn bytes(&self) -> &[u8] {
        match self {
            Self::Class32(bytes) => bytes,
            Self::Class64(bytes) => bytes,
        }
    }

    pub const fn decode(self, data: Data) -> Result<StorageUnit, RepresentationError> {
        match (self, data) {
            (Self::Class32(bytes), Data::LeastSignificantByteFirst) => {
                Ok(StorageUnit::Class32(u32::from_le_bytes(bytes)))
            }
            (Self::Class32(bytes), Data::MostSignificantByteFirst) => {
                Ok(StorageUnit::Class32(u32::from_be_bytes(bytes)))
            }
            (Self::Class64(bytes), Data::LeastSignificantByteFirst) => {
                Ok(StorageUnit::Class64(u64::from_le_bytes(bytes)))
            }
            (Self::Class64(bytes), Data::MostSignificantByteFirst) => {
                Ok(StorageUnit::Class64(u64::from_be_bytes(bytes)))
            }
            (_, Data::None | Data::Reserved(_)) => Err(RepresentationError::UnsupportedData(data)),
        }
    }
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

    pub const fn relocated_value(self, factor: RelocationFactor) -> i128 {
        self.value() as i128 + factor.value()
    }

    pub fn relocated(self, factor: RelocationFactor) -> Result<Self, RepresentationError> {
        let value = self.relocated_value(factor);

        match self {
            Self::Class32(_) => {
                let value = representation_32::Address::try_from(value)
                    .map_err(|_| RepresentationError::ValueOutOfRange { value })?;
                Ok(Self::Class32(value))
            }
            Self::Class64(_) => {
                let value = representation_64::Address::try_from(value)
                    .map_err(|_| RepresentationError::ValueOutOfRange { value })?;
                Ok(Self::Class64(value))
            }
        }
    }

    pub const fn representation(
        self,
        data: Data,
    ) -> Result<StorageUnitRepresentation, RepresentationError> {
        match (self, data) {
            (Self::Class32(value), Data::LeastSignificantByteFirst) => {
                Ok(StorageUnitRepresentation::Class32(value.to_le_bytes()))
            }
            (Self::Class32(value), Data::MostSignificantByteFirst) => {
                Ok(StorageUnitRepresentation::Class32(value.to_be_bytes()))
            }
            (Self::Class64(value), Data::LeastSignificantByteFirst) => {
                Ok(StorageUnitRepresentation::Class64(value.to_le_bytes()))
            }
            (Self::Class64(value), Data::MostSignificantByteFirst) => {
                Ok(StorageUnitRepresentation::Class64(value.to_be_bytes()))
            }
            (_, Data::None | Data::Reserved(_)) => Err(RepresentationError::UnsupportedData(data)),
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

    pub const fn from_base_address(base_address: BaseAddress) -> Self {
        Self(base_address.value() as i128)
    }

    pub const fn value(self) -> i128 {
        self.0
    }

    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }

    pub fn relocate_virtual_address(
        self,
        link_time_virtual_address: u64,
    ) -> Result<u64, VirtualAddressError> {
        let value = link_time_virtual_address as i128 + self.0;
        u64::try_from(value)
            .map_err(|_| VirtualAddressError::OutOfRange { value })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VirtualAddressError {
    OutOfRange { value: i128 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WriteError {
    Expansion(ExpansionError),
    Representation(RepresentationError),
    VirtualAddress(VirtualAddressError),
    StorageUnitCountMismatch {
        addresses: usize,
        storage_units: usize,
    },
    StorageUnitClassMismatch {
        index: usize,
        class: Class,
    },
    StorageUnitUnavailable {
        index: usize,
        load_time_virtual_address: u64,
    },
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
    pub fn storage_unit_writes(
        &self,
        storage_units: &[StorageUnitRepresentation],
        data: Data,
        factor: RelocationFactor,
    ) -> Result<Vec<StorageUnitWrite>, WriteError> {
        let virtual_addresses = self
            .virtual_addresses()
            .map_err(WriteError::Expansion)?;

        if virtual_addresses.len() != storage_units.len() {
            return Err(WriteError::StorageUnitCountMismatch {
                addresses: virtual_addresses.len(),
                storage_units: storage_units.len(),
            });
        }

        let mut writes = Vec::with_capacity(virtual_addresses.len());

        for (index, (virtual_address, representation)) in virtual_addresses
            .into_iter()
            .zip(storage_units.iter().copied())
            .enumerate()
        {
            let class_matches = matches!(
                (self.class, representation),
                (Class::Class32, StorageUnitRepresentation::Class32(_))
                    | (Class::Class64, StorageUnitRepresentation::Class64(_))
            );
            if !class_matches {
                return Err(WriteError::StorageUnitClassMismatch {
                    index,
                    class: self.class,
                });
            }

            let storage_unit = representation
                .decode(data)
                .map_err(WriteError::Representation)?;
            let relocated = storage_unit
                .relocated(factor)
                .map_err(WriteError::Representation)?;
            let representation = relocated
                .representation(data)
                .map_err(WriteError::Representation)?;

            let load_time_virtual_address = factor
                .relocate_virtual_address(virtual_address)
                .map_err(WriteError::VirtualAddress)?;

            writes.push(StorageUnitWrite {
                link_time_virtual_address: virtual_address,
                load_time_virtual_address,
                representation,
            });
        }

        Ok(writes)
    }
    pub fn storage_unit_writes_from_memory_image(
        &self,
        memory_image: &MemoryImage<'_>,
        data: Data,
        factor: RelocationFactor,
    ) -> Result<Vec<StorageUnitWrite>, WriteError> {
        let link_time_virtual_addresses = self
            .virtual_addresses()
            .map_err(WriteError::Expansion)?;
        let storage_unit_size = match self.class {
            Class::Class32 => core::mem::size_of::<representation_32::Address>(),
            Class::Class64 => core::mem::size_of::<representation_64::Address>(),
            Class::None | Class::Reserved(_) => {
                return Err(WriteError::Expansion(ExpansionError::UnsupportedClass))
            }
        };

        let mut storage_units = Vec::with_capacity(link_time_virtual_addresses.len());

        for (index, link_time_virtual_address) in
            link_time_virtual_addresses.iter().copied().enumerate()
        {
            let load_time_virtual_address = factor
                .relocate_virtual_address(link_time_virtual_address)
                .map_err(WriteError::VirtualAddress)?;
            let bytes = memory_image
                .bytes(load_time_virtual_address, storage_unit_size)
                .ok_or(WriteError::StorageUnitUnavailable {
                    index,
                    load_time_virtual_address,
                })?;

            let representation = match self.class {
                Class::Class32 => StorageUnitRepresentation::Class32(
                    bytes
                        .try_into()
                        .expect("ELF32 storage-unit width established above"),
                ),
                Class::Class64 => StorageUnitRepresentation::Class64(
                    bytes
                        .try_into()
                        .expect("ELF64 storage-unit width established above"),
                ),
                Class::None | Class::Reserved(_) => unreachable!(),
            };
            storage_units.push(representation);
        }

        self.storage_unit_writes(&storage_units, data, factor)
    }

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
