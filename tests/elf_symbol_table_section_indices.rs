use userspace::file::format::elf::{
    symbol_table::ValidationError,
    ObjectFile,
};

fn half(bytes: &mut Vec<u8>, value: u16) { bytes.extend_from_slice(&value.to_le_bytes()); }
fn word(bytes: &mut Vec<u8>, value: u32) { bytes.extend_from_slice(&value.to_le_bytes()); }
fn xword(bytes: &mut Vec<u8>, value: u64) { bytes.extend_from_slice(&value.to_le_bytes()); }

fn symbol(bytes: &mut Vec<u8>, name: u32, info: u8, shndx: u16) {
    word(bytes, name);
    bytes.push(info);
    bytes.push(0);
    half(bytes, shndx);
    xword(bytes, 0);
    xword(bytes, 0);
}

fn section_header(
    bytes: &mut Vec<u8>,
    ty: u32,
    offset: u64,
    size: u64,
    link: u32,
    info: u32,
    align: u64,
    entsize: u64,
) {
    word(bytes, 0);
    word(bytes, ty);
    xword(bytes, 0);
    xword(bytes, 0);
    xword(bytes, offset);
    xword(bytes, size);
    word(bytes, link);
    word(bytes, info);
    xword(bytes, align);
    xword(bytes, entsize);
}

fn fixture(include_companion: bool, companion_words: &[u32], second_shndx: u16) -> Vec<u8> {
    const HEADER_SIZE: u64 = 64;
    const STRING_OFFSET: u64 = HEADER_SIZE;
    const STRING_SIZE: u64 = 5;
    const SYMBOL_OFFSET: u64 = STRING_OFFSET + STRING_SIZE;
    const SYMBOL_SIZE: u64 = 48;
    const COMPANION_OFFSET: u64 = SYMBOL_OFFSET + SYMBOL_SIZE;
    let companion_size = (companion_words.len() * 4) as u64;
    let section_header_offset = COMPANION_OFFSET + companion_size;
    let section_count = if include_companion { 5 } else { 4 };

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
    xword(&mut bytes, section_header_offset);
    word(&mut bytes, 0);
    half(&mut bytes, HEADER_SIZE as u16);
    half(&mut bytes, 56);
    half(&mut bytes, 0);
    half(&mut bytes, 64);
    half(&mut bytes, section_count);
    half(&mut bytes, 0);

    bytes.extend_from_slice(b"\0foo\0");
    symbol(&mut bytes, 0, 0x00, 0);
    symbol(&mut bytes, 1, 0x10, second_shndx);
    for value in companion_words { word(&mut bytes, *value); }

    bytes.extend_from_slice(&[0u8; 64]);
    section_header(&mut bytes, 3, STRING_OFFSET, STRING_SIZE, 0, 0, 1, 0);
    section_header(&mut bytes, 2, SYMBOL_OFFSET, SYMBOL_SIZE, 1, 1, 8, 24);
    section_header(&mut bytes, 1, 0, 0, 0, 0, 1, 0);
    if include_companion {
        section_header(
            &mut bytes,
            18,
            COMPANION_OFFSET,
            companion_size,
            2,
            0,
            4,
            4,
        );
    }

    bytes
}

#[test]
fn accepts_one_to_one_extended_section_indices() {
    let bytes = fixture(true, &[0, 0xff00], 0xffff);
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");
    assert_eq!(object.validate_symbol_table_section_indices(2), Ok(()));
}

#[test]
fn rejects_missing_extended_section_index_table() {
    let bytes = fixture(false, &[], 0xffff);
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");
    assert_eq!(
        object.validate_symbol_table_section_indices(2),
        Err(ValidationError::MissingSectionIndexTable),
    );
}

#[test]
fn rejects_extended_section_index_table_size_mismatch() {
    let bytes = fixture(true, &[0], 0xffff);
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");
    assert_eq!(
        object.validate_symbol_table_section_indices(2),
        Err(ValidationError::SectionIndexTableSizeMismatch),
    );
}

#[test]
fn rejects_nonzero_extended_index_for_non_extended_symbol() {
    let bytes = fixture(true, &[0, 3], 3);
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");
    assert_eq!(
        object.validate_symbol_table_section_indices(2),
        Err(ValidationError::SectionIndexTableUnexpectedValue {
            index: 1,
            value: 3,
        }),
    );
}


#[test]
fn rejects_extended_section_index_below_reserved_range() {
    let bytes = fixture(true, &[0, 3], 0xffff);
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");
    assert_eq!(
        object.validate_symbol_table_section_indices(2),
        Err(ValidationError::ExtendedSectionIndexBelowReservedRange {
            index: 1,
            value: 3,
        }),
    );
}
