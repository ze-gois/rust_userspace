use userspace::file::format::elf::{
    relocation::ValidationError,
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

fn sxword(bytes: &mut Vec<u8>, value: i64) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn symbol(bytes: &mut Vec<u8>, name_index: u32, information: u8, section_index: u16) {
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

fn fixture(
    object_type: u16,
    with_addend: bool,
    entry_size: u64,
    symbol_index: u32,
    relocation_offset: u64,
    target_section_index: u32,
) -> Vec<u8> {
    const HEADER_SIZE: u64 = 64;
    const STRING_OFFSET: u64 = HEADER_SIZE;
    const STRING_SIZE: u64 = 8;
    const SYMBOL_OFFSET: u64 = STRING_OFFSET + STRING_SIZE;
    const SYMBOL_SIZE: u64 = 48;
    const TARGET_OFFSET: u64 = SYMBOL_OFFSET + SYMBOL_SIZE;
    const TARGET_SIZE: u64 = 8;

    let relocation_size = if with_addend { 24 } else { 16 };
    let relocation_offset_in_file = TARGET_OFFSET + TARGET_SIZE;
    let section_header_offset = relocation_offset_in_file + relocation_size;

    let mut bytes = Vec::new();

    bytes.extend_from_slice(&[
        0x7f, b'E', b'L', b'F',
        2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]);
    half(&mut bytes, object_type);
    half(&mut bytes, 0x3e);
    word(&mut bytes, 1);
    xword(&mut bytes, 0);
    xword(&mut bytes, 0);
    xword(&mut bytes, section_header_offset);
    word(&mut bytes, 0);
    half(&mut bytes, HEADER_SIZE as u16);
    half(&mut bytes, 56);
    half(&mut bytes, 0);
    half(&mut bytes, 64);
    half(&mut bytes, 5);
    half(&mut bytes, 0);

    bytes.extend_from_slice(b"\0symbol\0");

    symbol(&mut bytes, 0, 0, 0);
    symbol(&mut bytes, 1, 0x11, 3);

    bytes.extend_from_slice(&[0u8; TARGET_SIZE as usize]);

    xword(&mut bytes, relocation_offset);
    xword(&mut bytes, (u64::from(symbol_index) << 32) | 1);
    if with_addend {
        sxword(&mut bytes, 0);
    }

    bytes.extend_from_slice(&[0u8; 64]);
    section_header(&mut bytes, 3, STRING_OFFSET, STRING_SIZE, 0, 0, 1, 0);
    section_header(&mut bytes, 2, SYMBOL_OFFSET, SYMBOL_SIZE, 1, 1, 8, 24);
    section_header(&mut bytes, 1, TARGET_OFFSET, TARGET_SIZE, 0, 0, 1, 0);
    section_header(
        &mut bytes,
        if with_addend { 4 } else { 9 },
        relocation_offset_in_file,
        relocation_size,
        2,
        target_section_index,
        8,
        entry_size,
    );

    bytes
}

#[test]
fn accepts_rel_and_rela_sections_in_relocatable_object() {
    for (with_addend, entry_size) in [(false, 16), (true, 24)] {
        let bytes = fixture(1, with_addend, entry_size, 1, 4, 3);
        let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

        assert_eq!(object.validate_relocation_section(4), Ok(()));
    }
}

#[test]
fn accepts_dynamic_style_relocation_without_section_target() {
    let bytes = fixture(3, true, 24, 1, 0x401000, 0);
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(object.validate_relocation_section(4), Ok(()));
}

#[test]
fn rejects_wrong_relocation_entry_size() {
    let bytes = fixture(1, true, 16, 1, 4, 3);
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_relocation_section(4),
        Err(ValidationError::EntrySizeMismatch {
            expected: 24,
            actual: 16,
        }),
    );
}

#[test]
fn rejects_relocation_section_size_not_entry_multiple() {
    let mut bytes = fixture(1, true, 24, 1, 4, 3);
    let section_header_offset = 64 + 8 + 48 + 8 + 24;
    let relocation_header = section_header_offset + 4 * 64;
    bytes[relocation_header + 32..relocation_header + 40]
        .copy_from_slice(&23u64.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_relocation_section(4),
        Err(ValidationError::SizeNotEntryMultiple),
    );
}

#[test]
fn rejects_relocation_symbol_index_outside_symbol_table() {
    let bytes = fixture(1, true, 24, 2, 4, 3);
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_relocation_section(4),
        Err(ValidationError::SymbolIndexOutOfBounds {
            entry_index: 0,
            symbol_index: 2,
        }),
    );
}

#[test]
fn rejects_missing_target_section_in_relocatable_object() {
    let bytes = fixture(1, true, 24, 1, 4, 0);
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_relocation_section(4),
        Err(ValidationError::MissingTargetSectionInRelocatableObject),
    );
}

#[test]
fn rejects_relocation_offset_outside_target_section() {
    let bytes = fixture(1, true, 24, 1, 8, 3);
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_relocation_section(4),
        Err(ValidationError::OffsetOutsideTargetSection {
            entry_index: 0,
            offset: 8,
            target_section_index: 3,
            target_size: 8,
        }),
    );
}
