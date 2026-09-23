use userspace::file::format::elf::ObjectFile;

const BASE: u64 = 0x400000;
const HEADER_SIZE: u64 = 64;
const PROGRAM_HEADER_SIZE: u64 = 56;
const PROGRAM_HEADER_COUNT: u64 = 2;
const DYNAMIC_OFFSET: u64 = HEADER_SIZE + PROGRAM_HEADER_SIZE * PROGRAM_HEADER_COUNT;
const DYNAMIC_SIZE: u64 = 8 * 16;
const STRING_OFFSET: u64 = DYNAMIC_OFFSET + DYNAMIC_SIZE;
const STRING_SIZE: u64 = 30;
const TOTAL_SIZE: u64 = STRING_OFFSET + STRING_SIZE;

fn half(bytes: &mut Vec<u8>, value: u16) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn word(bytes: &mut Vec<u8>, value: u32) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn sxword(bytes: &mut Vec<u8>, value: i64) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn xword(bytes: &mut Vec<u8>, value: u64) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn dynamic_entry(bytes: &mut Vec<u8>, tag: i64, payload: u64) {
    sxword(bytes, tag);
    xword(bytes, payload);
}

fn fixture() -> Vec<u8> {
    let mut bytes = Vec::with_capacity(TOTAL_SIZE as usize);

    bytes.extend_from_slice(&[
        0x7f, b'E', b'L', b'F',
        2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]);
    half(&mut bytes, 3);
    half(&mut bytes, 0x3e);
    word(&mut bytes, 1);
    xword(&mut bytes, 0);
    xword(&mut bytes, HEADER_SIZE);
    xword(&mut bytes, 0);
    word(&mut bytes, 0);
    half(&mut bytes, HEADER_SIZE as u16);
    half(&mut bytes, PROGRAM_HEADER_SIZE as u16);
    half(&mut bytes, PROGRAM_HEADER_COUNT as u16);
    half(&mut bytes, 64);
    half(&mut bytes, 0);
    half(&mut bytes, 0);

    word(&mut bytes, 1);
    word(&mut bytes, 4);
    xword(&mut bytes, 0);
    xword(&mut bytes, BASE);
    xword(&mut bytes, BASE);
    xword(&mut bytes, TOTAL_SIZE);
    xword(&mut bytes, TOTAL_SIZE);
    xword(&mut bytes, 0x1000);

    word(&mut bytes, 2);
    word(&mut bytes, 4);
    xword(&mut bytes, DYNAMIC_OFFSET);
    xword(&mut bytes, BASE + DYNAMIC_OFFSET);
    xword(&mut bytes, BASE + DYNAMIC_OFFSET);
    xword(&mut bytes, DYNAMIC_SIZE);
    xword(&mut bytes, DYNAMIC_SIZE);
    xword(&mut bytes, 8);

    dynamic_entry(&mut bytes, 5, BASE + STRING_OFFSET);
    dynamic_entry(&mut bytes, 10, STRING_SIZE);
    dynamic_entry(&mut bytes, 1, 1);
    dynamic_entry(&mut bytes, 14, 17);
    dynamic_entry(&mut bytes, 1, 9);
    dynamic_entry(&mut bytes, 29, 25);
    dynamic_entry(&mut bytes, 1, 1);
    dynamic_entry(&mut bytes, 0, 0);

    bytes.extend_from_slice(b"\0liba.so\0libb.so\0self.so\0/lib\0");

    assert_eq!(bytes.len(), TOTAL_SIZE as usize);
    bytes
}

#[test]
fn preserves_needed_shared_object_order_and_dynamic_entry_indices() {
    let bytes = fixture();
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");
    let dependencies = object
        .shared_object_dependencies_from_program_header(1)
        .expect("shared object dependencies must resolve");

    assert_eq!(dependencies.needed.len(), 3);

    assert_eq!(dependencies.needed[0].dynamic_entry_index, 2);
    assert_eq!(dependencies.needed[0].name, "liba.so");

    assert_eq!(dependencies.needed[1].dynamic_entry_index, 4);
    assert_eq!(dependencies.needed[1].name, "libb.so");

    assert_eq!(dependencies.needed[2].dynamic_entry_index, 6);
    assert_eq!(dependencies.needed[2].name, "liba.so");
}

#[test]
fn keeps_shared_object_identity_and_search_metadata_separate_from_needed_dependencies() {
    let bytes = fixture();
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");
    let dependencies = object
        .shared_object_dependencies_from_program_header(1)
        .expect("shared object dependencies must resolve");

    assert_eq!(dependencies.shared_object_name, Some("self.so"));
    assert_eq!(dependencies.runtime_search_path, None);
    assert_eq!(dependencies.run_path, Some("/lib"));
}
