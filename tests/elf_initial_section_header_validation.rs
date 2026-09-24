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
    const HEADER_SIZE: u64 = 64;
    const SECTION_HEADER_OFFSET: u64 = HEADER_SIZE;

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
    half(&mut bytes, 2);
    half(&mut bytes, 0);

    section_header(&mut bytes, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
    section_header(&mut bytes, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0);

    bytes
}

fn initial_field_offset(field_offset: usize) -> usize {
    64 + field_offset
}

#[test]
fn accepts_canonical_initial_section_header() {
    let bytes = fixture();
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(object.validate_section_headers(), Ok(()));
}

#[test]
fn rejects_noncanonical_fixed_fields_in_initial_section_header() {
    let cases: &[(usize, &[u8], ValidationError)] = &[
        (0, &1u32.to_le_bytes(), ValidationError::UndefinedSectionNameNotZero),
        (4, &1u32.to_le_bytes(), ValidationError::UndefinedSectionTypeNotNull),
        (8, &1u64.to_le_bytes(), ValidationError::UndefinedSectionFlagsNotZero),
        (16, &1u64.to_le_bytes(), ValidationError::UndefinedSectionAddressNotZero),
        (24, &1u64.to_le_bytes(), ValidationError::UndefinedSectionOffsetNotZero),
        (44, &1u32.to_le_bytes(), ValidationError::UndefinedSectionInformationNotZero),
        (48, &1u64.to_le_bytes(), ValidationError::UndefinedSectionAlignmentNotZero),
        (56, &1u64.to_le_bytes(), ValidationError::UndefinedSectionEntrySizeNotZero),
    ];

    for (field_offset, value, expected) in cases {
        let mut bytes = fixture();
        let start = initial_field_offset(*field_offset);
        bytes[start..start + value.len()].copy_from_slice(value);

        let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");
        assert_eq!(object.validate_section_headers(), Err(*expected));
    }
}

#[test]
fn rejects_initial_size_when_section_count_is_direct() {
    let mut bytes = fixture();
    let start = initial_field_offset(32);
    bytes[start..start + 8].copy_from_slice(&2u64.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_section_headers(),
        Err(ValidationError::DirectSectionCountWithInitialSize { size: 2 }),
    );
}

#[test]
fn rejects_extended_section_count_below_reserved_range() {
    let mut bytes = fixture();

    // e_shnum = SHN_UNDEF escape.
    bytes[60..62].copy_from_slice(&0u16.to_le_bytes());
    let start = initial_field_offset(32);
    bytes[start..start + 8].copy_from_slice(&2u64.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_section_headers(),
        Err(ValidationError::ExtendedSectionCountBelowReservedRange { count: 2 }),
    );
}

#[test]
fn rejects_initial_link_when_section_name_index_is_direct() {
    let mut bytes = fixture();

    // Direct section-name string table index [1].
    bytes[62..64].copy_from_slice(&1u16.to_le_bytes());
    let start = initial_field_offset(40);
    bytes[start..start + 4].copy_from_slice(&1u32.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_section_headers(),
        Err(ValidationError::DirectSectionNameStringTableWithInitialLink { link: 1 }),
    );
}

#[test]
fn rejects_extended_section_name_index_below_reserved_range() {
    let mut bytes = fixture();

    // e_shstrndx = SHN_XINDEX, with actual index [1] in sh_link.
    bytes[62..64].copy_from_slice(&0xffffu16.to_le_bytes());
    let start = initial_field_offset(40);
    bytes[start..start + 4].copy_from_slice(&1u32.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_section_headers(),
        Err(
            ValidationError::ExtendedSectionNameStringTableBelowReservedRange {
                index: 1,
            },
        ),
    );
}
