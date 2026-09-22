use userspace::file::format::elf::{
    relocation::relative::{Entry, SectionValidationError},
    ObjectFile,
};

fn half(bytes: &mut Vec<u8>, value: u16) { bytes.extend_from_slice(&value.to_le_bytes()); }
fn word(bytes: &mut Vec<u8>, value: u32) { bytes.extend_from_slice(&value.to_le_bytes()); }
fn xword(bytes: &mut Vec<u8>, value: u64) { bytes.extend_from_slice(&value.to_le_bytes()); }

fn fixture(object_type: u16, entry_size: u64) -> Vec<u8> {
    const HEADER_SIZE: u64 = 64;
    const RELR_OFFSET: u64 = HEADER_SIZE;
    const RELR_SIZE: u64 = 16;
    const SECTION_HEADER_OFFSET: u64 = RELR_OFFSET + RELR_SIZE;
    const SECTION_HEADER_SIZE: u16 = 64;
    const SECTION_HEADER_COUNT: u16 = 2;

    let mut bytes = Vec::new();

    bytes.extend_from_slice(&[
        0x7f, b'E', b'L', b'F',
        2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]);
    half(&mut bytes, object_type);
    half(&mut bytes, 0x3e);
    word(&mut bytes, 1);
    xword(&mut bytes, 0);
    xword(&mut bytes, 0);
    xword(&mut bytes, SECTION_HEADER_OFFSET);
    word(&mut bytes, 0);
    half(&mut bytes, HEADER_SIZE as u16);
    half(&mut bytes, 56);
    half(&mut bytes, 0);
    half(&mut bytes, SECTION_HEADER_SIZE);
    half(&mut bytes, SECTION_HEADER_COUNT);
    half(&mut bytes, 0);

    xword(&mut bytes, 0x401000);
    xword(&mut bytes, 0x7);

    // Section header 0: SHT_NULL.
    bytes.extend_from_slice(&[0u8; 64]);

    // Section header 1: SHT_RELR.
    word(&mut bytes, 0);
    word(&mut bytes, 19);
    xword(&mut bytes, 0);
    xword(&mut bytes, 0);
    xword(&mut bytes, RELR_OFFSET);
    xword(&mut bytes, RELR_SIZE);
    word(&mut bytes, 0);
    word(&mut bytes, 0);
    xword(&mut bytes, 8);
    xword(&mut bytes, entry_size);

    bytes
}

#[test]
fn resolves_relative_relocation_section() {
    let bytes = fixture(2, 8);
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    object
        .validate_relative_relocation_section(1)
        .expect("SHT_RELR must conform");

    let table = object
        .relative_relocation_table(1)
        .expect("SHT_RELR table must resolve");

    assert_eq!(table.len(), 2);
    let entries: Vec<_> = table.iter().copied().collect();
    assert_eq!(entries, vec![Entry::Address(0x401000), Entry::Bitmap(7)]);
}

#[test]
fn rejects_relative_relocation_section_in_relocatable_object() {
    let bytes = fixture(1, 8);
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_relative_relocation_section(1),
        Err(SectionValidationError::ObjectTypeNotExecutableOrSharedObject),
    );
}

#[test]
fn rejects_wrong_relative_relocation_section_entry_size() {
    let bytes = fixture(2, 4);
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_relative_relocation_section(1),
        Err(SectionValidationError::EntrySizeMismatch),
    );
}
