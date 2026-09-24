use userspace::file::format::elf::{
    section_link::ValidationError,
    ObjectFile,
};

fn half(bytes: &mut Vec<u8>, value: u16) { bytes.extend_from_slice(&value.to_le_bytes()); }
fn word(bytes: &mut Vec<u8>, value: u32) { bytes.extend_from_slice(&value.to_le_bytes()); }
fn xword(bytes: &mut Vec<u8>, value: u64) { bytes.extend_from_slice(&value.to_le_bytes()); }

fn fixture(link_order: bool, linked_index: u32) -> Vec<u8> {
    const HEADER_SIZE: u64 = 64;
    const REFERENCED_OFFSET: u64 = HEADER_SIZE;
    const REFERENCED_SIZE: u64 = 4;
    const METADATA_OFFSET: u64 = REFERENCED_OFFSET + REFERENCED_SIZE;
    const METADATA_SIZE: u64 = 4;
    const SECTION_HEADER_OFFSET: u64 = METADATA_OFFSET + METADATA_SIZE;

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
    half(&mut bytes, 0);

    bytes.extend_from_slice(&[1, 2, 3, 4]);
    bytes.extend_from_slice(&[5, 6, 7, 8]);

    // Section 0: SHT_NULL.
    bytes.extend_from_slice(&[0u8; 64]);

    // Section 1: referenced section.
    word(&mut bytes, 0);
    word(&mut bytes, 1);
    xword(&mut bytes, 0);
    xword(&mut bytes, 0);
    xword(&mut bytes, REFERENCED_OFFSET);
    xword(&mut bytes, REFERENCED_SIZE);
    word(&mut bytes, 0);
    word(&mut bytes, 0);
    xword(&mut bytes, 1);
    xword(&mut bytes, 0);

    // Section 2: metadata section with optional SHF_LINK_ORDER.
    word(&mut bytes, 0);
    word(&mut bytes, 1);
    xword(&mut bytes, if link_order { 0x80 } else { 0 });
    xword(&mut bytes, 0);
    xword(&mut bytes, METADATA_OFFSET);
    xword(&mut bytes, METADATA_SIZE);
    word(&mut bytes, linked_index);
    word(&mut bytes, 0);
    xword(&mut bytes, 1);
    xword(&mut bytes, 0);

    bytes
}

#[test]
fn resolves_link_order_metadata_relationship() {
    let bytes = fixture(true, 1);
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");
    let relationship = object.link_order(2).expect("SHF_LINK_ORDER relation must resolve");

    assert_eq!(relationship.metadata_index, 2);
    assert_eq!(relationship.referenced_index, 1);
    assert_eq!(relationship.metadata.contents, &[5, 6, 7, 8]);
    assert_eq!(relationship.referenced.contents, &[1, 2, 3, 4]);
}

#[test]
fn ignores_section_without_link_order_flag() {
    let bytes = fixture(false, 1);
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert!(object.link_order(2).is_none());
}

#[test]
fn validates_link_order_target_bounds() {
    let bytes = fixture(true, 7);
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_section_links(),
        Err(ValidationError::LinkOrderTargetOutOfBounds {
            section_index: 2,
            target_section_index: 7,
        }),
    );
}

#[test]
fn rejects_link_order_relationship_with_invalid_section_index() {
    let bytes = fixture(true, 7);
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert!(object.link_order(2).is_none());
}
