use userspace::file::format::elf::ObjectFile;

fn half(bytes: &mut Vec<u8>, value: u16) { bytes.extend_from_slice(&value.to_le_bytes()); }
fn word(bytes: &mut Vec<u8>, value: u32) { bytes.extend_from_slice(&value.to_le_bytes()); }
fn xword(bytes: &mut Vec<u8>, value: u64) { bytes.extend_from_slice(&value.to_le_bytes()); }

fn elf64_note_fixture() -> Vec<u8> {
    const HEADER_SIZE: u64 = 64;
    const PROGRAM_HEADER_SIZE: u64 = 56;
    const NOTE_OFFSET: u64 = HEADER_SIZE + PROGRAM_HEADER_SIZE;
    const NOTE_SIZE: u64 = 24;

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
    half(&mut bytes, PROGRAM_HEADER_SIZE as u16);
    half(&mut bytes, 1);
    half(&mut bytes, 64);
    half(&mut bytes, 0);
    half(&mut bytes, 0);

    // PT_NOTE
    word(&mut bytes, 4);
    word(&mut bytes, 4);
    xword(&mut bytes, NOTE_OFFSET);
    xword(&mut bytes, NOTE_OFFSET);
    xword(&mut bytes, NOTE_OFFSET);
    xword(&mut bytes, NOTE_SIZE);
    xword(&mut bytes, NOTE_SIZE);
    xword(&mut bytes, 8);

    // namesz, descsz, type are always 32-bit words.
    word(&mut bytes, 4);
    word(&mut bytes, 4);
    word(&mut bytes, 7);
    bytes.extend_from_slice(b"GNU\0");
    bytes.extend_from_slice(&[1, 2, 3, 4]);
    bytes.extend_from_slice(&[0u8; 4]); // ELF64 descriptor padding.

    assert_eq!(bytes.len(), (NOTE_OFFSET + NOTE_SIZE) as usize);
    bytes
}

#[test]
fn parses_elf64_note_with_32_bit_fields_and_64_bit_alignment() {
    let bytes = elf64_note_fixture();
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");
    let notes = object
        .note_table_from_program_header(0)
        .expect("PT_NOTE must resolve");

    assert_eq!(notes.len(), 1);
    let note = notes.get(0).expect("first note");
    assert_eq!(note.originator, b"GNU\0");
    assert_eq!(note.r#type, 7);
    assert_eq!(note.descriptor, &[1, 2, 3, 4]);
}
