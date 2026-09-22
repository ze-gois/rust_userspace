use userspace::file::format::elf::{
    identification::{Class, Data},
    ObjectFile,
};

#[derive(Clone, Copy)]
enum ByteOrder {
    Little,
    Big,
}

fn half(bytes: &mut Vec<u8>, value: u16, order: ByteOrder) {
    bytes.extend_from_slice(&match order {
        ByteOrder::Little => value.to_le_bytes(),
        ByteOrder::Big => value.to_be_bytes(),
    });
}

fn word(bytes: &mut Vec<u8>, value: u32, order: ByteOrder) {
    bytes.extend_from_slice(&match order {
        ByteOrder::Little => value.to_le_bytes(),
        ByteOrder::Big => value.to_be_bytes(),
    });
}

fn xword(bytes: &mut Vec<u8>, value: u64, order: ByteOrder) {
    bytes.extend_from_slice(&match order {
        ByteOrder::Little => value.to_le_bytes(),
        ByteOrder::Big => value.to_be_bytes(),
    });
}

fn identification(class: Class, data: Data) -> [u8; 16] {
    let mut bytes = [0u8; 16];
    bytes[..4].copy_from_slice(&[0x7f, b'E', b'L', b'F']);
    bytes[4] = class.raw();
    bytes[5] = data.raw();
    bytes[6] = 1;
    bytes
}

fn elf32(order: ByteOrder, data: Data) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(52);
    bytes.extend_from_slice(&identification(Class::Class32, data));
    half(&mut bytes, 2, order);
    half(&mut bytes, 0x1234, order);
    word(&mut bytes, 0x0102_0304, order);
    word(&mut bytes, 0x1234_5678, order);
    word(&mut bytes, 0, order);
    word(&mut bytes, 0, order);
    word(&mut bytes, 0x1122_3344, order);
    half(&mut bytes, 52, order);
    half(&mut bytes, 32, order);
    half(&mut bytes, 0, order);
    half(&mut bytes, 40, order);
    half(&mut bytes, 0, order);
    half(&mut bytes, 0, order);
    assert_eq!(bytes.len(), 52);
    bytes
}

fn elf64(order: ByteOrder, data: Data) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(64);
    bytes.extend_from_slice(&identification(Class::Class64, data));
    half(&mut bytes, 2, order);
    half(&mut bytes, 0x1234, order);
    word(&mut bytes, 0x0102_0304, order);
    xword(&mut bytes, 0x0102_0304_0506_0708, order);
    xword(&mut bytes, 0, order);
    xword(&mut bytes, 0, order);
    word(&mut bytes, 0x1122_3344, order);
    half(&mut bytes, 64, order);
    half(&mut bytes, 56, order);
    half(&mut bytes, 0, order);
    half(&mut bytes, 64, order);
    half(&mut bytes, 0, order);
    half(&mut bytes, 0, order);
    assert_eq!(bytes.len(), 64);
    bytes
}

fn assert_elf32(bytes: &[u8], data: Data) {
    let object = ObjectFile::parse(bytes).expect("ELF32 fixture must parse");
    assert_eq!(object.header.identification.class, Class::Class32);
    assert_eq!(object.header.identification.data, data);
    assert_eq!(object.header.r#type.raw(), 2);
    assert_eq!(object.header.machine.raw(), 0x1234);
    assert_eq!(object.header.version.raw(), 0x0102_0304);
    assert_eq!(object.header.entry, 0x1234_5678);
    assert_eq!(object.header.flags, 0x1122_3344);
    assert_eq!(object.header.header_size, 52);
    assert_eq!(object.header.program_header_entry_size, 32);
    assert_eq!(object.header.section_header_entry_size, 40);
    assert!(object.program_headers.is_empty());
    assert!(object.section_headers.is_empty());
}

fn assert_elf64(bytes: &[u8], data: Data) {
    let object = ObjectFile::parse(bytes).expect("ELF64 fixture must parse");
    assert_eq!(object.header.identification.class, Class::Class64);
    assert_eq!(object.header.identification.data, data);
    assert_eq!(object.header.r#type.raw(), 2);
    assert_eq!(object.header.machine.raw(), 0x1234);
    assert_eq!(object.header.version.raw(), 0x0102_0304);
    assert_eq!(object.header.entry, 0x0102_0304_0506_0708);
    assert_eq!(object.header.flags, 0x1122_3344);
    assert_eq!(object.header.header_size, 64);
    assert_eq!(object.header.program_header_entry_size, 56);
    assert_eq!(object.header.section_header_entry_size, 64);
    assert!(object.program_headers.is_empty());
    assert!(object.section_headers.is_empty());
}

#[test]
fn parses_elf32_little_endian() {
    assert_elf32(
        &elf32(ByteOrder::Little, Data::LeastSignificantByteFirst),
        Data::LeastSignificantByteFirst,
    );
}

#[test]
fn parses_elf32_big_endian() {
    assert_elf32(
        &elf32(ByteOrder::Big, Data::MostSignificantByteFirst),
        Data::MostSignificantByteFirst,
    );
}

#[test]
fn parses_elf64_little_endian() {
    assert_elf64(
        &elf64(ByteOrder::Little, Data::LeastSignificantByteFirst),
        Data::LeastSignificantByteFirst,
    );
}

#[test]
fn parses_elf64_big_endian() {
    assert_elf64(
        &elf64(ByteOrder::Big, Data::MostSignificantByteFirst),
        Data::MostSignificantByteFirst,
    );
}
