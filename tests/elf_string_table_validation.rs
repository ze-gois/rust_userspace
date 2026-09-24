use userspace::file::format::elf::string_table::{
    StringTable,
    ValidationError,
};

#[test]
fn accepts_empty_string_table() {
    assert_eq!(StringTable::new(&[]).validate(), Ok(()));
}

#[test]
fn accepts_null_bounded_string_table() {
    assert_eq!(StringTable::new(b"\0name\0").validate(), Ok(()));
}

#[test]
fn rejects_string_table_without_initial_null() {
    assert_eq!(
        StringTable::new(b"name\0").validate(),
        Err(ValidationError::MissingInitialNull),
    );
}

#[test]
fn rejects_string_table_without_final_null() {
    assert_eq!(
        StringTable::new(b"\0name").validate(),
        Err(ValidationError::MissingFinalNull),
    );
}
