use userspace::file::format::elf::symbol::Visibility;

#[test]
fn decodes_all_generic_symbol_visibility_values() {
    assert_eq!(Visibility::from_raw(0), Visibility::Default);
    assert_eq!(Visibility::from_raw(1), Visibility::Internal);
    assert_eq!(Visibility::from_raw(2), Visibility::Hidden);
    assert_eq!(Visibility::from_raw(3), Visibility::Protected);
    assert_eq!(Visibility::from_raw(4), Visibility::Exported);
    assert_eq!(Visibility::from_raw(5), Visibility::Singleton);
    assert_eq!(Visibility::from_raw(6), Visibility::Eliminate);
    assert_eq!(Visibility::from_raw(7), Visibility::Reserved(7));
}

#[test]
fn ignores_undefined_upper_st_other_bits() {
    assert_eq!(Visibility::from_raw(0b1111_1100), Visibility::Exported);
    assert_eq!(Visibility::from_raw(0b1111_1101), Visibility::Singleton);
    assert_eq!(Visibility::from_raw(0b1111_1110), Visibility::Eliminate);
    assert_eq!(Visibility::from_raw(0b1111_1111), Visibility::Reserved(7));
}

#[test]
fn preserves_visibility_raw_values() {
    for raw in 0u8..=7 {
        assert_eq!(Visibility::from_raw(raw).raw(), raw);
    }
}
