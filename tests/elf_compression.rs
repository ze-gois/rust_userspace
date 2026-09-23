use userspace::file::format::elf::{
    compression::{Type as CompressionType, ValidationError},
    ObjectFile,
};

fn half(bytes: &mut Vec<u8>, value: u16) { bytes.extend_from_slice(&value.to_le_bytes()); }
fn word(bytes: &mut Vec<u8>, value: u32) { bytes.extend_from_slice(&value.to_le_bytes()); }
fn xword(bytes: &mut Vec<u8>, value: u64) { bytes.extend_from_slice(&value.to_le_bytes()); }

fn fixture(object_type: u16, section_type: u32, flags: u64) -> Vec<u8> {
    const HEADER_SIZE: u64 = 64;
    const COMPRESSION_HEADER_SIZE: u64 = 24;
    const PAYLOAD_SIZE: u64 = 3;
    const SECTION_OFFSET: u64 = HEADER_SIZE;
    const SECTION_SIZE: u64 = COMPRESSION_HEADER_SIZE + PAYLOAD_SIZE;
    const SECTION_HEADER_OFFSET: u64 = SECTION_OFFSET + SECTION_SIZE;

    let mut bytes = Vec::new();

    bytes.extend_from_slice(&[
        0x7f, b'E', b'L', b'F',
        2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]);
    half(&mut bytes, object_type);
    half(&mut bytes, 0x3e);
    word(&mut bytes, 1);
    xword(&mut bytes, 0);
    xword(&mut bytes, 0);
    xword(&mut bytes, SECTION_HEADER_OFFSET);
    word(&mut bytes, 0);
    half(&mut bytes, HEADER_SIZE as u16);
    half(&mut bytes, 56);
    half(&mut bytes, 0);
    half(&mut bytes, 64);
    half(&mut bytes, 2);
    half(&mut bytes, 0);

    // Elf64_Chdr: ELFCOMPRESS_ZSTD, reserved=0, uncompressed size=16, alignment=8.
    word(&mut bytes, 2);
    word(&mut bytes, 0);
    xword(&mut bytes, 16);
    xword(&mut bytes, 8);
    bytes.extend_from_slice(&[0xaa, 0xbb, 0xcc]);

    // Section 0: SHT_NULL.
    bytes.extend_from_slice(&[0u8; 64]);

    // Section 1.
    word(&mut bytes, 0);
    word(&mut bytes, section_type);
    xword(&mut bytes, flags);
    xword(&mut bytes, 0);
    xword(&mut bytes, SECTION_OFFSET);
    xword(&mut bytes, if section_type == 8 { 0 } else { SECTION_SIZE });
    word(&mut bytes, 0);
    word(&mut bytes, 0);
    xword(&mut bytes, 1);
    xword(&mut bytes, 0);

    bytes
}

#[test]
fn recognizes_zstandard_compression_type() {
    assert_eq!(CompressionType::from_raw(2), CompressionType::Zstandard);
    assert_eq!(CompressionType::Zstandard.raw(), 2);
}

#[test]
fn resolves_non_allocated_compressed_section() {
    let bytes = fixture(1, 1, 0x800);
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    object
        .validate_compressed_section(1)
        .expect("non-allocable compressed section must be valid");

    let section = object
        .compressed_section(1)
        .expect("compressed section must resolve");

    assert_eq!(section.header.r#type, CompressionType::Zstandard);
    assert_eq!(section.header.uncompressed_size, 16);
    assert_eq!(section.header.alignment, 8);
    assert_eq!(section.data, &[0xaa, 0xbb, 0xcc]);
}

#[test]
fn rejects_allocated_compressed_section_in_executable() {
    let bytes = fixture(2, 1, 0x2 | 0x800);
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_compressed_section(1),
        Err(ValidationError::AllocatedCompressedSectionInExecutableOrSharedObject),
    );
    assert!(object.compressed_section(1).is_none());
}

#[test]
fn rejects_compressed_nobits_section() {
    let bytes = fixture(1, 8, 0x800);
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_compressed_section(1),
        Err(ValidationError::NoBitsCompressedSection),
    );
    assert!(object.compressed_section(1).is_none());
}


#[test]
fn allows_allocated_compressed_section_in_relocatable_object() {
    let bytes = fixture(1, 1, 0x2 | 0x800);
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(object.validate_compressed_section(1), Ok(()));
}

#[test]
fn allows_allocated_compressed_section_in_core_object() {
    let bytes = fixture(4, 1, 0x2 | 0x800);
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(object.validate_compressed_section(1), Ok(()));
}

#[test]
fn rejects_truncated_compression_header() {
    let mut bytes = fixture(1, 1, 0x800);

    // Section [1] begins after the ELF header and compression payload;
    // sh_size is 32 bytes into Elf64_Shdr. Keep fewer bytes than Elf64_Chdr.
    let section_header_offset = 64 + 27 + 64;
    bytes[section_header_offset + 32..section_header_offset + 40]
        .copy_from_slice(&8u64.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_compressed_section(1),
        Err(ValidationError::MissingCompressionHeader),
    );
}

#[test]
fn rejects_non_power_of_two_uncompressed_alignment() {
    let mut bytes = fixture(1, 1, 0x800);

    // Elf64_Chdr.ch_addralign.
    bytes[64 + 16..64 + 24].copy_from_slice(&3u64.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_compressed_section(1),
        Err(ValidationError::UncompressedAlignmentNotPowerOfTwo),
    );
}
