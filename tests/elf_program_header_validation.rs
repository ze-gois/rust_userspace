use userspace::file::format::elf::{
    program_header::ValidationError,
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

fn one_segment_fixture() -> Vec<u8> {
    const HEADER_SIZE: u64 = 64;
    const PROGRAM_HEADER_SIZE: u16 = 56;
    const PAYLOAD_OFFSET: u64 = HEADER_SIZE + PROGRAM_HEADER_SIZE as u64;

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
    half(&mut bytes, 1);
    half(&mut bytes, 64);
    half(&mut bytes, 0);
    half(&mut bytes, 0);

    program_header(
        &mut bytes,
        4,
        4,
        PAYLOAD_OFFSET,
        0,
        4,
        4,
        1,
    );
    bytes.extend_from_slice(&[1, 2, 3, 0]);

    bytes
}

fn two_segment_phdr_fixture() -> Vec<u8> {
    const HEADER_SIZE: u64 = 64;
    const PROGRAM_HEADER_SIZE: u16 = 56;
    const PROGRAM_HEADER_COUNT: u16 = 2;
    const TABLE_SIZE: u64 = PROGRAM_HEADER_SIZE as u64 * PROGRAM_HEADER_COUNT as u64;
    const FILE_SIZE: u64 = HEADER_SIZE + TABLE_SIZE;
    const BASE: u64 = 0x400000;

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
        6,
        4,
        HEADER_SIZE,
        BASE + HEADER_SIZE,
        TABLE_SIZE,
        TABLE_SIZE,
        8,
    );
    program_header(
        &mut bytes,
        1,
        5,
        0,
        BASE,
        FILE_SIZE,
        FILE_SIZE,
        0x1000,
    );

    bytes
}

fn program_header_field_offset(index: usize, field_offset: usize) -> usize {
    64 + index * 56 + field_offset
}

#[test]
fn accepts_program_segment_with_file_image_in_bounds() {
    let bytes = one_segment_fixture();
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(object.validate_program_headers(), Ok(()));
}

#[test]
fn rejects_reserved_segment_type() {
    let mut bytes = one_segment_fixture();
    bytes[program_header_field_offset(0, 0)..program_header_field_offset(0, 0) + 4]
        .copy_from_slice(&8u32.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_program_headers(),
        Err(ValidationError::ReservedSegmentType { index: 0, raw: 8 }),
    );
}

#[test]
fn rejects_non_power_of_two_segment_alignment() {
    let mut bytes = one_segment_fixture();
    let offset = program_header_field_offset(0, 48);
    bytes[offset..offset + 8].copy_from_slice(&3u64.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_program_headers(),
        Err(ValidationError::SegmentAlignmentNotPowerOfTwo { index: 0 }),
    );
}

#[test]
fn rejects_incongruent_segment_address_and_offset() {
    let mut bytes = one_segment_fixture();
    let virtual_address = program_header_field_offset(0, 16);
    bytes[virtual_address..virtual_address + 8].copy_from_slice(&1u64.to_le_bytes());
    let alignment = program_header_field_offset(0, 48);
    bytes[alignment..alignment + 8].copy_from_slice(&4u64.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_program_headers(),
        Err(ValidationError::SegmentAddressOffsetIncongruent { index: 0 }),
    );
}

#[test]
fn rejects_segment_file_image_outside_file() {
    let mut bytes = one_segment_fixture();
    let file_size = program_header_field_offset(0, 32);
    bytes[file_size..file_size + 8].copy_from_slice(&5u64.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_program_headers(),
        Err(ValidationError::SegmentFileImageOutsideFile { index: 0 }),
    );
}

#[test]
fn accepts_null_terminated_interpreter_path() {
    let mut bytes = one_segment_fixture();
    bytes[program_header_field_offset(0, 0)..program_header_field_offset(0, 0) + 4]
        .copy_from_slice(&3u32.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(object.validate_program_headers(), Ok(()));
}

#[test]
fn rejects_non_null_terminated_interpreter_path() {
    let mut bytes = one_segment_fixture();
    bytes[program_header_field_offset(0, 0)..program_header_field_offset(0, 0) + 4]
        .copy_from_slice(&3u32.to_le_bytes());
    *bytes.last_mut().expect("payload must exist") = 4;

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_program_headers(),
        Err(ValidationError::InterpreterNotNullTerminated { index: 0 }),
    );
}

#[test]
fn accepts_read_only_thread_local_storage_template() {
    let mut bytes = one_segment_fixture();
    bytes[program_header_field_offset(0, 0)..program_header_field_offset(0, 0) + 4]
        .copy_from_slice(&7u32.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(object.validate_program_headers(), Ok(()));
}

#[test]
fn rejects_thread_local_storage_file_image_larger_than_template() {
    let mut bytes = one_segment_fixture();
    bytes[program_header_field_offset(0, 0)..program_header_field_offset(0, 0) + 4]
        .copy_from_slice(&7u32.to_le_bytes());
    let memory_size = program_header_field_offset(0, 40);
    bytes[memory_size..memory_size + 8].copy_from_slice(&3u64.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_program_headers(),
        Err(ValidationError::ThreadLocalStorageFileImageLargerThanTemplate {
            index: 0,
        }),
    );
}

#[test]
fn rejects_thread_local_storage_flags_other_than_read_only() {
    let mut bytes = one_segment_fixture();
    bytes[program_header_field_offset(0, 0)..program_header_field_offset(0, 0) + 4]
        .copy_from_slice(&7u32.to_le_bytes());
    let flags = program_header_field_offset(0, 4);
    bytes[flags..flags + 4].copy_from_slice(&6u32.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_program_headers(),
        Err(ValidationError::ThreadLocalStorageFlagsNotReadOnly {
            index: 0,
            flags: 6,
        }),
    );
}

#[test]
fn accepts_program_header_table_image_inside_load_segment() {
    let bytes = two_segment_phdr_fixture();
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(object.validate_program_headers(), Ok(()));
}

#[test]
fn rejects_program_header_table_image_mismatch() {
    let mut bytes = two_segment_phdr_fixture();
    let file_size = program_header_field_offset(0, 32);
    bytes[file_size..file_size + 8].copy_from_slice(&56u64.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_program_headers(),
        Err(ValidationError::ProgramHeaderTableImageMismatch { index: 0 }),
    );
}

#[test]
fn rejects_program_header_table_outside_load_image() {
    let mut bytes = two_segment_phdr_fixture();

    let load_file_size = program_header_field_offset(1, 32);
    bytes[load_file_size..load_file_size + 8].copy_from_slice(&64u64.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_program_headers(),
        Err(ValidationError::ProgramHeaderTableNotLoaded { index: 0 }),
    );
}


#[test]
fn rejects_load_file_image_larger_than_memory_image() {
    let mut bytes = one_segment_fixture();
    bytes[program_header_field_offset(0, 0)..program_header_field_offset(0, 0) + 4]
        .copy_from_slice(&1u32.to_le_bytes());
    let memory_size = program_header_field_offset(0, 40);
    bytes[memory_size..memory_size + 8].copy_from_slice(&3u64.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_program_headers(),
        Err(ValidationError::LoadFileImageLargerThanMemoryImage { index: 0 }),
    );
}

#[test]
fn rejects_non_power_of_two_load_alignment() {
    let mut bytes = one_segment_fixture();
    bytes[program_header_field_offset(0, 0)..program_header_field_offset(0, 0) + 4]
        .copy_from_slice(&1u32.to_le_bytes());
    let alignment = program_header_field_offset(0, 48);
    bytes[alignment..alignment + 8].copy_from_slice(&3u64.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_program_headers(),
        Err(ValidationError::LoadAlignmentNotPowerOfTwo { index: 0 }),
    );
}

#[test]
fn rejects_incongruent_load_address_and_offset() {
    let mut bytes = one_segment_fixture();
    bytes[program_header_field_offset(0, 0)..program_header_field_offset(0, 0) + 4]
        .copy_from_slice(&1u32.to_le_bytes());
    let virtual_address = program_header_field_offset(0, 16);
    bytes[virtual_address..virtual_address + 8].copy_from_slice(&1u64.to_le_bytes());
    let alignment = program_header_field_offset(0, 48);
    bytes[alignment..alignment + 8].copy_from_slice(&4u64.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_program_headers(),
        Err(ValidationError::LoadAddressOffsetIncongruent { index: 0 }),
    );
}

#[test]
fn rejects_load_segments_out_of_virtual_address_order() {
    let mut bytes = two_segment_phdr_fixture();
    bytes[program_header_field_offset(0, 0)..program_header_field_offset(0, 0) + 4]
        .copy_from_slice(&1u32.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_program_headers(),
        Err(ValidationError::LoadSegmentsNotOrderedByVirtualAddress {
            previous: 0,
            current: 1,
        }),
    );
}

#[test]
fn rejects_multiple_interpreters() {
    let mut bytes = two_segment_phdr_fixture();
    for index in 0..2 {
        bytes[program_header_field_offset(index, 0)..program_header_field_offset(index, 0) + 4]
            .copy_from_slice(&3u32.to_le_bytes());
    }

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_program_headers(),
        Err(ValidationError::MultipleInterpreters),
    );
}

#[test]
fn rejects_interpreter_after_load_segment() {
    let mut bytes = two_segment_phdr_fixture();
    bytes[program_header_field_offset(0, 0)..program_header_field_offset(0, 0) + 4]
        .copy_from_slice(&1u32.to_le_bytes());
    bytes[program_header_field_offset(1, 0)..program_header_field_offset(1, 0) + 4]
        .copy_from_slice(&3u32.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_program_headers(),
        Err(ValidationError::InterpreterAfterLoad { index: 1 }),
    );
}

#[test]
fn rejects_multiple_program_header_table_images() {
    let mut bytes = two_segment_phdr_fixture();
    bytes[program_header_field_offset(1, 0)..program_header_field_offset(1, 0) + 4]
        .copy_from_slice(&6u32.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_program_headers(),
        Err(ValidationError::MultipleProgramHeaderTableImages),
    );
}

#[test]
fn rejects_program_header_table_image_after_load_segment() {
    let mut bytes = two_segment_phdr_fixture();
    bytes[program_header_field_offset(0, 0)..program_header_field_offset(0, 0) + 4]
        .copy_from_slice(&1u32.to_le_bytes());
    bytes[program_header_field_offset(1, 0)..program_header_field_offset(1, 0) + 4]
        .copy_from_slice(&6u32.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_program_headers(),
        Err(ValidationError::ProgramHeaderTableImageAfterLoad { index: 1 }),
    );
}

#[test]
fn rejects_reserved_shared_library_segment() {
    let mut bytes = one_segment_fixture();
    bytes[program_header_field_offset(0, 0)..program_header_field_offset(0, 0) + 4]
        .copy_from_slice(&5u32.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_program_headers(),
        Err(ValidationError::SharedLibrarySegment { index: 0 }),
    );
}
