use userspace::file::format::elf::{
    section_group::ValidationError,
    ObjectFile,
};

const HEADER_SIZE: u64 = 64;
const STRING_SIZE: u64 = 11;
const SYMBOL_SIZE: u64 = 48;
const GROUP_SIZE: u64 = 12;
const STRING_OFFSET: u64 = HEADER_SIZE;
const SYMBOL_OFFSET: u64 = STRING_OFFSET + STRING_SIZE;
const GROUP_OFFSET: u64 = SYMBOL_OFFSET + SYMBOL_SIZE;
const SECTION_HEADER_OFFSET: u64 = GROUP_OFFSET + GROUP_SIZE;

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
    flags: u64,
    offset: u64,
    size: u64,
    link: u32,
    information: u32,
    alignment: u64,
    entry_size: u64,
) {
    word(bytes, 0);
    word(bytes, section_type);
    xword(bytes, flags);
    xword(bytes, 0);
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
    half(&mut bytes, 6);
    half(&mut bytes, 0);

    bytes.extend_from_slice(b"\0signature\0");

    symbol(&mut bytes, 0, 0, 0);
    symbol(&mut bytes, 1, 0x12, 4);

    word(&mut bytes, 1);
    word(&mut bytes, 4);
    word(&mut bytes, 5);

    bytes.extend_from_slice(&[0u8; 64]);
    section_header(&mut bytes, 3, 0, STRING_OFFSET, STRING_SIZE, 0, 0, 1, 0);
    section_header(
        &mut bytes,
        2,
        0,
        SYMBOL_OFFSET,
        SYMBOL_SIZE,
        1,
        1,
        8,
        24,
    );
    section_header(
        &mut bytes,
        17,
        0,
        GROUP_OFFSET,
        GROUP_SIZE,
        2,
        1,
        4,
        4,
    );
    section_header(&mut bytes, 1, 0x200, 0, 0, 0, 0, 1, 0);
    section_header(&mut bytes, 1, 0x200, 0, 0, 0, 0, 1, 0);

    bytes
}

fn section_flags_offset(index: usize) -> usize {
    SECTION_HEADER_OFFSET as usize + index * 64 + 8
}

#[test]
fn accepts_relocatable_section_group() {
    let bytes = fixture();
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(object.validate_section_groups(), Ok(()));
}

#[test]
fn rejects_section_group_outside_relocatable_object() {
    let mut bytes = fixture();
    bytes[16..18].copy_from_slice(&2u16.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_section_groups(),
        Err(ValidationError::GroupOutsideRelocatableObject { index: 3 }),
    );
}

#[test]
fn rejects_nonzero_group_section_flags() {
    let mut bytes = fixture();
    let offset = section_flags_offset(3);
    bytes[offset..offset + 8].copy_from_slice(&0x2u64.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_section_groups(),
        Err(ValidationError::GroupSectionFlagsNotZero { index: 3 }),
    );
}

#[test]
fn rejects_member_without_group_flag() {
    let mut bytes = fixture();
    let offset = section_flags_offset(4);
    bytes[offset..offset + 8].copy_from_slice(&0u64.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_section_groups(),
        Err(ValidationError::MemberMissingGroupFlag {
            group_index: 3,
            member_index: 4,
        }),
    );
}

#[test]
fn rejects_member_that_precedes_group_header() {
    let mut bytes = fixture();

    // First member word in SHT_GROUP contents.
    let first_member_offset = GROUP_OFFSET as usize + 4;
    bytes[first_member_offset..first_member_offset + 4]
        .copy_from_slice(&2u32.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_section_groups(),
        Err(ValidationError::MemberNotAfterGroup {
            group_index: 3,
            member_index: 2,
        }),
    );
}

#[test]
fn rejects_group_flag_without_group_membership() {
    let mut bytes = fixture();
    let offset = section_flags_offset(1);
    bytes[offset..offset + 8].copy_from_slice(&0x200u64.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_section_groups(),
        Err(ValidationError::GroupFlagWithoutGroup { member_index: 1 }),
    );
}


#[test]
fn rejects_member_in_multiple_groups() {
    let mut bytes = fixture();

    // Expand section-header count from 6 to 7.
    bytes[60..62].copy_from_slice(&7u16.to_le_bytes());

    // Insert a second SHT_GROUP header after the original table entries.
    section_header(
        &mut bytes,
        17,
        0,
        GROUP_OFFSET,
        GROUP_SIZE,
        2,
        1,
        4,
        4,
    );

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_section_groups(),
        Err(ValidationError::MemberInMultipleGroups { member_index: 4 }),
    );
}
