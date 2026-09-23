use userspace::file::format::elf::{
    section_header::ValidationError,
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

fn section_header(
    bytes: &mut Vec<u8>,
    section_type: u32,
    flags: u64,
    information: u32,
    alignment: u64,
) {
    word(bytes, 0);
    word(bytes, section_type);
    xword(bytes, flags);
    xword(bytes, 0);
    xword(bytes, 0);
    xword(bytes, 0);
    word(bytes, 0);
    word(bytes, information);
    xword(bytes, alignment);
    xword(bytes, 0);
}

fn fixture(flags: u64, information: u32, alignment: u64) -> Vec<u8> {
    const HEADER_SIZE: u64 = 64;
    const SECTION_HEADER_SIZE: u16 = 64;
    const SECTION_HEADER_COUNT: u16 = 2;

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
    xword(&mut bytes, HEADER_SIZE);
    word(&mut bytes, 0);
    half(&mut bytes, HEADER_SIZE as u16);
    half(&mut bytes, 56);
    half(&mut bytes, 0);
    half(&mut bytes, SECTION_HEADER_SIZE);
    half(&mut bytes, SECTION_HEADER_COUNT);
    half(&mut bytes, 0);

    bytes.extend_from_slice(&[0u8; 64]);
    section_header(&mut bytes, 1, flags, information, alignment);

    bytes
}

#[test]
fn accepts_zero_one_and_power_of_two_section_alignment() {
    for alignment in [0, 1, 2, 4, 8, 16] {
        let bytes = fixture(0, 0, alignment);
        let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");
        assert_eq!(object.validate_section_headers(), Ok(()));
    }
}

#[test]
fn rejects_non_power_of_two_section_alignment() {
    let bytes = fixture(0, 0, 3);
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_section_headers(),
        Err(ValidationError::AlignmentNotPowerOfTwo { index: 1 }),
    );
}

#[test]
fn resolves_information_link_as_section_index() {
    let bytes = fixture(0x40, 1, 1);
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(object.validate_section_headers(), Ok(()));
}

#[test]
fn rejects_information_link_outside_section_table() {
    let bytes = fixture(0x40, 2, 1);
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_section_headers(),
        Err(ValidationError::InformationLinkOutOfBounds {
            index: 1,
            target: 2,
        }),
    );
}


#[test]
fn accepts_merge_section_with_integral_entries() {
    let mut bytes = fixture(0x10, 0, 1);

    // Section [1] size and entry size live at offsets +32 and +56.
    let header_offset = 64 + 64;
    bytes[header_offset + 32..header_offset + 40]
        .copy_from_slice(&8u64.to_le_bytes());
    bytes[header_offset + 56..header_offset + 64]
        .copy_from_slice(&4u64.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");
    assert_eq!(object.validate_section_headers(), Ok(()));
}

#[test]
fn rejects_merge_or_strings_section_with_zero_entry_size() {
    for flags in [0x10u64, 0x20u64, 0x30u64] {
        let bytes = fixture(flags, 0, 1);
        let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

        assert_eq!(
            object.validate_section_headers(),
            Err(ValidationError::MergeOrStringsEntrySizeZero { index: 1 }),
        );
    }
}

#[test]
fn rejects_merge_or_strings_section_with_partial_entry() {
    for flags in [0x10u64, 0x20u64, 0x30u64] {
        let mut bytes = fixture(flags, 0, 1);
        let header_offset = 64 + 64;
        bytes[header_offset + 32..header_offset + 40]
            .copy_from_slice(&7u64.to_le_bytes());
        bytes[header_offset + 56..header_offset + 64]
            .copy_from_slice(&4u64.to_le_bytes());

        let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

        assert_eq!(
            object.validate_section_headers(),
            Err(ValidationError::MergeOrStringsSizeNotEntryMultiple { index: 1 }),
        );
    }
}
