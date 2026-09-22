use userspace::file::format::elf::ObjectFile;

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

fn fixture() -> Vec<u8> {
    const HEADER_SIZE: u64 = 64;
    const STRING_OFFSET: u64 = HEADER_SIZE;
    const STRING_SIZE: u64 = 7;
    const SYMBOL_OFFSET: u64 = STRING_OFFSET + STRING_SIZE;
    const SYMBOL_SIZE: u64 = 48;
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

    bytes.extend_from_slice(b"\0thing\0");

    // Symbol [0]: STN_UNDEF.
    symbol(&mut bytes, 0, 0, 0);
    // Symbol [1]: global object defined in section [3].
    symbol(&mut bytes, 1, 0x11, 3);

    // [0] SHT_NULL.
    bytes.extend_from_slice(&[0u8; 64]);
    // [1] SHT_STRTAB.
    section_header(&mut bytes, 3, STRING_OFFSET, STRING_SIZE, 0, 0, 1, 0);
    // [2] SHT_SYMTAB -> string table [1], first non-local symbol [1].
    section_header(&mut bytes, 2, SYMBOL_OFFSET, SYMBOL_SIZE, 1, 1, 8, 24);
    // [3] SHT_PROGBITS.
    section_header(&mut bytes, 1, 0, 0, 0, 0, 1, 0);

    bytes
}

#[test]
fn resolves_symbol_defined_in_section() {
    let bytes = fixture();
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    let section = object
        .symbol_section(2, 1)
        .expect("defined symbol section must resolve");

    assert_eq!(section.header.r#type.raw(), 1);
}

#[test]
fn leaves_undefined_symbol_without_section_relation() {
    let bytes = fixture();
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert!(object.symbol_section(2, 0).is_none());
}
