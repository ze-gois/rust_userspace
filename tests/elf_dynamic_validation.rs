use userspace::file::format::elf::{
    dynamic::validation::ValidationError,
    ObjectFile,
};

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

fn fixture(object_type: u16, extra: &[(i64, u64)]) -> Vec<u8> {
    const BASE: u64 = 0x400000;
    const HEADER_SIZE: u64 = 64;
    const PROGRAM_HEADER_SIZE: u64 = 56;
    const PROGRAM_HEADER_COUNT: u64 = 2;
    const DYNAMIC_OFFSET: u64 = HEADER_SIZE + PROGRAM_HEADER_SIZE * PROGRAM_HEADER_COUNT;

    let dynamic_count = 6 + extra.len() as u64;
    let dynamic_size = dynamic_count * 16;
    let total_size = DYNAMIC_OFFSET + dynamic_size;

    let mut bytes = Vec::with_capacity(total_size as usize);

    bytes.extend_from_slice(&[
        0x7f, b'E', b'L', b'F',
        2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]);
    half(&mut bytes, object_type);
    half(&mut bytes, 0x3e);
    word(&mut bytes, 1);
    xword(&mut bytes, BASE);
    xword(&mut bytes, HEADER_SIZE);
    xword(&mut bytes, 0);
    word(&mut bytes, 0);
    half(&mut bytes, HEADER_SIZE as u16);
    half(&mut bytes, PROGRAM_HEADER_SIZE as u16);
    half(&mut bytes, PROGRAM_HEADER_COUNT as u16);
    half(&mut bytes, 64);
    half(&mut bytes, 0);
    half(&mut bytes, 0);

    // PT_LOAD
    word(&mut bytes, 1);
    word(&mut bytes, 4);
    xword(&mut bytes, 0);
    xword(&mut bytes, BASE);
    xword(&mut bytes, BASE);
    xword(&mut bytes, total_size);
    xword(&mut bytes, total_size);
    xword(&mut bytes, 0x1000);

    // PT_DYNAMIC
    word(&mut bytes, 2);
    word(&mut bytes, 4);
    xword(&mut bytes, DYNAMIC_OFFSET);
    xword(&mut bytes, BASE + DYNAMIC_OFFSET);
    xword(&mut bytes, BASE + DYNAMIC_OFFSET);
    xword(&mut bytes, dynamic_size);
    xword(&mut bytes, dynamic_size);
    xword(&mut bytes, 8);

    // Mandatory shared-object dynamic metadata, using DT_SYMTABSZ instead of DT_HASH.
    dynamic_entry(&mut bytes, 5, 0x401000); // DT_STRTAB
    dynamic_entry(&mut bytes, 10, 1); // DT_STRSZ
    dynamic_entry(&mut bytes, 6, 0x402000); // DT_SYMTAB
    dynamic_entry(&mut bytes, 11, 24); // DT_SYMENT
    dynamic_entry(&mut bytes, 39, 24); // DT_SYMTABSZ

    for &(tag, payload) in extra {
        dynamic_entry(&mut bytes, tag, payload);
    }

    dynamic_entry(&mut bytes, 0, 0); // DT_NULL

    assert_eq!(bytes.len(), total_size as usize);
    bytes
}

fn dynamic_entry_offset(index: usize) -> usize {
    64 + 2 * 56 + index * 16
}

#[test]
fn accepts_minimal_shared_object_dynamic_array() {
    let bytes = fixture(3, &[]);
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(object.validate_dynamic_array(1), Ok(()));
}

#[test]
fn accepts_executable_with_rel_relocations() {
    let bytes = fixture(2, &[(17, 0x403000), (18, 16), (19, 16)]);
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(object.validate_dynamic_array(1), Ok(()));
}

#[test]
fn rejects_dynamic_array_without_null_terminator() {
    let mut bytes = fixture(3, &[]);
    let null = dynamic_entry_offset(5);
    bytes[null..null + 8].copy_from_slice(&30i64.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_dynamic_array(1),
        Err(ValidationError::MissingNullTerminator),
    );
}

#[test]
fn rejects_reserved_dynamic_tag() {
    let bytes = fixture(3, &[(38, 0)]);
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_dynamic_array(1),
        Err(ValidationError::ReservedTag { index: 5, raw: 38 }),
    );
}

#[test]
fn rejects_missing_string_table() {
    let mut bytes = fixture(3, &[]);
    let entry = dynamic_entry_offset(0);
    bytes[entry..entry + 8].copy_from_slice(&39i64.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_dynamic_array(1),
        Err(ValidationError::MissingStringTable),
    );
}

#[test]
fn rejects_missing_symbol_table() {
    let mut bytes = fixture(3, &[]);
    let entry = dynamic_entry_offset(2);
    bytes[entry..entry + 8].copy_from_slice(&39i64.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_dynamic_array(1),
        Err(ValidationError::MissingSymbolTable),
    );
}

#[test]
fn rejects_missing_string_table_size() {
    let mut bytes = fixture(3, &[]);
    let entry = dynamic_entry_offset(1);
    bytes[entry..entry + 8].copy_from_slice(&39i64.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_dynamic_array(1),
        Err(ValidationError::MissingStringTableSize),
    );
}

#[test]
fn rejects_missing_symbol_entry_size() {
    let mut bytes = fixture(3, &[]);
    let entry = dynamic_entry_offset(3);
    bytes[entry..entry + 8].copy_from_slice(&39i64.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_dynamic_array(1),
        Err(ValidationError::MissingSymbolEntrySize),
    );
}

#[test]
fn rejects_missing_hash_and_symbol_table_size() {
    let mut bytes = fixture(3, &[]);
    let entry = dynamic_entry_offset(4);
    bytes[entry..entry + 8].copy_from_slice(&30i64.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_dynamic_array(1),
        Err(ValidationError::MissingHashOrSymbolTableSize),
    );
}

#[test]
fn rejects_wrong_symbol_entry_size() {
    let mut bytes = fixture(3, &[]);
    let payload = dynamic_entry_offset(3) + 8;
    bytes[payload..payload + 8].copy_from_slice(&16u64.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_dynamic_array(1),
        Err(ValidationError::SymbolEntrySizeMismatch {
            expected: 24,
            actual: 16,
        }),
    );
}

#[test]
fn rejects_symbol_table_size_not_entry_multiple() {
    let mut bytes = fixture(3, &[]);
    let payload = dynamic_entry_offset(4) + 8;
    bytes[payload..payload + 8].copy_from_slice(&25u64.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_dynamic_array(1),
        Err(ValidationError::SymbolTableSizeNotEntryMultiple),
    );
}

#[test]
fn rejects_executable_without_rel_or_rela_table() {
    let bytes = fixture(2, &[]);
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_dynamic_array(1),
        Err(ValidationError::ExecutableMissingRelocationTable),
    );
}

#[test]
fn rejects_rel_without_size() {
    let bytes = fixture(3, &[(17, 0x403000)]);
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_dynamic_array(1),
        Err(ValidationError::RelocationMissingSize),
    );
}

#[test]
fn rejects_rel_without_entry_size() {
    let bytes = fixture(3, &[(17, 0x403000), (18, 16)]);
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_dynamic_array(1),
        Err(ValidationError::RelocationMissingEntrySize),
    );
}

#[test]
fn rejects_wrong_rel_entry_size() {
    let bytes = fixture(3, &[(17, 0x403000), (18, 16), (19, 8)]);
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_dynamic_array(1),
        Err(ValidationError::RelocationEntrySizeMismatch {
            expected: 16,
            actual: 8,
        }),
    );
}

#[test]
fn rejects_rela_without_size() {
    let bytes = fixture(3, &[(7, 0x403000)]);
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_dynamic_array(1),
        Err(ValidationError::RelocationWithAddendMissingSize),
    );
}

#[test]
fn rejects_rela_without_entry_size() {
    let bytes = fixture(3, &[(7, 0x403000), (8, 24)]);
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_dynamic_array(1),
        Err(ValidationError::RelocationWithAddendMissingEntrySize),
    );
}

#[test]
fn rejects_wrong_rela_entry_size() {
    let bytes = fixture(3, &[(7, 0x403000), (8, 24), (9, 16)]);
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_dynamic_array(1),
        Err(ValidationError::RelocationWithAddendEntrySizeMismatch {
            expected: 24,
            actual: 16,
        }),
    );
}

#[test]
fn rejects_jump_relocation_without_size() {
    let bytes = fixture(3, &[(23, 0x404000)]);
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_dynamic_array(1),
        Err(ValidationError::JumpRelocationMissingSize),
    );
}

#[test]
fn rejects_jump_relocation_without_format() {
    let bytes = fixture(3, &[(23, 0x404000), (2, 16)]);
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_dynamic_array(1),
        Err(ValidationError::JumpRelocationMissingFormat),
    );
}

#[test]
fn rejects_invalid_jump_relocation_format() {
    let bytes = fixture(3, &[(23, 0x404000), (2, 16), (20, 35)]);
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_dynamic_array(1),
        Err(ValidationError::JumpRelocationInvalidFormat { raw: 35 }),
    );
}

#[test]
fn rejects_reserved_dynamic_flags() {
    let bytes = fixture(3, &[(30, 0x20)]);
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_dynamic_array(1),
        Err(ValidationError::ReservedFlags { bits: 0x20 }),
    );
}


#[test]
fn rejects_jump_relocation_size_not_entry_multiple() {
    let bytes = fixture(3, &[(23, 0x404000), (2, 17), (20, 17)]);
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_dynamic_array(1),
        Err(ValidationError::JumpRelocationSizeNotEntryMultiple),
    );
}
