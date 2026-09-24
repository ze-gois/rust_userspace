use userspace::file::format::elf::{
    section_header::ValidationError,
    ObjectFile,
};

const HEADER_SIZE: u64 = 64;
const STRING_OFFSET: u64 = HEADER_SIZE;
const STRING_SIZE: u64 = 6;
const DATA_OFFSET: u64 = STRING_OFFSET + STRING_SIZE;
const DATA_SIZE: u64 = 4;
const SECTION_HEADER_OFFSET: u64 = DATA_OFFSET + DATA_SIZE;

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
    name: u32,
    section_type: u32,
    flags: u64,
    address: u64,
    offset: u64,
    size: u64,
    link: u32,
    information: u32,
    alignment: u64,
    entry_size: u64,
) {
    word(bytes, name);
    word(bytes, section_type);
    xword(bytes, flags);
    xword(bytes, address);
    xword(bytes, offset);
    xword(bytes, size);
    word(bytes, link);
    word(bytes, information);
    xword(bytes, alignment);
    xword(bytes, entry_size);
}

fn fixture() -> Vec<u8> {
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
    half(&mut bytes, 3);
    half(&mut bytes, 1);

    bytes.extend_from_slice(b"\0text\0");
    bytes.extend_from_slice(&[1, 2, 3, 4]);

    section_header(&mut bytes, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
    section_header(
        &mut bytes,
        0,
        3,
        0,
        0,
        STRING_OFFSET,
        STRING_SIZE,
        0,
        0,
        1,
        0,
    );
    section_header(
        &mut bytes,
        1,
        1,
        0,
        0,
        DATA_OFFSET,
        DATA_SIZE,
        0,
        0,
        1,
        0,
    );

    bytes
}

fn section_header_offset(index: usize) -> usize {
    SECTION_HEADER_OFFSET as usize + index * 64
}

#[test]
fn accepts_non_overlapping_sections_with_valid_names() {
    let bytes = fixture();
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(object.validate_section_headers(), Ok(()));
    assert_eq!(object.section_name(2), Some("text"));
}

#[test]
fn rejects_misaligned_section_address() {
    let mut bytes = fixture();
    let header = section_header_offset(2);

    bytes[header + 8..header + 16].copy_from_slice(&2u64.to_le_bytes());
    bytes[header + 16..header + 24].copy_from_slice(&3u64.to_le_bytes());
    bytes[header + 48..header + 56].copy_from_slice(&4u64.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_section_headers(),
        Err(ValidationError::AddressMisaligned { index: 2 }),
    );
}

#[test]
fn rejects_section_outside_file() {
    let mut bytes = fixture();
    let header = section_header_offset(2);
    let offset = bytes.len() as u64 - 1;

    bytes[header + 24..header + 32].copy_from_slice(&offset.to_le_bytes());
    bytes[header + 32..header + 40].copy_from_slice(&4u64.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_section_headers(),
        Err(ValidationError::SectionOutsideFile { index: 2 }),
    );
}

#[test]
fn rejects_overlapping_file_sections() {
    let mut bytes = fixture();
    let header = section_header_offset(2);

    bytes[header + 24..header + 32].copy_from_slice(&68u64.to_le_bytes());
    bytes[header + 32..header + 40].copy_from_slice(&4u64.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_section_headers(),
        Err(ValidationError::SectionsOverlap {
            first: 1,
            second: 2,
        }),
    );
}

#[test]
fn permits_nobits_conceptual_offset_outside_file() {
    let mut bytes = fixture();
    let header = section_header_offset(2);

    bytes[header + 4..header + 8].copy_from_slice(&8u32.to_le_bytes());
    bytes[header + 24..header + 32].copy_from_slice(&(u64::MAX - 16).to_le_bytes());
    bytes[header + 32..header + 40].copy_from_slice(&128u64.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(object.validate_section_headers(), Ok(()));
}

#[test]
fn rejects_section_name_outside_section_name_string_table() {
    let mut bytes = fixture();
    let header = section_header_offset(2);

    bytes[header..header + 4].copy_from_slice(&6u32.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_section_headers(),
        Err(ValidationError::InvalidSectionName {
            index: 2,
            name_index: 6,
        }),
    );
}

#[test]
fn rejects_section_name_table_with_non_string_table_type() {
    let mut bytes = fixture();
    let header = section_header_offset(1);

    bytes[header + 4..header + 8].copy_from_slice(&1u32.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_section_headers(),
        Err(ValidationError::InvalidSectionNameStringTableType { index: 1 }),
    );
}
