use userspace::file::format::elf::{
    dynamic::validation::ValidationError,
    dynamic_relocation_table::{Addend, Purpose},
    ObjectFile,
};

const BASE: u64 = 0x400000;
const HEADER_SIZE: u64 = 64;
const PROGRAM_HEADER_SIZE: u64 = 56;
const PROGRAM_HEADER_COUNT: u64 = 2;
const DYNAMIC_OFFSET: u64 = HEADER_SIZE + PROGRAM_HEADER_SIZE * PROGRAM_HEADER_COUNT;
const DYNAMIC_SIZE: u64 = 9 * 16;
const STRING_OFFSET: u64 = DYNAMIC_OFFSET + DYNAMIC_SIZE;
const STRING_SIZE: u64 = 5;
const SYMBOL_OFFSET: u64 = STRING_OFFSET + STRING_SIZE;
const SYMBOL_SIZE: u64 = 48;
const RELOCATION_OFFSET: u64 = SYMBOL_OFFSET + SYMBOL_SIZE;
const RELOCATION_SIZE: u64 = 16;
const TOTAL_SIZE: u64 = RELOCATION_OFFSET + RELOCATION_SIZE;

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

fn fixture(symbol_index: u32, relocation_address: u64) -> Vec<u8> {
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

    dynamic_entry(&mut bytes, 5, BASE + STRING_OFFSET); // DT_STRTAB
    dynamic_entry(&mut bytes, 10, STRING_SIZE); // DT_STRSZ
    dynamic_entry(&mut bytes, 6, BASE + SYMBOL_OFFSET); // DT_SYMTAB
    dynamic_entry(&mut bytes, 11, 24); // DT_SYMENT
    dynamic_entry(&mut bytes, 39, SYMBOL_SIZE); // DT_SYMTABSZ
    dynamic_entry(&mut bytes, 23, relocation_address); // DT_JMPREL
    dynamic_entry(&mut bytes, 2, RELOCATION_SIZE); // DT_PLTRELSZ
    dynamic_entry(&mut bytes, 20, 17); // DT_PLTREL = DT_REL
    dynamic_entry(&mut bytes, 0, 0); // DT_NULL

    bytes.extend_from_slice(b"\0foo\0");

    // STN_UNDEF.
    bytes.extend_from_slice(&[0u8; 24]);

    // Global object symbol named "foo".
    word(&mut bytes, 1);
    bytes.push(0x11);
    bytes.push(0);
    half(&mut bytes, 1);
    xword(&mut bytes, 0x401000);
    xword(&mut bytes, 4);

    // One Elf64_Rel. No DT_RELENT is intentionally present.
    xword(&mut bytes, 0x401000);
    xword(&mut bytes, (u64::from(symbol_index) << 32) | 1);

    assert_eq!(bytes.len(), TOTAL_SIZE as usize);
    bytes
}

#[test]
fn resolves_and_validates_plt_only_rel_without_relent() {
    let bytes = fixture(1, BASE + RELOCATION_OFFSET);
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    let tables = object
        .dynamic_relocation_tables_from_program_header(1)
        .expect("PLT-only relocation table must resolve");

    assert_eq!(tables.len(), 1);
    assert_eq!(tables[0].addend, Addend::Implicit);
    assert_eq!(tables[0].purpose, Purpose::ProcedureLinkageTable);
    assert_eq!(tables[0].len(), 1);
    assert_eq!(object.validate_dynamic_relocation_tables(1), Ok(()));
}

#[test]
fn rejects_dynamic_relocation_symbol_index_outside_symbol_table() {
    let bytes = fixture(2, BASE + RELOCATION_OFFSET);
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_dynamic_relocation_tables(1),
        Err(ValidationError::DynamicRelocationSymbolIndexOutOfBounds {
            table_index: 0,
            entry_index: 0,
            symbol_index: 2,
        }),
    );
}

#[test]
fn rejects_unmapped_dynamic_relocation_table() {
    let bytes = fixture(1, 0x500000);
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_dynamic_relocation_tables(1),
        Err(ValidationError::DynamicRelocationTablesUnavailable),
    );
}
