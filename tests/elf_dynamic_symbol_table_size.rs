use userspace::file::format::elf::{
    dynamic::{PayloadKind, Tag},
    ObjectFile,
};

fn half(bytes: &mut Vec<u8>, value: u16) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn word(bytes: &mut Vec<u8>, value: u32) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn sxword(bytes: &mut Vec<u8>, value: i64) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn xword(bytes: &mut Vec<u8>, value: u64) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn dynamic_entry(bytes: &mut Vec<u8>, tag: i64, payload: u64) {
    sxword(bytes, tag);
    xword(bytes, payload);
}

fn fixture_without_hash() -> Vec<u8> {
    const BASE: u64 = 0x400000;
    const HEADER_SIZE: u64 = 64;
    const PROGRAM_HEADER_SIZE: u64 = 56;
    const PROGRAM_HEADER_COUNT: u64 = 2;
    const DYNAMIC_OFFSET: u64 = HEADER_SIZE + PROGRAM_HEADER_SIZE * PROGRAM_HEADER_COUNT;
    const DYNAMIC_SIZE: u64 = 6 * 16;
    const STRING_OFFSET: u64 = DYNAMIC_OFFSET + DYNAMIC_SIZE;
    const STRING_SIZE: u64 = 5;
    const SYMBOL_OFFSET: u64 = STRING_OFFSET + STRING_SIZE;
    const SYMBOL_ENTRY_SIZE: u64 = 24;
    const SYMBOL_COUNT: u64 = 2;
    const SYMBOL_TABLE_SIZE: u64 = SYMBOL_ENTRY_SIZE * SYMBOL_COUNT;
    const TOTAL_SIZE: u64 = SYMBOL_OFFSET + SYMBOL_TABLE_SIZE;

    let mut bytes = Vec::with_capacity(TOTAL_SIZE as usize);

    bytes.extend_from_slice(&[
        0x7f, b'E', b'L', b'F',
        2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]);
    half(&mut bytes, 3);
    half(&mut bytes, 0x3e);
    word(&mut bytes, 1);
    xword(&mut bytes, BASE);
    xword(&mut bytes, HEADER_SIZE);
    xword(&mut bytes, 0);
    word(&mut bytes, 0);
    half(&mut bytes, HEADER_SIZE as u16);
    half(&mut bytes, PROGRAM_HEADER_SIZE as u16);
    half(&mut bytes, PROGRAM_HEADER_COUNT as u16);
    half(&mut bytes, 64);
    half(&mut bytes, 0);
    half(&mut bytes, 0);

    // PT_LOAD
    word(&mut bytes, 1);
    word(&mut bytes, 4);
    xword(&mut bytes, 0);
    xword(&mut bytes, BASE);
    xword(&mut bytes, BASE);
    xword(&mut bytes, TOTAL_SIZE);
    xword(&mut bytes, TOTAL_SIZE);
    xword(&mut bytes, 0x1000);

    // PT_DYNAMIC
    word(&mut bytes, 2);
    word(&mut bytes, 4);
    xword(&mut bytes, DYNAMIC_OFFSET);
    xword(&mut bytes, BASE + DYNAMIC_OFFSET);
    xword(&mut bytes, BASE + DYNAMIC_OFFSET);
    xword(&mut bytes, DYNAMIC_SIZE);
    xword(&mut bytes, DYNAMIC_SIZE);
    xword(&mut bytes, 8);

    dynamic_entry(&mut bytes, 5, BASE + STRING_OFFSET);
    dynamic_entry(&mut bytes, 10, STRING_SIZE);
    dynamic_entry(&mut bytes, 6, BASE + SYMBOL_OFFSET);
    dynamic_entry(&mut bytes, 11, SYMBOL_ENTRY_SIZE);
    dynamic_entry(&mut bytes, 39, SYMBOL_TABLE_SIZE);
    dynamic_entry(&mut bytes, 0, 0);

    bytes.extend_from_slice(b"\0foo\0");

    // Undefined symbol.
    bytes.extend_from_slice(&[0u8; 24]);

    // Global object symbol named "foo".
    word(&mut bytes, 1);
    bytes.push(0x11);
    bytes.push(0);
    half(&mut bytes, 1);
    xword(&mut bytes, 0x401000);
    xword(&mut bytes, 4);

    assert_eq!(bytes.len(), TOTAL_SIZE as usize);
    bytes
}

#[test]
fn recognizes_dynamic_symbol_table_size_tag() {
    assert_eq!(Tag::from_raw(39), Tag::SymbolTableSize);
    assert_eq!(Tag::SymbolTableSize.payload_kind(), PayloadKind::Value);
}

#[test]
fn resolves_dynamic_symbol_table_without_system_v_hash() {
    let bytes = fixture_without_hash();
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");
    let symbols = object
        .dynamic_symbol_table_from_program_header(1)
        .expect("DT_SYMTABSZ must make DT_HASH unnecessary");

    assert_eq!(symbols.len(), 2);
    assert_eq!(symbols.name(0), Some(""));
    assert_eq!(symbols.name(1), Some("foo"));
    assert_eq!(symbols.get(1).expect("second symbol").value, 0x401000);
    assert_eq!(symbols.get(1).expect("second symbol").size, 4);
}
