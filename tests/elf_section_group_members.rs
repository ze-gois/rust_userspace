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
    const STRING_SIZE: u64 = 1;
    const GROUP_OFFSET: u64 = STRING_OFFSET + STRING_SIZE;
    const GROUP_SIZE: u64 = 12;
    const SECTION_HEADER_OFFSET: u64 = GROUP_OFFSET + GROUP_SIZE;

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
    half(&mut bytes, 6);
    half(&mut bytes, 0);

    bytes.push(0);

    // SHT_GROUP contents: GRP_COMDAT, section [4], section [5].
    word(&mut bytes, 1);
    word(&mut bytes, 4);
    word(&mut bytes, 5);

    // [0] SHT_NULL
    bytes.extend_from_slice(&[0u8; 64]);
    // [1] SHT_STRTAB
    section_header(&mut bytes, 3, STRING_OFFSET, STRING_SIZE, 0, 0, 1, 0);
    // [2] SHT_SYMTAB -> string table [1]
    section_header(&mut bytes, 2, 0, 0, 1, 0, 8, 24);
    // [3] SHT_GROUP -> symbol table [2], signature symbol [0]
    section_header(&mut bytes, 17, GROUP_OFFSET, GROUP_SIZE, 2, 0, 4, 4);
    // [4], [5] member sections.
    section_header(&mut bytes, 1, 0, 0, 0, 0, 1, 0);
    section_header(&mut bytes, 1, 0, 0, 0, 0, 1, 0);

    bytes
}

#[test]
fn resolves_section_group_members() {
    let bytes = fixture();
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    let members = object
        .section_group_members(3)
        .expect("section-group members must resolve");

    assert_eq!(members.len(), 2);
    assert_eq!(members[0].section_index, 4);
    assert_eq!(members[1].section_index, 5);
}

#[test]
fn rejects_invalid_section_group_member_index() {
    let mut bytes = fixture();

    // The second member is the third word in SHT_GROUP contents.
    let second_member_offset = 64 + 1 + 8;
    bytes[second_member_offset..second_member_offset + 4]
        .copy_from_slice(&99u32.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");
    assert!(object.section_group_members(3).is_none());
}

#[test]
fn resolves_section_group_signature_symbol() {
    let bytes = fixture();
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    let group = object.section_group(3).expect("section group must resolve");
    let signature = group
        .signature_symbol()
        .expect("section-group signature symbol must resolve");

    assert_eq!(group.signature_symbol_index, 0);
    assert_eq!(signature.name_index, 0);
}
