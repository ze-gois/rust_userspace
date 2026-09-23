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
