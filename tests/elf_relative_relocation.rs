use userspace::file::format::elf::{
    dynamic::{PayloadKind, Tag},
    identification::{Class, Data},
    memory_image::{
        MemoryImage, MemoryImageWriter, Region, RegionWriter,
        WriteError as MemoryImageWriteError,
    },
    relocation::relative::{
        class_32, class_64, Entry, ExpansionError, RelocationFactor, RepresentationError,
        apply_storage_unit_writes, ApplicationError, BatchApplicationError, StorageUnit,
        StorageUnitRepresentation, StorageUnitWrite, Table, VirtualAddressError, WriteError,
    },
    section_header,
};

#[test]
fn recognizes_relative_relocation_dynamic_tags() {
    assert_eq!(Tag::from_raw(35), Tag::RelativeRelocationSize);
    assert_eq!(Tag::from_raw(36), Tag::RelativeRelocation);
    assert_eq!(Tag::from_raw(37), Tag::RelativeRelocationEntrySize);

    assert_eq!(
        Tag::RelativeRelocationSize.payload_kind(),
        PayloadKind::Value
    );
    assert_eq!(
        Tag::RelativeRelocation.payload_kind(),
        PayloadKind::Pointer
    );
    assert_eq!(
        Tag::RelativeRelocationEntrySize.payload_kind(),
        PayloadKind::Value
    );
}

#[test]
fn recognizes_relative_relocation_section_type() {
    assert_eq!(
        section_header::Type::from_raw(19),
        section_header::Type::RelativeRelocation
    );
    assert_eq!(section_header::Type::RelativeRelocation.raw(), 19);
}

#[test]
fn decodes_elf32_relative_relocation_entries_by_byte_order() {
    let address = class_32::Representation::decode(
        &0x1234_5678u32.to_le_bytes(),
        0,
        Data::LeastSignificantByteFirst,
    )
    .expect("little-endian Elf32_Relr must decode");
    let bitmap = class_32::Representation::decode(
        &0x8000_0003u32.to_be_bytes(),
        0,
        Data::MostSignificantByteFirst,
    )
    .expect("big-endian Elf32_Relr must decode");

    assert_eq!(Entry::from(address), Entry::Address(0x1234_5678));
    assert_eq!(Entry::from(bitmap), Entry::Bitmap(0x8000_0003));
}

#[test]
fn decodes_elf64_relative_relocation_entries_by_byte_order() {
    let address = class_64::Representation::decode(
        &0x0123_4567_89ab_cde0u64.to_le_bytes(),
        0,
        Data::LeastSignificantByteFirst,
    )
    .expect("little-endian Elf64_Relr must decode");
    let bitmap = class_64::Representation::decode(
        &0x8000_0000_0000_0003u64.to_be_bytes(),
        0,
        Data::MostSignificantByteFirst,
    )
    .expect("big-endian Elf64_Relr must decode");

    assert_eq!(
        Entry::from(address),
        Entry::Address(0x0123_4567_89ab_cde0)
    );
    assert_eq!(
        Entry::from(bitmap),
        Entry::Bitmap(0x8000_0000_0000_0003)
    );
}


#[test]
fn expands_elf32_relative_relocation_bitmaps() {
    let table = Table::new(
        vec![
            Entry::Address(0x1000),
            Entry::Bitmap(0b1011),
            Entry::Bitmap(0b11),
        ],
        Class::Class32,
    );

    assert_eq!(
        table.virtual_addresses(),
        Ok(vec![
            0x1000,
            0x1004,
            0x100c,
            0x1080,
        ]),
    );
}

#[test]
fn expands_elf64_relative_relocation_bitmaps() {
    let table = Table::new(
        vec![
            Entry::Address(0x400000),
            Entry::Bitmap(0b111),
            Entry::Address(0x500000),
            Entry::Bitmap(0b101),
        ],
        Class::Class64,
    );

    assert_eq!(
        table.virtual_addresses(),
        Ok(vec![
            0x400000,
            0x400008,
            0x400010,
            0x500000,
            0x500010,
        ]),
    );
}

#[test]
fn rejects_relative_relocation_bitmap_without_address() {
    let table = Table::new(vec![Entry::Bitmap(3)], Class::Class64);

    assert_eq!(
        table.virtual_addresses(),
        Err(ExpansionError::BitmapWithoutAddress),
    );
}

#[test]
fn rejects_relative_relocation_address_expansion_overflow() {
    let table = Table::new(
        vec![Entry::Address(u64::MAX - 1)],
        Class::Class64,
    );

    assert_eq!(
        table.virtual_addresses(),
        Err(ExpansionError::VirtualAddressOverflow),
    );
}


#[test]
fn computes_positive_relative_relocation_factor() {
    let factor = RelocationFactor::from_virtual_addresses(0x500000, 0x400000);

    assert_eq!(factor.value(), 0x100000);
    assert!(!factor.is_zero());
}

#[test]
fn computes_zero_relative_relocation_factor() {
    let factor = RelocationFactor::from_virtual_addresses(0x400000, 0x400000);

    assert_eq!(factor.value(), 0);
    assert!(factor.is_zero());
}

#[test]
fn computes_negative_relative_relocation_factor_without_loss() {
    let factor = RelocationFactor::from_virtual_addresses(0x1000, 0xffff_ffff_ffff_f000);

    assert_eq!(
        factor.value(),
        0x1000i128 - 0xffff_ffff_ffff_f000u64 as i128,
    );
    assert!(!factor.is_zero());
}


#[test]
fn computes_elf32_relative_relocation_storage_unit_value() {
    let factor = RelocationFactor::from_virtual_addresses(0x5000, 0x4000);
    let storage_unit = StorageUnit::Class32(0x2000);

    assert_eq!(storage_unit.relocated_value(factor), 0x3000);
}

#[test]
fn computes_elf64_relative_relocation_storage_unit_value_with_negative_factor() {
    let factor = RelocationFactor::from_virtual_addresses(0x3000, 0x5000);
    let storage_unit = StorageUnit::Class64(0x9000);

    assert_eq!(storage_unit.relocated_value(factor), 0x7000);
}

#[test]
fn preserves_full_relative_relocation_result_before_storage_write() {
    let factor = RelocationFactor::from_virtual_addresses(8, 0);

    assert_eq!(
        StorageUnit::Class32(u32::MAX).relocated_value(factor),
        u32::MAX as i128 + 8,
    );
    assert_eq!(
        StorageUnit::Class64(u64::MAX).relocated_value(factor),
        u64::MAX as i128 + 8,
    );
}

#[test]
fn zero_relative_relocation_factor_preserves_storage_unit_value() {
    let factor = RelocationFactor::from_virtual_addresses(0x400000, 0x400000);

    assert_eq!(
        StorageUnit::Class32(0x1234_5678).relocated_value(factor),
        0x1234_5678,
    );
    assert_eq!(
        StorageUnit::Class64(0x1234_5678_9abc_def0).relocated_value(factor),
        0x1234_5678_9abc_def0,
    );
}


#[test]
fn represents_relocated_elf32_storage_unit() {
    let factor = RelocationFactor::from_virtual_addresses(0x5000, 0x4000);
    let storage_unit = StorageUnit::Class32(0x2000);

    assert_eq!(
        storage_unit.relocated(factor),
        Ok(StorageUnit::Class32(0x3000)),
    );
}

#[test]
fn represents_relocated_elf64_storage_unit() {
    let factor = RelocationFactor::from_virtual_addresses(0x3000, 0x5000);
    let storage_unit = StorageUnit::Class64(0x9000);

    assert_eq!(
        storage_unit.relocated(factor),
        Ok(StorageUnit::Class64(0x7000)),
    );
}

#[test]
fn rejects_relocated_elf32_value_above_address_width() {
    let factor = RelocationFactor::from_virtual_addresses(1, 0);
    let storage_unit = StorageUnit::Class32(u32::MAX);

    assert_eq!(
        storage_unit.relocated(factor),
        Err(RepresentationError::ValueOutOfRange {
            value: u32::MAX as i128 + 1,
        }),
    );
}

#[test]
fn rejects_negative_relocated_storage_unit_value() {
    let factor = RelocationFactor::from_virtual_addresses(0, 1);
    let storage_unit = StorageUnit::Class64(0);

    assert_eq!(
        storage_unit.relocated(factor),
        Err(RepresentationError::ValueOutOfRange { value: -1 }),
    );
}

#[test]
fn zero_factor_representation_preserves_storage_unit() {
    let factor = RelocationFactor::from_virtual_addresses(0x400000, 0x400000);

    assert_eq!(
        StorageUnit::Class32(0x1234_5678).relocated(factor),
        Ok(StorageUnit::Class32(0x1234_5678)),
    );
    assert_eq!(
        StorageUnit::Class64(0x1234_5678_9abc_def0).relocated(factor),
        Ok(StorageUnit::Class64(0x1234_5678_9abc_def0)),
    );
}


#[test]
fn represents_elf32_storage_unit_little_endian() {
    let storage_unit = StorageUnit::Class32(0x1234_5678);

    assert_eq!(
        storage_unit.representation(Data::LeastSignificantByteFirst),
        Ok(StorageUnitRepresentation::Class32([0x78, 0x56, 0x34, 0x12])),
    );
}

#[test]
fn represents_elf32_storage_unit_big_endian() {
    let storage_unit = StorageUnit::Class32(0x1234_5678);

    assert_eq!(
        storage_unit.representation(Data::MostSignificantByteFirst),
        Ok(StorageUnitRepresentation::Class32([0x12, 0x34, 0x56, 0x78])),
    );
}

#[test]
fn represents_elf64_storage_unit_little_and_big_endian() {
    let storage_unit = StorageUnit::Class64(0x0123_4567_89ab_cdef);

    assert_eq!(
        storage_unit.representation(Data::LeastSignificantByteFirst),
        Ok(StorageUnitRepresentation::Class64([
            0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23, 0x01,
        ])),
    );
    assert_eq!(
        storage_unit.representation(Data::MostSignificantByteFirst),
        Ok(StorageUnitRepresentation::Class64([
            0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef,
        ])),
    );
}

#[test]
fn storage_unit_representation_exposes_exact_width_bytes() {
    let class_32 = StorageUnit::Class32(1)
        .representation(Data::LeastSignificantByteFirst)
        .expect("ELF32 storage unit must serialize");
    let class_64 = StorageUnit::Class64(1)
        .representation(Data::LeastSignificantByteFirst)
        .expect("ELF64 storage unit must serialize");

    assert_eq!(class_32.bytes().len(), 4);
    assert_eq!(class_64.bytes().len(), 8);
}

#[test]
fn rejects_storage_unit_representation_without_data_encoding() {
    let storage_unit = StorageUnit::Class64(0x1234);

    assert_eq!(
        storage_unit.representation(Data::None),
        Err(RepresentationError::UnsupportedData(Data::None)),
    );
    assert_eq!(
        storage_unit.representation(Data::Reserved(7)),
        Err(RepresentationError::UnsupportedData(Data::Reserved(7))),
    );
}


#[test]
fn decodes_elf32_storage_unit_representation_by_byte_order() {
    let little = StorageUnitRepresentation::Class32([0x78, 0x56, 0x34, 0x12]);
    let big = StorageUnitRepresentation::Class32([0x12, 0x34, 0x56, 0x78]);

    assert_eq!(
        little.decode(Data::LeastSignificantByteFirst),
        Ok(StorageUnit::Class32(0x1234_5678)),
    );
    assert_eq!(
        big.decode(Data::MostSignificantByteFirst),
        Ok(StorageUnit::Class32(0x1234_5678)),
    );
}

#[test]
fn decodes_elf64_storage_unit_representation_by_byte_order() {
    let little = StorageUnitRepresentation::Class64([
        0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23, 0x01,
    ]);
    let big = StorageUnitRepresentation::Class64([
        0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef,
    ]);

    assert_eq!(
        little.decode(Data::LeastSignificantByteFirst),
        Ok(StorageUnit::Class64(0x0123_4567_89ab_cdef)),
    );
    assert_eq!(
        big.decode(Data::MostSignificantByteFirst),
        Ok(StorageUnit::Class64(0x0123_4567_89ab_cdef)),
    );
}

#[test]
fn storage_unit_representation_round_trips_elf32() {
    let storage_unit = StorageUnit::Class32(0x89ab_cdef);

    for data in [
        Data::LeastSignificantByteFirst,
        Data::MostSignificantByteFirst,
    ] {
        let representation = storage_unit
            .representation(data)
            .expect("ELF32 storage unit must serialize");
        assert_eq!(representation.decode(data), Ok(storage_unit));
    }
}

#[test]
fn storage_unit_representation_round_trips_elf64() {
    let storage_unit = StorageUnit::Class64(0x0123_4567_89ab_cdef);

    for data in [
        Data::LeastSignificantByteFirst,
        Data::MostSignificantByteFirst,
    ] {
        let representation = storage_unit
            .representation(data)
            .expect("ELF64 storage unit must serialize");
        assert_eq!(representation.decode(data), Ok(storage_unit));
    }
}

#[test]
fn rejects_storage_unit_decoding_without_data_encoding() {
    let representation = StorageUnitRepresentation::Class32([0; 4]);

    assert_eq!(
        representation.decode(Data::None),
        Err(RepresentationError::UnsupportedData(Data::None)),
    );
    assert_eq!(
        representation.decode(Data::Reserved(7)),
        Err(RepresentationError::UnsupportedData(Data::Reserved(7))),
    );
}


#[test]
fn plans_elf64_relative_relocation_storage_unit_writes() {
    let table = Table::new(
        vec![Entry::Address(0x400000), Entry::Bitmap(0b11)],
        Class::Class64,
    );
    let storage_units = [
        StorageUnitRepresentation::Class64(0x1000u64.to_le_bytes()),
        StorageUnitRepresentation::Class64(0x2000u64.to_le_bytes()),
    ];
    let factor = RelocationFactor::from_virtual_addresses(0x500000, 0x400000);

    assert_eq!(
        table.storage_unit_writes(
            &storage_units,
            Data::LeastSignificantByteFirst,
            factor,
        ),
        Ok(vec![
            StorageUnitWrite {
                link_time_virtual_address: 0x400000,
                load_time_virtual_address: 0x500000,
                representation: StorageUnitRepresentation::Class64(
                    0x101000u64.to_le_bytes(),
                ),
            },
            StorageUnitWrite {
                link_time_virtual_address: 0x400008,
                load_time_virtual_address: 0x500008,
                representation: StorageUnitRepresentation::Class64(
                    0x102000u64.to_le_bytes(),
                ),
            },
        ]),
    );
}

#[test]
fn plans_big_endian_relative_relocation_storage_unit_write() {
    let table = Table::new(vec![Entry::Address(0x1000)], Class::Class32);
    let storage_units = [
        StorageUnitRepresentation::Class32(0x2000u32.to_be_bytes()),
    ];
    let factor = RelocationFactor::from_virtual_addresses(0x2000, 0x1000);

    assert_eq!(
        table.storage_unit_writes(
            &storage_units,
            Data::MostSignificantByteFirst,
            factor,
        ),
        Ok(vec![StorageUnitWrite {
            link_time_virtual_address: 0x1000,
            load_time_virtual_address: 0x2000,
            representation: StorageUnitRepresentation::Class32(
                0x3000u32.to_be_bytes(),
            ),
        }]),
    );
}

#[test]
fn rejects_relative_relocation_storage_unit_count_mismatch() {
    let table = Table::new(
        vec![Entry::Address(0x400000), Entry::Bitmap(0b11)],
        Class::Class64,
    );
    let storage_units = [
        StorageUnitRepresentation::Class64(0x1000u64.to_le_bytes()),
    ];
    let factor = RelocationFactor::from_virtual_addresses(0x500000, 0x400000);

    assert_eq!(
        table.storage_unit_writes(
            &storage_units,
            Data::LeastSignificantByteFirst,
            factor,
        ),
        Err(WriteError::StorageUnitCountMismatch {
            addresses: 2,
            storage_units: 1,
        }),
    );
}

#[test]
fn rejects_relative_relocation_storage_unit_class_mismatch() {
    let table = Table::new(vec![Entry::Address(0x400000)], Class::Class64);
    let storage_units = [
        StorageUnitRepresentation::Class32(0x1000u32.to_le_bytes()),
    ];
    let factor = RelocationFactor::from_virtual_addresses(0x500000, 0x400000);

    assert_eq!(
        table.storage_unit_writes(
            &storage_units,
            Data::LeastSignificantByteFirst,
            factor,
        ),
        Err(WriteError::StorageUnitClassMismatch {
            index: 0,
            class: Class::Class64,
        }),
    );
}

#[test]
fn propagates_relative_relocation_storage_unit_representation_error() {
    let table = Table::new(vec![Entry::Address(0x400000)], Class::Class64);
    let storage_units = [
        StorageUnitRepresentation::Class64(0u64.to_le_bytes()),
    ];
    let factor = RelocationFactor::from_virtual_addresses(0, 1);

    assert_eq!(
        table.storage_unit_writes(
            &storage_units,
            Data::LeastSignificantByteFirst,
            factor,
        ),
        Err(WriteError::Representation(
            RepresentationError::ValueOutOfRange { value: -1 },
        )),
    );
}


#[test]
fn relocates_relative_relocation_virtual_address_to_load_time() {
    let factor = RelocationFactor::from_virtual_addresses(0x500000, 0x400000);

    assert_eq!(
        factor.relocate_virtual_address(0x401000),
        Ok(0x501000),
    );
}

#[test]
fn relocates_relative_relocation_virtual_address_with_negative_factor() {
    let factor = RelocationFactor::from_virtual_addresses(0x300000, 0x400000);

    assert_eq!(
        factor.relocate_virtual_address(0x401000),
        Ok(0x301000),
    );
}

#[test]
fn rejects_relative_relocation_load_time_virtual_address_out_of_range() {
    let factor = RelocationFactor::from_virtual_addresses(u64::MAX, 0);

    assert_eq!(
        factor.relocate_virtual_address(1),
        Err(VirtualAddressError::OutOfRange {
            value: u64::MAX as i128 + 1,
        }),
    );
}


#[test]
fn plans_relative_relocation_writes_from_loaded_memory_image() {
    let table = Table::new(
        vec![Entry::Address(0x400000), Entry::Bitmap(0b11)],
        Class::Class64,
    );
    let mut bytes = [0u8; 16];
    bytes[0..8].copy_from_slice(&0x1000u64.to_le_bytes());
    bytes[8..16].copy_from_slice(&0x2000u64.to_le_bytes());

    let mut regions = ample::r#type::Vec::new();
    regions.push(Region::new(0x500000, &bytes));
    let image = MemoryImage::new(regions);
    let factor = RelocationFactor::from_virtual_addresses(0x500000, 0x400000);

    assert_eq!(
        table.storage_unit_writes_from_memory_image(
            &image,
            Data::LeastSignificantByteFirst,
            factor,
        ),
        Ok(vec![
            StorageUnitWrite {
                link_time_virtual_address: 0x400000,
                load_time_virtual_address: 0x500000,
                representation: StorageUnitRepresentation::Class64(
                    0x101000u64.to_le_bytes(),
                ),
            },
            StorageUnitWrite {
                link_time_virtual_address: 0x400008,
                load_time_virtual_address: 0x500008,
                representation: StorageUnitRepresentation::Class64(
                    0x102000u64.to_le_bytes(),
                ),
            },
        ]),
    );
}

#[test]
fn plans_big_endian_relative_relocation_write_from_loaded_memory_image() {
    let table = Table::new(vec![Entry::Address(0x1000)], Class::Class32);
    let bytes = 0x2000u32.to_be_bytes();

    let mut regions = ample::r#type::Vec::new();
    regions.push(Region::new(0x2000, &bytes));
    let image = MemoryImage::new(regions);
    let factor = RelocationFactor::from_virtual_addresses(0x2000, 0x1000);

    assert_eq!(
        table.storage_unit_writes_from_memory_image(
            &image,
            Data::MostSignificantByteFirst,
            factor,
        ),
        Ok(vec![StorageUnitWrite {
            link_time_virtual_address: 0x1000,
            load_time_virtual_address: 0x2000,
            representation: StorageUnitRepresentation::Class32(
                0x3000u32.to_be_bytes(),
            ),
        }]),
    );
}

#[test]
fn rejects_relative_relocation_storage_unit_missing_from_memory_image() {
    let table = Table::new(vec![Entry::Address(0x400000)], Class::Class64);
    let mut regions = ample::r#type::Vec::new();
    regions.push(Region::new(0x600000, &[0u8; 8]));
    let image = MemoryImage::new(regions);
    let factor = RelocationFactor::from_virtual_addresses(0x500000, 0x400000);

    assert_eq!(
        table.storage_unit_writes_from_memory_image(
            &image,
            Data::LeastSignificantByteFirst,
            factor,
        ),
        Err(WriteError::StorageUnitUnavailable {
            index: 0,
            load_time_virtual_address: 0x500000,
        }),
    );
}

#[test]
fn rejects_relative_relocation_load_address_overflow_before_memory_read() {
    let table = Table::new(vec![Entry::Address(1)], Class::Class64);
    let image = MemoryImage::new(ample::r#type::Vec::new());
    let factor = RelocationFactor::from_virtual_addresses(u64::MAX, 0);

    assert_eq!(
        table.storage_unit_writes_from_memory_image(
            &image,
            Data::LeastSignificantByteFirst,
            factor,
        ),
        Err(WriteError::VirtualAddress(
            VirtualAddressError::OutOfRange {
                value: u64::MAX as i128 + 1,
            },
        )),
    );
}


#[test]
fn applies_relative_relocation_storage_unit_write() {
    let mut bytes = [0u8; 16];
    let mut regions = ample::r#type::Vec::new();
    regions.push(RegionWriter::new(0x500000, &mut bytes));
    let mut image = MemoryImageWriter::new(regions);

    let write = StorageUnitWrite {
        link_time_virtual_address: 0x400008,
        load_time_virtual_address: 0x500008,
        representation: StorageUnitRepresentation::Class64(
            0x1234_5678_9abc_def0u64.to_le_bytes(),
        ),
    };

    assert_eq!(write.apply(&mut image), Ok(()));
    drop(image);

    assert_eq!(
        &bytes[8..16],
        &0x1234_5678_9abc_def0u64.to_le_bytes(),
    );
}

#[test]
fn applies_relative_relocation_write_at_load_time_address_only() {
    let mut link_time_bytes = [0u8; 8];
    let mut load_time_bytes = [0u8; 8];

    let mut regions = ample::r#type::Vec::new();
    regions.push(RegionWriter::new(0x400000, &mut link_time_bytes));
    regions.push(RegionWriter::new(0x500000, &mut load_time_bytes));
    let mut image = MemoryImageWriter::new(regions);

    let write = StorageUnitWrite {
        link_time_virtual_address: 0x400000,
        load_time_virtual_address: 0x500000,
        representation: StorageUnitRepresentation::Class64(
            0x55aa_55aa_55aa_55aau64.to_le_bytes(),
        ),
    };

    assert_eq!(write.apply(&mut image), Ok(()));
    drop(image);

    assert_eq!(link_time_bytes, [0; 8]);
    assert_eq!(
        load_time_bytes,
        0x55aa_55aa_55aa_55aau64.to_le_bytes(),
    );
}

#[test]
fn rejects_relative_relocation_write_outside_memory_image() {
    let mut bytes = [0u8; 8];
    let mut regions = ample::r#type::Vec::new();
    regions.push(RegionWriter::new(0x600000, &mut bytes));
    let mut image = MemoryImageWriter::new(regions);

    let write = StorageUnitWrite {
        link_time_virtual_address: 0x400000,
        load_time_virtual_address: 0x500000,
        representation: StorageUnitRepresentation::Class64(
            0x1000u64.to_le_bytes(),
        ),
    };

    assert_eq!(
        write.apply(&mut image),
        Err(ApplicationError::MemoryImage(
            MemoryImageWriteError::Unavailable {
                virtual_address: 0x500000,
                size: 8,
            },
        )),
    );
    drop(image);
    assert_eq!(bytes, [0; 8]);
}

#[test]
fn applied_relative_relocation_write_matches_planned_representation() {
    let table = Table::new(vec![Entry::Address(0x400000)], Class::Class64);
    let source = 0x1000u64.to_le_bytes();

    let mut read_regions = ample::r#type::Vec::new();
    read_regions.push(Region::new(0x500000, &source));
    let read_image = MemoryImage::new(read_regions);
    let factor = RelocationFactor::from_virtual_addresses(0x500000, 0x400000);

    let writes = table
        .storage_unit_writes_from_memory_image(
            &read_image,
            Data::LeastSignificantByteFirst,
            factor,
        )
        .expect("RELR write must plan");

    let mut loaded = source;
    let mut write_regions = ample::r#type::Vec::new();
    write_regions.push(RegionWriter::new(0x500000, &mut loaded));
    let mut write_image = MemoryImageWriter::new(write_regions);

    assert_eq!(writes[0].apply(&mut write_image), Ok(()));
    drop(write_image);

    assert_eq!(loaded, 0x101000u64.to_le_bytes());
}


#[test]
fn applies_relative_relocation_storage_unit_write_batch() {
    let mut first = [0u8; 8];
    let mut second = [0u8; 8];

    let mut regions = ample::r#type::Vec::new();
    regions.push(RegionWriter::new(0x500000, &mut first));
    regions.push(RegionWriter::new(0x500008, &mut second));
    let mut image = MemoryImageWriter::new(regions);

    let writes = [
        StorageUnitWrite {
            link_time_virtual_address: 0x400000,
            load_time_virtual_address: 0x500000,
            representation: StorageUnitRepresentation::Class64(
                0x1111u64.to_le_bytes(),
            ),
        },
        StorageUnitWrite {
            link_time_virtual_address: 0x400008,
            load_time_virtual_address: 0x500008,
            representation: StorageUnitRepresentation::Class64(
                0x2222u64.to_le_bytes(),
            ),
        },
    ];

    assert_eq!(apply_storage_unit_writes(&writes, &mut image), Ok(()));
    drop(image);

    assert_eq!(first, 0x1111u64.to_le_bytes());
    assert_eq!(second, 0x2222u64.to_le_bytes());
}

#[test]
fn relative_relocation_storage_unit_write_batch_is_atomic_on_late_failure() {
    let mut first = [0u8; 8];

    let mut regions = ample::r#type::Vec::new();
    regions.push(RegionWriter::new(0x500000, &mut first));
    let mut image = MemoryImageWriter::new(regions);

    let writes = [
        StorageUnitWrite {
            link_time_virtual_address: 0x400000,
            load_time_virtual_address: 0x500000,
            representation: StorageUnitRepresentation::Class64(
                0x1111u64.to_le_bytes(),
            ),
        },
        StorageUnitWrite {
            link_time_virtual_address: 0x400008,
            load_time_virtual_address: 0x600000,
            representation: StorageUnitRepresentation::Class64(
                0x2222u64.to_le_bytes(),
            ),
        },
    ];

    assert_eq!(
        apply_storage_unit_writes(&writes, &mut image),
        Err(BatchApplicationError {
            index: 1,
            error: ApplicationError::MemoryImage(
                MemoryImageWriteError::Unavailable {
                    virtual_address: 0x600000,
                    size: 8,
                },
            ),
        }),
    );
    drop(image);

    assert_eq!(first, [0; 8]);
}

#[test]
fn relative_relocation_storage_unit_write_batch_is_atomic_on_early_failure() {
    let mut second = [0u8; 8];

    let mut regions = ample::r#type::Vec::new();
    regions.push(RegionWriter::new(0x500008, &mut second));
    let mut image = MemoryImageWriter::new(regions);

    let writes = [
        StorageUnitWrite {
            link_time_virtual_address: 0x400000,
            load_time_virtual_address: 0x600000,
            representation: StorageUnitRepresentation::Class64(
                0x1111u64.to_le_bytes(),
            ),
        },
        StorageUnitWrite {
            link_time_virtual_address: 0x400008,
            load_time_virtual_address: 0x500008,
            representation: StorageUnitRepresentation::Class64(
                0x2222u64.to_le_bytes(),
            ),
        },
    ];

    assert_eq!(
        apply_storage_unit_writes(&writes, &mut image),
        Err(BatchApplicationError {
            index: 0,
            error: ApplicationError::MemoryImage(
                MemoryImageWriteError::Unavailable {
                    virtual_address: 0x600000,
                    size: 8,
                },
            ),
        }),
    );
    drop(image);

    assert_eq!(second, [0; 8]);
}

#[test]
fn empty_relative_relocation_storage_unit_write_batch_is_a_noop() {
    let mut bytes = [0x5au8; 8];

    let mut regions = ample::r#type::Vec::new();
    regions.push(RegionWriter::new(0x500000, &mut bytes));
    let mut image = MemoryImageWriter::new(regions);

    assert_eq!(apply_storage_unit_writes(&[], &mut image), Ok(()));
    drop(image);

    assert_eq!(bytes, [0x5a; 8]);
}
