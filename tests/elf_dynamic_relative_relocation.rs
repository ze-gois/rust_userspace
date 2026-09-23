use userspace::file::format::elf::{
    dynamic::validation::ValidationError,
    relocation::relative::Entry,
    ObjectFile,
};

fn half(bytes: &mut Vec<u8>, value: u16) { bytes.extend_from_slice(&value.to_le_bytes()); }
fn word(bytes: &mut Vec<u8>, value: u32) { bytes.extend_from_slice(&value.to_le_bytes()); }
fn sxword(bytes: &mut Vec<u8>, value: i64) { bytes.extend_from_slice(&value.to_le_bytes()); }
fn xword(bytes: &mut Vec<u8>, value: u64) { bytes.extend_from_slice(&value.to_le_bytes()); }
fn dynamic_entry(bytes: &mut Vec<u8>, tag: i64, payload: u64) { sxword(bytes, tag); xword(bytes, payload); }

fn fixture(entry_size: u64) -> Vec<u8> {
    const BASE: u64 = 0x400000;
    const HEADER: u64 = 64;
    const PH: u64 = 56;
    const PH_COUNT: u64 = 2;
    const DYNAMIC_OFFSET: u64 = HEADER + PH * PH_COUNT;
    const DYNAMIC_SIZE: u64 = 4 * 16;
    const RELR_OFFSET: u64 = DYNAMIC_OFFSET + DYNAMIC_SIZE;
    const RELR_SIZE: u64 = 16;
    const TOTAL: u64 = RELR_OFFSET + RELR_SIZE;

    let mut bytes = Vec::with_capacity(TOTAL as usize);
    bytes.extend_from_slice(&[0x7f,b'E',b'L',b'F',2,1,1,0,0,0,0,0,0,0,0,0]);
    half(&mut bytes, 3); half(&mut bytes, 0x3e); word(&mut bytes, 1);
    xword(&mut bytes, BASE); xword(&mut bytes, HEADER); xword(&mut bytes, 0);
    word(&mut bytes, 0); half(&mut bytes, 64); half(&mut bytes, 56);
    half(&mut bytes, 2); half(&mut bytes, 64); half(&mut bytes, 0); half(&mut bytes, 0);

    word(&mut bytes, 1); word(&mut bytes, 4); xword(&mut bytes, 0);
    xword(&mut bytes, BASE); xword(&mut bytes, BASE); xword(&mut bytes, TOTAL);
    xword(&mut bytes, TOTAL); xword(&mut bytes, 0x1000);

    word(&mut bytes, 2); word(&mut bytes, 4); xword(&mut bytes, DYNAMIC_OFFSET);
    xword(&mut bytes, BASE + DYNAMIC_OFFSET); xword(&mut bytes, BASE + DYNAMIC_OFFSET);
    xword(&mut bytes, DYNAMIC_SIZE); xword(&mut bytes, DYNAMIC_SIZE); xword(&mut bytes, 8);

    dynamic_entry(&mut bytes, 35, RELR_SIZE);
    dynamic_entry(&mut bytes, 36, BASE + RELR_OFFSET);
    dynamic_entry(&mut bytes, 37, entry_size);
    dynamic_entry(&mut bytes, 0, 0);

    xword(&mut bytes, 0x401000);
    xword(&mut bytes, 0x0000_0000_0000_0007);

    assert_eq!(bytes.len(), TOTAL as usize);
    bytes
}

#[test]
fn resolves_dynamic_relative_relocation_table() {
    let bytes = fixture(8);
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");
    object.validate_dynamic_relative_relocation(1).expect("RELR metadata must conform");
    let table = object.relative_relocation_table_from_program_header(1).expect("RELR table must resolve");

    assert_eq!(table.len(), 2);
    let entries: Vec<_> = table.iter().copied().collect();
    assert_eq!(entries, vec![Entry::Address(0x401000), Entry::Bitmap(7)]);
}

#[test]
fn rejects_wrong_dynamic_relative_relocation_entry_size() {
    let bytes = fixture(4);
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");
    assert_eq!(
        object.validate_dynamic_relative_relocation(1),
        Err(ValidationError::RelativeRelocationEntrySizeMismatch),
    );
}


#[test]
fn rejects_dynamic_relative_relocation_table_starting_with_bitmap() {
    let mut bytes = fixture(8);

    // First Elf64_Relr entry begins immediately after the dynamic array.
    let first_relr = 64 + 2 * 56 + 4 * 16;
    bytes[first_relr..first_relr + 8].copy_from_slice(&3u64.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_dynamic_relative_relocation(1),
        Err(ValidationError::RelativeRelocationFirstEntryMustBeAddress),
    );
}

#[test]
fn rejects_unmapped_dynamic_relative_relocation_table() {
    let mut bytes = fixture(8);

    // DT_RELR payload is entry [1], d_un at +8.
    let relr_payload = 64 + 2 * 56 + 16 + 8;
    bytes[relr_payload..relr_payload + 8]
        .copy_from_slice(&0x500000u64.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_dynamic_relative_relocation(1),
        Err(ValidationError::RelativeRelocationTableUnavailable),
    );
}
