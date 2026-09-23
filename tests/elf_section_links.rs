use userspace::file::format::elf::{
    section_link::{Meaning, ValidationError},
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
    information: u32,
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
    half(&mut bytes, 5);
    half(&mut bytes, 0);

    bytes.push(0);

    // [0] SHT_NULL
    bytes.extend_from_slice(&[0u8; 64]);
    // [1] SHT_STRTAB
    section_header(&mut bytes, 3, STRING_OFFSET, STRING_SIZE, 0, 0, 0);
    // [2] SHT_SYMTAB -> string table [1]
    section_header(&mut bytes, 2, 0, 0, 1, 0, 24);
    // [3] SHT_RELA -> symbol table [2]
    section_header(&mut bytes, 4, 0, 0, 2, 4, 24);
    // [4] SHT_PROGBITS relocation target
    section_header(&mut bytes, 1, 0, 0, 0, 0, 0);

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

#[test]
fn resolves_relocation_target_section_information() {
    let bytes = fixture();
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    let target = object
        .relocation_target_section(3)
        .expect("relocation target must resolve");
    assert_eq!(target.section_index, 4);
}

#[test]
fn leaves_undefined_relocation_target_without_relation() {
    let mut bytes = fixture();

    // Section [3] starts after the ELF header, one byte of string data,
    // and three preceding section headers. sh_info is 44 bytes into Elf64_Shdr.
    let relocation_information_offset = 64 + 1 + (3 * 64) + 44;
    bytes[relocation_information_offset..relocation_information_offset + 4]
        .copy_from_slice(&0u32.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");
    assert!(object.relocation_target_section(3).is_none());
}


fn section_header_field_offset(section_index: usize, field_offset: usize) -> usize {
    64 + 1 + section_index * 64 + field_offset
}

#[test]
fn validates_sh_link_and_sh_info_table() {
    let bytes = fixture();
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(object.validate_section_links(), Ok(()));
}

#[test]
fn rejects_string_table_link_to_wrong_section_type() {
    let mut bytes = fixture();

    let link_offset = section_header_field_offset(2, 40);
    bytes[link_offset..link_offset + 4].copy_from_slice(&4u32.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_section_links(),
        Err(ValidationError::LinkNotStringTable {
            section_index: 2,
            linked_section_index: 4,
        }),
    );
}

#[test]
fn rejects_symbol_table_link_to_wrong_section_type() {
    let mut bytes = fixture();

    let link_offset = section_header_field_offset(3, 40);
    bytes[link_offset..link_offset + 4].copy_from_slice(&1u32.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_section_links(),
        Err(ValidationError::LinkNotSymbolTable {
            section_index: 3,
            linked_section_index: 1,
        }),
    );
}

#[test]
fn rejects_required_zero_information_fields() {
    for section_type in [5u32, 6u32, 18u32] {
        let mut bytes = fixture();

        let type_offset = section_header_field_offset(4, 4);
        bytes[type_offset..type_offset + 4].copy_from_slice(&section_type.to_le_bytes());

        let link_offset = section_header_field_offset(4, 40);
        let link = if section_type == 6 { 1u32 } else { 2u32 };
        bytes[link_offset..link_offset + 4].copy_from_slice(&link.to_le_bytes());

        let information_offset = section_header_field_offset(4, 44);
        bytes[information_offset..information_offset + 4]
            .copy_from_slice(&1u32.to_le_bytes());

        let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

        assert_eq!(
            object.validate_section_links(),
            Err(ValidationError::InformationMustBeZero {
                section_index: 4,
                information: 1,
            }),
        );
    }
}

#[test]
fn rejects_relocation_target_outside_section_table() {
    let mut bytes = fixture();

    let information_offset = section_header_field_offset(3, 44);
    bytes[information_offset..information_offset + 4]
        .copy_from_slice(&99u32.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_section_links(),
        Err(ValidationError::RelocationTargetOutOfBounds {
            section_index: 3,
            target_section_index: 99,
        }),
    );
}
