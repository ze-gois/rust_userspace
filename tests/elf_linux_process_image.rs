use userspace::{
    file::format::elf::ObjectFile,
    target::operating_system::process_image::{self, Error},
};

const HEADER_SIZE: u64 = 64;
const PROGRAM_HEADER_SIZE: u64 = 56;
const PROGRAM_HEADER_COUNT: u64 = 2;
const DYNAMIC_OFFSET: u64 =
    HEADER_SIZE + PROGRAM_HEADER_SIZE * PROGRAM_HEADER_COUNT;
const DYNAMIC_SIZE: u64 = 9 * 16;
const STRING_OFFSET: u64 = DYNAMIC_OFFSET + DYNAMIC_SIZE;
const STRING_SIZE: u64 = 1;
const SYMBOL_OFFSET: u64 = STRING_OFFSET + STRING_SIZE;
const SYMBOL_SIZE: u64 = 24;
const RELOCATION_OFFSET: u64 = SYMBOL_OFFSET + SYMBOL_SIZE;
const RELOCATION_SIZE: u64 = 24;
const TARGET_OFFSET: u64 = 0x180;
const TOTAL_SIZE: u64 = TARGET_OFFSET + 8;
const PAGE_SIZE: usize = 0x1000;

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

fn fixture(machine: u16, relocation_type: u32) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(TOTAL_SIZE as usize);

    bytes.extend_from_slice(&[
        0x7f, b'E', b'L', b'F',
        2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]);
    half(&mut bytes, 3); // ET_DYN
    half(&mut bytes, machine);
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

    // PT_LOAD: one writable page containing the entire fixture.
    word(&mut bytes, 1);
    word(&mut bytes, 6);
    xword(&mut bytes, 0);
    xword(&mut bytes, 0);
    xword(&mut bytes, 0);
    xword(&mut bytes, TOTAL_SIZE);
    xword(&mut bytes, PAGE_SIZE as u64);
    xword(&mut bytes, PAGE_SIZE as u64);

    // PT_DYNAMIC.
    word(&mut bytes, 2);
    word(&mut bytes, 6);
    xword(&mut bytes, DYNAMIC_OFFSET);
    xword(&mut bytes, DYNAMIC_OFFSET);
    xword(&mut bytes, DYNAMIC_OFFSET);
    xword(&mut bytes, DYNAMIC_SIZE);
    xword(&mut bytes, DYNAMIC_SIZE);
    xword(&mut bytes, 8);

    dynamic_entry(&mut bytes, 5, STRING_OFFSET); // DT_STRTAB
    dynamic_entry(&mut bytes, 10, STRING_SIZE); // DT_STRSZ
    dynamic_entry(&mut bytes, 6, SYMBOL_OFFSET); // DT_SYMTAB
    dynamic_entry(&mut bytes, 11, SYMBOL_SIZE); // DT_SYMENT
    dynamic_entry(&mut bytes, 39, SYMBOL_SIZE); // DT_SYMTABSZ
    dynamic_entry(&mut bytes, 7, RELOCATION_OFFSET); // DT_RELA
    dynamic_entry(&mut bytes, 8, RELOCATION_SIZE); // DT_RELASZ
    dynamic_entry(&mut bytes, 9, RELOCATION_SIZE); // DT_RELAENT
    dynamic_entry(&mut bytes, 0, 0); // DT_NULL

    bytes.push(0); // dynamic string table

    // STN_UNDEF.
    bytes.extend_from_slice(&[0u8; SYMBOL_SIZE as usize]);

    // One Elf64_Rela.
    xword(&mut bytes, TARGET_OFFSET);
    xword(&mut bytes, u64::from(relocation_type));
    sxword(&mut bytes, 0x1234);

    bytes.resize(TARGET_OFFSET as usize, 0);
    xword(&mut bytes, 0);

    assert_eq!(bytes.len(), TOTAL_SIZE as usize);
    bytes
}

#[test]
fn maps_and_applies_x86_64_relative_relocation_before_final_protection() {
    let bytes = fixture(62, 8);
    let object = ObjectFile::parse(&bytes).expect("fixture must parse");

    let mapping = process_image::map(&object, PAGE_SIZE)
        .expect("x86_64 ELF process image must map");

    let relocated = unsafe {
        core::ptr::read_unaligned(
            mapping.address().add(TARGET_OFFSET as usize) as *const u64,
        )
    };

    assert_eq!(
        relocated,
        mapping.base_address().value() + 0x1234,
    );
    assert!(mapping.unmap());
}

#[test]
fn rejects_unimplemented_x86_64_relocation_in_real_mapping() {
    let bytes = fixture(62, 6);
    let object = ObjectFile::parse(&bytes).expect("fixture must parse");

    assert!(matches!(
        process_image::map(&object, PAGE_SIZE),
        Err(Error::UnsupportedProcessorRelocation { raw: 6 }),
    ));
}

#[test]
fn rejects_foreign_machine_before_exposing_process_mapping() {
    let bytes = fixture(183, 8);
    let object = ObjectFile::parse(&bytes).expect("fixture must parse");

    assert!(matches!(
        process_image::map(&object, PAGE_SIZE),
        Err(Error::UnsupportedMachine { machine: 183 }),
    ));
}
