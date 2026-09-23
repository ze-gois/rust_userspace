use userspace::file::format::elf::{
    program_header::Flags,
    program_image::Error,
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

fn program_header(
    bytes: &mut Vec<u8>,
    segment_type: u32,
    flags: u32,
    offset: u64,
    virtual_address: u64,
    file_size: u64,
    memory_size: u64,
    alignment: u64,
) {
    word(bytes, segment_type);
    word(bytes, flags);
    xword(bytes, offset);
    xword(bytes, virtual_address);
    xword(bytes, 0);
    xword(bytes, file_size);
    xword(bytes, memory_size);
    xword(bytes, alignment);
}

fn fixture() -> Vec<u8> {
    const HEADER_SIZE: u64 = 64;
    const PROGRAM_HEADER_SIZE: u16 = 56;
    const PROGRAM_HEADER_COUNT: u16 = 2;
    const PAYLOAD_OFFSET: u64 =
        HEADER_SIZE + PROGRAM_HEADER_SIZE as u64 * PROGRAM_HEADER_COUNT as u64;

    let mut bytes = Vec::new();

    bytes.extend_from_slice(&[
        0x7f, b'E', b'L', b'F',
        2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]);
    half(&mut bytes, 2);
    half(&mut bytes, 0x3e);
    word(&mut bytes, 1);
    xword(&mut bytes, 0);
    xword(&mut bytes, HEADER_SIZE);
    xword(&mut bytes, 0);
    word(&mut bytes, 0);
    half(&mut bytes, HEADER_SIZE as u16);
    half(&mut bytes, PROGRAM_HEADER_SIZE);
    half(&mut bytes, PROGRAM_HEADER_COUNT);
    half(&mut bytes, 64);
    half(&mut bytes, 0);
    half(&mut bytes, 0);

    program_header(
        &mut bytes,
        1,
        Flags::READ | Flags::EXECUTE,
        PAYLOAD_OFFSET,
        0x400000,
        4,
        8,
        1,
    );
    program_header(
        &mut bytes,
        1,
        Flags::READ | Flags::WRITE,
        PAYLOAD_OFFSET + 4,
        0x500000,
        2,
        2,
        1,
    );

    bytes.extend_from_slice(&[0x10, 0x20, 0x30, 0x40, 0xaa, 0xbb]);
    bytes
}

fn program_header_field_offset(index: usize, field_offset: usize) -> usize {
    64 + index * 56 + field_offset
}

#[test]
fn builds_program_image_from_load_segments() {
    let bytes = fixture();
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");
    let image = object.program_image().expect("program image must build");

    assert_eq!(image.len(), 2);

    let first = image.segments[0];
    assert_eq!(first.program_header_index, 0);
    assert_eq!(first.link_time_virtual_address, 0x400000);
    assert_eq!(first.file_image, &[0x10, 0x20, 0x30, 0x40]);
    assert_eq!(first.memory_size(), 8);
    assert_eq!(first.link_time_end_virtual_address, 0x400008);

    let second = image.segments[1];
    assert_eq!(second.program_header_index, 1);
    assert_eq!(second.link_time_virtual_address, 0x500000);
    assert_eq!(second.file_image, &[0xaa, 0xbb]);
    assert_eq!(second.memory_size(), 2);
    assert_eq!(second.link_time_end_virtual_address, 0x500002);
}

#[test]
fn records_zero_fill_after_file_image() {
    let bytes = fixture();
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");
    let image = object.program_image().expect("program image must build");

    let first = image.segments[0];
    assert_eq!(first.zero_fill.link_time_virtual_address, 0x400004);
    assert_eq!(first.zero_fill.size, 4);
    assert!(!first.zero_fill.is_empty());

    let second = image.segments[1];
    assert_eq!(second.zero_fill.link_time_virtual_address, 0x500002);
    assert_eq!(second.zero_fill.size, 0);
    assert!(second.zero_fill.is_empty());
}

#[test]
fn preserves_elf_segment_flags_without_os_mapping_policy() {
    let bytes = fixture();
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");
    let image = object.program_image().expect("program image must build");

    let first = image.segments[0];
    assert!(first.flags.readable());
    assert!(!first.flags.writable());
    assert!(first.flags.executable());
    assert_eq!(first.flags.raw(), Flags::READ | Flags::EXECUTE);

    let second = image.segments[1];
    assert!(second.flags.readable());
    assert!(second.flags.writable());
    assert!(!second.flags.executable());
    assert_eq!(second.flags.raw(), Flags::READ | Flags::WRITE);
}

#[test]
fn ignores_non_load_program_headers_in_program_image() {
    let mut bytes = fixture();
    bytes[program_header_field_offset(1, 0)..program_header_field_offset(1, 0) + 4]
        .copy_from_slice(&4u32.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");
    let image = object.program_image().expect("program image must build");

    assert_eq!(image.len(), 1);
    assert_eq!(image.segments[0].program_header_index, 0);
}

#[test]
fn rejects_program_image_file_larger_than_memory_image() {
    let mut bytes = fixture();
    let memory_size = program_header_field_offset(0, 40);
    bytes[memory_size..memory_size + 8].copy_from_slice(&3u64.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert!(matches!(
        object.program_image(),
        Err(Error::FileImageLargerThanMemoryImage { index: 0 }),
    ));
}

#[test]
fn rejects_program_image_file_image_outside_file() {
    let mut bytes = fixture();
    let file_size = program_header_field_offset(0, 32);
    bytes[file_size..file_size + 8].copy_from_slice(&7u64.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert!(matches!(
        object.program_image(),
        Err(Error::FileImageUnavailable { index: 0 }),
    ));
}

#[test]
fn rejects_program_image_virtual_address_range_overflow() {
    let mut bytes = fixture();
    let virtual_address = program_header_field_offset(0, 16);
    bytes[virtual_address..virtual_address + 8]
        .copy_from_slice(&(u64::MAX - 3).to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert!(matches!(
        object.program_image(),
        Err(Error::VirtualAddressRangeOverflow { index: 0 }),
    ));
}
