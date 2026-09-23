use userspace::file::format::elf::{
    dynamic::{validation::ValidationError, Tag},
    dynamic_symbol_table::ValidationError as DynamicSymbolTableValidationError,
    hash::ValidationError as HashValidationError,
    string_table::ValidationError as StringTableValidationError,
    ObjectFile,
};

const BASE: u64 = 0x400000;
const HEADER_SIZE: u64 = 64;
const PROGRAM_HEADER_SIZE: u64 = 56;
const PROGRAM_HEADER_COUNT: u64 = 2;
const DYNAMIC_OFFSET: u64 = HEADER_SIZE + PROGRAM_HEADER_SIZE * PROGRAM_HEADER_COUNT;
const DYNAMIC_SIZE: u64 = 7 * 16;
const HASH_OFFSET: u64 = DYNAMIC_OFFSET + DYNAMIC_SIZE;
const HASH_SIZE: u64 = 20;
const STRING_OFFSET: u64 = HASH_OFFSET + HASH_SIZE;
const STRING_SIZE: u64 = 13;
const SYMBOL_OFFSET: u64 = STRING_OFFSET + STRING_SIZE;
const SYMBOL_SIZE: u64 = 48;
const TOTAL_SIZE: u64 = SYMBOL_OFFSET + SYMBOL_SIZE;

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
    xword(&mut bytes, TOTAL_SIZE);
    xword(&mut bytes, TOTAL_SIZE);
    xword(&mut bytes, 0x1000);

    // PT_DYNAMIC
    word(&mut bytes, 2);
    word(&mut bytes, 4);
    xword(&mut bytes, DYNAMIC_OFFSET);
    xword(&mut bytes, BASE + DYNAMIC_OFFSET);
    xword(&mut bytes, BASE + DYNAMIC_OFFSET);
    xword(&mut bytes, DYNAMIC_SIZE);
    xword(&mut bytes, DYNAMIC_SIZE);
    xword(&mut bytes, 8);

    dynamic_entry(&mut bytes, 4, BASE + HASH_OFFSET); // DT_HASH
    dynamic_entry(&mut bytes, 5, BASE + STRING_OFFSET); // DT_STRTAB
    dynamic_entry(&mut bytes, 6, BASE + SYMBOL_OFFSET); // DT_SYMTAB
    dynamic_entry(&mut bytes, 10, STRING_SIZE); // DT_STRSZ
    dynamic_entry(&mut bytes, 11, 24); // DT_SYMENT
    dynamic_entry(&mut bytes, 1, 5); // DT_NEEDED -> "libx.so"
    dynamic_entry(&mut bytes, 0, 0); // DT_NULL

    // System V hash: nbucket=1, nchain=2, bucket[0]=1, chain[0]=0, chain[1]=0.
    word(&mut bytes, 1);
    word(&mut bytes, 2);
    word(&mut bytes, 1);
    word(&mut bytes, 0);
    word(&mut bytes, 0);

    bytes.extend_from_slice(b"\0foo\0libx.so\0");

    // STN_UNDEF.
    bytes.extend_from_slice(&[0u8; 24]);

    // Global object symbol named "foo".
    word(&mut bytes, 1);
    bytes.push(0x11);
    bytes.push(0);
    half(&mut bytes, 1);
    xword(&mut bytes, 0x401000);
    xword(&mut bytes, 4);

    assert_eq!(bytes.len(), TOTAL_SIZE as usize);
    bytes
}

fn dynamic_entry_offset(index: usize) -> usize {
    DYNAMIC_OFFSET as usize + index * 16
}

fn second_symbol_offset() -> usize {
    SYMBOL_OFFSET as usize + 24
}

#[test]
fn accepts_dynamic_string_symbol_and_hash_tables() {
    let bytes = fixture();
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(object.validate_dynamic_linking_tables(1), Ok(()));
}

#[test]
fn rejects_dynamic_string_table_without_initial_null() {
    let mut bytes = fixture();
    bytes[STRING_OFFSET as usize] = b'x';

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_dynamic_linking_tables(1),
        Err(ValidationError::DynamicStringTableInvalid(
            StringTableValidationError::MissingInitialNull,
        )),
    );
}

#[test]
fn rejects_dynamic_string_table_without_final_null() {
    let mut bytes = fixture();
    bytes[(STRING_OFFSET + STRING_SIZE - 1) as usize] = b'x';

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_dynamic_linking_tables(1),
        Err(ValidationError::DynamicStringTableInvalid(
            StringTableValidationError::MissingFinalNull,
        )),
    );
}

#[test]
fn rejects_invalid_dynamic_string_offset() {
    let mut bytes = fixture();
    let payload = dynamic_entry_offset(5) + 8;
    bytes[payload..payload + 8].copy_from_slice(&99u64.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_dynamic_linking_tables(1),
        Err(ValidationError::InvalidStringOffset {
            index: 5,
            tag: Tag::Needed,
            offset: 99,
        }),
    );
}

#[test]
fn rejects_undefined_upper_bits_in_dynamic_symbol_other() {
    let mut bytes = fixture();
    bytes[second_symbol_offset() + 5] = 0x80;

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_dynamic_linking_tables(1),
        Err(ValidationError::DynamicSymbolTableInvalid(
            DynamicSymbolTableValidationError::UndefinedOtherBits {
                index: 1,
                bits: 0x80,
            },
        )),
    );
}

#[test]
fn rejects_invalid_dynamic_symbol_name_index() {
    let mut bytes = fixture();
    let offset = second_symbol_offset();
    bytes[offset..offset + 4].copy_from_slice(&99u32.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_dynamic_linking_tables(1),
        Err(ValidationError::DynamicSymbolTableInvalid(
            DynamicSymbolTableValidationError::InvalidNameIndex {
                index: 1,
                name_index: 99,
            },
        )),
    );
}

#[test]
fn rejects_reserved_dynamic_symbol_section_index() {
    let mut bytes = fixture();
    let offset = second_symbol_offset() + 6;
    bytes[offset..offset + 2].copy_from_slice(&0xff40u16.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_dynamic_linking_tables(1),
        Err(ValidationError::DynamicSymbolTableInvalid(
            DynamicSymbolTableValidationError::ReservedSectionIndex {
                index: 1,
                raw: 0xff40,
            },
        )),
    );
}

#[test]
fn rejects_hash_bucket_outside_symbol_and_chain_tables() {
    let mut bytes = fixture();
    let bucket = HASH_OFFSET as usize + 8;
    bytes[bucket..bucket + 4].copy_from_slice(&2u32.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_dynamic_linking_tables(1),
        Err(ValidationError::DynamicHashTableInvalid(
            HashValidationError::BucketIndexOutOfBounds {
                bucket_index: 0,
                symbol_index: 2,
            },
        )),
    );
}

#[test]
fn rejects_hash_chain_outside_symbol_and_chain_tables() {
    let mut bytes = fixture();
    let chain_one = HASH_OFFSET as usize + 16;
    bytes[chain_one..chain_one + 4].copy_from_slice(&2u32.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_dynamic_linking_tables(1),
        Err(ValidationError::DynamicHashTableInvalid(
            HashValidationError::ChainIndexOutOfBounds {
                chain_index: 1,
                symbol_index: 2,
            },
        )),
    );
}


#[test]
fn rejects_dynamic_extended_section_index_without_companion_table() {
    let mut bytes = fixture();
    let offset = second_symbol_offset() + 6;
    bytes[offset..offset + 2].copy_from_slice(&0xffffu16.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_dynamic_linking_tables(1),
        Err(ValidationError::DynamicSymbolTableUnavailable),
    );
}
