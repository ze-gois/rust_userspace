use userspace::file::format::elf::{
    header::ValidationError,
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

fn fixture() -> Vec<u8> {
    let mut bytes = Vec::new();

    bytes.extend_from_slice(&[
        0x7f, b'E', b'L', b'F',
        2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]);
    half(&mut bytes, 1);
    half(&mut bytes, 0x3e);
    word(&mut bytes, 1);
    xword(&mut bytes, 0);
    xword(&mut bytes, 0);
    xword(&mut bytes, 0);
    word(&mut bytes, 0);
    half(&mut bytes, 64);
    half(&mut bytes, 56);
    half(&mut bytes, 0);
    half(&mut bytes, 64);
    half(&mut bytes, 0);
    half(&mut bytes, 0);

    bytes
}

#[test]
fn accepts_current_header_without_tables() {
    let bytes = fixture();
    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(object.validate_header(), Ok(()));
}

#[test]
fn rejects_non_current_identification_version() {
    let mut bytes = fixture();
    bytes[6] = 0;

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_header(),
        Err(ValidationError::IdentificationVersionNotCurrent { version: 0 }),
    );
}

#[test]
fn rejects_nonzero_identification_padding() {
    let mut bytes = fixture();
    bytes[9] = 7;

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_header(),
        Err(ValidationError::IdentificationPaddingNotZero {
            index: 9,
            value: 7,
        }),
    );
}

#[test]
fn rejects_non_current_object_version() {
    let mut bytes = fixture();
    bytes[20..24].copy_from_slice(&0u32.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_header(),
        Err(ValidationError::ObjectVersionNotCurrent { version: 0 }),
    );
}

#[test]
fn rejects_reserved_object_type() {
    let mut bytes = fixture();
    bytes[16..18].copy_from_slice(&5u16.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_header(),
        Err(ValidationError::ReservedObjectType { raw: 5 }),
    );
}

#[test]
fn rejects_header_size_smaller_than_current_representation() {
    let mut bytes = fixture();
    bytes[52..54].copy_from_slice(&63u16.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_header(),
        Err(ValidationError::HeaderSizeTooSmall {
            size: 63,
            minimum: 64,
        }),
    );
}

#[test]
fn rejects_header_size_beyond_file_extent() {
    let mut bytes = fixture();
    bytes[52..54].copy_from_slice(&65u16.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_header(),
        Err(ValidationError::HeaderSizeExceedsFile { size: 65 }),
    );
}

#[test]
fn rejects_program_header_offset_without_table() {
    let mut bytes = fixture();
    bytes[32..40].copy_from_slice(&64u64.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_header(),
        Err(ValidationError::ProgramHeaderOffsetWithoutTable { offset: 64 }),
    );
}

#[test]
fn rejects_program_header_table_without_offset() {
    let mut bytes = fixture();
    bytes[56..58].copy_from_slice(&1u16.to_le_bytes());

    let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");

    assert_eq!(
        object.validate_header(),
        Err(ValidationError::ProgramHeaderTableWithoutOffset { count: 1 }),
    );
}


#[test]
fn accepts_currently_assigned_machine_values() {
    for machine in [0u16, 62u16, 243u16, 269u16] {
        let mut bytes = fixture();
        bytes[18..20].copy_from_slice(&machine.to_le_bytes());

        let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");
        assert_eq!(object.validate_header(), Ok(()));
    }
}

#[test]
fn rejects_reserved_machine_values() {
    for machine in [11u16, 16u16, 24u16, 121u16, 145u16, 182u16, 184u16, 225u16, 270u16] {
        let mut bytes = fixture();
        bytes[18..20].copy_from_slice(&machine.to_le_bytes());

        let object = ObjectFile::parse(&bytes).expect("ELF fixture must parse");
        assert_eq!(
            object.validate_header(),
            Err(ValidationError::ReservedMachine { raw: machine }),
        );
    }
}
