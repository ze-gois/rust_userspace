use userspace::file::format::elf::{
    section_link::Meaning,
    ObjectFile,
};

fn half(bytes: &mut Vec<u8>, value: u16) { bytes.extend_from_slice(&value.to_le_bytes()); }
fn word(bytes: &mut Vec<u8>, value: u32) { bytes.extend_from_slice(&value.to_le_bytes()); }
fn xword(bytes: &mut Vec<u8>, value: u64) { bytes.extend_from_slice(&value.to_le_bytes()); }

fn section_header(
    bytes: &mut Vec<u8>,
    section_type: u32,
    offset: u64,
    size: u64,
    link: u32,
    entry_size: u64,
) {
    word(bytes, 0);
    word(bytes, section_type);
    xword(bytes, 0);
    xword(bytes, 0);
    xword(bytes, offset);
    xword(bytes, size);
    word(bytes, link);
    word(bytes, 0);
    xword(bytes, 1);
    xword(bytes, entry_size);
}

fn fixture() -> Vec<u8> {
    const HEADER_SIZE: u64 = 64;
    const STRING_OFFSET: u64 = HEADER_SIZE;
    const STRING_SIZE: u64 = 1;
    const SECTION_HEADER_OFFSET: u64 = STRING_OFFSET + STRING_SIZE;

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

    bytes.push(0);

    // [0] SHT_NULL
    bytes.extend_from_slice(&[0u8; 64]);
    // [1] SHT_STRTAB
    section_header(&mut bytes, 3, STRING_OFFSET, STRING_SIZE, 0, 0);
    // [2] SHT_SYMTAB -> string table [1]
    section_header(&mut bytes, 2, 0, 0, 1, 24);
    // [3] SHT_RELA -> symbol table [2]
    section_header(&mut bytes, 4, 0, 0, 2, 24);

    bytes
}

#[test]
fn resolves_symbol_table_string_table_link() {
    let bytes = fixture();
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    let link = object.section_link(2).expect("symbol table link must resolve");
    assert_eq!(link.section_index, 2);
    assert_eq!(link.linked_section_index, 1);
    assert_eq!(link.meaning, Meaning::StringTable);
}

#[test]
fn resolves_relocation_symbol_table_link() {
    let bytes = fixture();
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    let link = object.section_link(3).expect("relocation link must resolve");
    assert_eq!(link.section_index, 3);
    assert_eq!(link.linked_section_index, 2);
    assert_eq!(link.meaning, Meaning::SymbolTable);
}

#[test]
fn leaves_unassigned_section_link_uninterpreted() {
    let bytes = fixture();
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert!(object.section_link(1).is_none());
}
