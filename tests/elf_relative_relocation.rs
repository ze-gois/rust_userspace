use userspace::file::format::elf::{
    dynamic::{PayloadKind, Tag},
    identification::{Class, Data},
    relocation::relative::{
        class_32, class_64, Entry, ExpansionError, RelocationFactor, StorageUnit, Table,
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
fn relocates_elf32_storage_unit_value() {
    let factor = RelocationFactor::from_virtual_addresses(0x5000, 0x4000);
    let storage_unit = StorageUnit::Class32(0x2000);

    assert_eq!(
        storage_unit.relocate(factor),
        StorageUnit::Class32(0x3000),
    );
}

#[test]
fn relocates_elf64_storage_unit_value_with_negative_factor() {
    let factor = RelocationFactor::from_virtual_addresses(0x3000, 0x5000);
    let storage_unit = StorageUnit::Class64(0x9000);

    assert_eq!(
        storage_unit.relocate(factor),
        StorageUnit::Class64(0x7000),
    );
}

#[test]
fn relative_relocation_storage_unit_uses_elf_class_width() {
    let factor = RelocationFactor::from_virtual_addresses(8, 0);

    assert_eq!(
        StorageUnit::Class32(u32::MAX - 3).relocate(factor),
        StorageUnit::Class32(4),
    );
    assert_eq!(
        StorageUnit::Class64(u64::MAX - 3).relocate(factor),
        StorageUnit::Class64(4),
    );
}

#[test]
fn zero_relative_relocation_factor_preserves_storage_unit() {
    let factor = RelocationFactor::from_virtual_addresses(0x400000, 0x400000);

    assert_eq!(
        StorageUnit::Class32(0x1234_5678).relocate(factor),
        StorageUnit::Class32(0x1234_5678),
    );
    assert_eq!(
        StorageUnit::Class64(0x1234_5678_9abc_def0).relocate(factor),
        StorageUnit::Class64(0x1234_5678_9abc_def0),
    );
}
