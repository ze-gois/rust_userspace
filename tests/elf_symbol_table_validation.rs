use userspace::file::format::elf::{
    symbol_table::ValidationError,
    ObjectFile,
};

fn half(bytes: &mut Vec<u8>, value: u16) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn word(bytes: &mut Vec<u8>, value: u32) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn xword(bytes: &mut Vec<u8>, value: u64) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn symbol(
    bytes: &mut Vec<u8>,
    name_index: u32,
    information: u8,
    section_index: u16,
) {
    word(bytes, name_index);
    bytes.push(information);
    bytes.push(0);
    half(bytes, section_index);
    xword(bytes, 0);
    xword(bytes, 0);
}

fn section_header(
    bytes: &mut Vec<u8>,
    section_type: u32,
    offset: u64,
    size: u64,
    link: u32,
    information: u32,
    alignment: u64,
    entry_size: u64,
) {
    word(bytes, 0);
    word(bytes, section_type);
    xword(bytes, 0);
    xword(bytes, 0);
    xword(bytes, offset);
    xword(bytes, size);
    word(bytes, link);
    word(bytes, information);
    xword(bytes, alignment);
    xword(bytes, entry_size);
}

fn fixture(first_non_local: u32, second_information: u8, third_information: u8) -> Vec<u8> {
    const HEADER_SIZE: u64 = 64;
    const STRING_OFFSET: u64 = HEADER_SIZE;
    const STRING_SIZE: u64 = 14;
    const SYMBOL_OFFSET: u64 = STRING_OFFSET + STRING_SIZE;
    const SYMBOL_SIZE: u64 = 72;
    const SECTION_HEADER_OFFSET: u64 = SYMBOL_OFFSET + SYMBOL_SIZE;

    let mut bytes = Vec::new();

    bytes.extend_from_slice(&[
        0x7f, b'E', b'L', b'F',
        2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]);
    half(&mut bytes, 1);
    half(&mut bytes, 0x3e);
    word(&mut bytes, 1);
    xword(&mut bytes, 0);
    xword(&mut bytes, 0);
    xword(&mut bytes, SECTION_HEADER_OFFSET);
    word(&mut bytes, 0);
    half(&mut bytes, HEADER_SIZE as u16);
    half(&mut bytes, 56);
    half(&mut bytes, 0);
    half(&mut bytes, 64);
    half(&mut bytes, 4);
    half(&mut bytes, 0);

    bytes.extend_from_slice(b"\0local\0global\0");

    // [0] STN_UNDEF.
    symbol(&mut bytes, 0, 0x00, 0);
    // [1] and [2] allow binding mutation through st_info.
    symbol(&mut bytes, 1, second_information, 3);
    symbol(&mut bytes, 7, third_information, 3);

    bytes.extend_from_slice(&[0u8; 64]);
    section_header(&mut bytes, 3, STRING_OFFSET, STRING_SIZE, 0, 0, 1, 0);
    section_header(
        &mut bytes,
        2,
        SYMBOL_OFFSET,
        SYMBOL_SIZE,
        1,
        first_non_local,
        8,
        24,
    );
    section_header(&mut bytes, 1, 0, 0, 0, 0, 1, 0);

    bytes
}

#[test]
fn accepts_undefined_symbol_and_local_prefix() {
    let bytes = fixture(2, 0x00, 0x10);
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");
    let symbols = object.symbol_table(2).expect("symbol table must resolve");

    assert_eq!(symbols.validate(), Ok(()));
}

#[test]
fn rejects_invalid_undefined_symbol() {
    let mut bytes = fixture(2, 0x00, 0x10);
    let undefined_name_offset = 64 + 14;
    bytes[undefined_name_offset..undefined_name_offset + 4]
        .copy_from_slice(&1u32.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");
    let symbols = object.symbol_table(2).expect("symbol table must resolve");

    assert_eq!(symbols.validate(), Err(ValidationError::InvalidUndefinedSymbol));
}

#[test]
fn rejects_non_local_symbol_before_sh_info_boundary() {
    let bytes = fixture(2, 0x10, 0x10);
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");
    let symbols = object.symbol_table(2).expect("symbol table must resolve");

    assert_eq!(
        symbols.validate(),
        Err(ValidationError::NonLocalSymbolBeforeFirstNonLocal { index: 1 }),
    );
}

#[test]
fn rejects_local_symbol_at_or_after_sh_info_boundary() {
    let bytes = fixture(2, 0x00, 0x00);
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");
    let symbols = object.symbol_table(2).expect("symbol table must resolve");

    assert_eq!(
        symbols.validate(),
        Err(ValidationError::LocalSymbolAtOrAfterFirstNonLocal { index: 2 }),
    );
}

#[test]
fn rejects_sh_info_boundary_beyond_symbol_table() {
    let bytes = fixture(4, 0x00, 0x10);
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");
    let symbols = object.symbol_table(2).expect("symbol table must resolve");

    assert_eq!(
        symbols.validate(),
        Err(ValidationError::FirstNonLocalIndexOutOfBounds { index: 4 }),
    );
}


fn symbol_offset(index: usize) -> usize {
    64 + 14 + index * 24
}

#[test]
fn rejects_undefined_upper_st_other_bits() {
    let mut bytes = fixture(2, 0x00, 0x10);
    bytes[symbol_offset(1) + 5] = 0x80;

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");
    let symbols = object.symbol_table(2).expect("symbol table must resolve");

    assert_eq!(
        symbols.validate(),
        Err(ValidationError::UndefinedOtherBits {
            index: 1,
            bits: 0x80,
        }),
    );
}

#[test]
fn rejects_reserved_symbol_binding() {
    let bytes = fixture(2, 0x00, 0x30);
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");
    let symbols = object.symbol_table(2).expect("symbol table must resolve");

    assert_eq!(
        symbols.validate(),
        Err(ValidationError::ReservedBinding { index: 2, raw: 3 }),
    );
}

#[test]
fn rejects_reserved_symbol_type() {
    let bytes = fixture(2, 0x00, 0x17);
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");
    let symbols = object.symbol_table(2).expect("symbol table must resolve");

    assert_eq!(
        symbols.validate(),
        Err(ValidationError::ReservedType { index: 2, raw: 7 }),
    );
}

#[test]
fn rejects_reserved_symbol_visibility() {
    let mut bytes = fixture(2, 0x00, 0x10);
    bytes[symbol_offset(2) + 5] = 7;

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");
    let symbols = object.symbol_table(2).expect("symbol table must resolve");

    assert_eq!(
        symbols.validate(),
        Err(ValidationError::ReservedVisibility { index: 2, raw: 7 }),
    );
}

#[test]
fn rejects_invalid_symbol_name_index() {
    let mut bytes = fixture(2, 0x00, 0x10);
    let offset = symbol_offset(2);
    bytes[offset..offset + 4].copy_from_slice(&99u32.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");
    let symbols = object.symbol_table(2).expect("symbol table must resolve");

    assert_eq!(
        symbols.validate(),
        Err(ValidationError::InvalidNameIndex {
            index: 2,
            name_index: 99,
        }),
    );
}

#[test]
fn rejects_protected_local_symbol() {
    let mut bytes = fixture(2, 0x00, 0x10);
    bytes[symbol_offset(1) + 5] = 3;

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");
    let symbols = object.symbol_table(2).expect("symbol table must resolve");

    assert_eq!(
        symbols.validate(),
        Err(ValidationError::LocalProtectedVisibility { index: 1 }),
    );
}

#[test]
fn accepts_local_absolute_file_symbol() {
    let mut bytes = fixture(2, 0x04, 0x10);
    let offset = symbol_offset(1);
    bytes[offset + 6..offset + 8].copy_from_slice(&0xfff1u16.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");
    let symbols = object.symbol_table(2).expect("symbol table must resolve");

    assert_eq!(symbols.validate(), Ok(()));
}

#[test]
fn rejects_non_local_file_symbol() {
    let mut bytes = fixture(2, 0x00, 0x14);
    let offset = symbol_offset(2);
    bytes[offset + 6..offset + 8].copy_from_slice(&0xfff1u16.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");
    let symbols = object.symbol_table(2).expect("symbol table must resolve");

    assert_eq!(
        symbols.validate(),
        Err(ValidationError::FileSymbolNotLocal { index: 2 }),
    );
}

#[test]
fn rejects_non_absolute_file_symbol() {
    let bytes = fixture(2, 0x04, 0x10);
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");
    let symbols = object.symbol_table(2).expect("symbol table must resolve");

    assert_eq!(
        symbols.validate(),
        Err(ValidationError::FileSymbolNotAbsolute { index: 1 }),
    );
}


#[test]
fn rejects_symbol_section_index_outside_section_table() {
    let mut bytes = fixture(2, 0x00, 0x10);
    let offset = symbol_offset(2);
    bytes[offset + 6..offset + 8].copy_from_slice(&99u16.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_symbol_table(2),
        Err(ValidationError::SectionIndexOutOfBounds {
            index: 2,
            section_index: 99,
        }),
    );
}

#[test]
fn rejects_reserved_symbol_section_index() {
    let mut bytes = fixture(2, 0x00, 0x10);
    let offset = symbol_offset(2);
    bytes[offset + 6..offset + 8].copy_from_slice(&0xff40u16.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_symbol_table(2),
        Err(ValidationError::ReservedSectionIndex {
            index: 2,
            raw: 0xff40,
        }),
    );
}

#[test]
fn accepts_common_symbol_in_relocatable_object() {
    let mut bytes = fixture(2, 0x00, 0x15);
    let offset = symbol_offset(2);
    bytes[offset + 6..offset + 8].copy_from_slice(&0xfff2u16.to_le_bytes());
    bytes[offset + 8..offset + 16].copy_from_slice(&8u64.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(object.validate_symbol_table(2), Ok(()));
}

#[test]
fn rejects_common_section_index_outside_relocatable_object() {
    let mut bytes = fixture(2, 0x00, 0x10);
    bytes[16..18].copy_from_slice(&2u16.to_le_bytes());

    let offset = symbol_offset(2);
    bytes[offset + 6..offset + 8].copy_from_slice(&0xfff2u16.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_symbol_table(2),
        Err(ValidationError::CommonSectionIndexOutsideRelocatableObject {
            index: 2,
        }),
    );
}

#[test]
fn rejects_common_symbol_without_common_section_in_relocatable_object() {
    let bytes = fixture(2, 0x00, 0x15);
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_symbol_table(2),
        Err(ValidationError::CommonSymbolWithoutCommonSection { index: 2 }),
    );
}

#[test]
fn rejects_unallocated_common_symbol_in_executable() {
    let mut bytes = fixture(2, 0x00, 0x15);
    bytes[16..18].copy_from_slice(&2u16.to_le_bytes());

    let offset = symbol_offset(2);
    bytes[offset + 6..offset + 8].copy_from_slice(&0xfff1u16.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_symbol_table(2),
        Err(ValidationError::CommonSymbolWithoutAllocatedSection { index: 2 }),
    );
}

#[test]
fn rejects_non_power_of_two_common_alignment() {
    let mut bytes = fixture(2, 0x00, 0x15);
    let offset = symbol_offset(2);
    bytes[offset + 6..offset + 8].copy_from_slice(&0xfff2u16.to_le_bytes());
    bytes[offset + 8..offset + 16].copy_from_slice(&3u64.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_symbol_table(2),
        Err(ValidationError::CommonAlignmentNotPowerOfTwo {
            index: 2,
            alignment: 3,
        }),
    );
}
