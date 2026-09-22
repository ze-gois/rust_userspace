use userspace::file::format::elf::dynamic::Flags;

#[test]
fn preserves_dynamic_state_flag_bits() {
    let flags = Flags::from_raw(
        Flags::ORIGIN
            | Flags::SYMBOLIC
            | Flags::TEXT_RELOCATION
            | Flags::BIND_NOW
            | Flags::STATIC_THREAD_LOCAL_STORAGE,
    );

    assert_eq!(flags.raw(), 0x1f);
    assert!(flags.contains(Flags::ORIGIN));
    assert!(flags.contains(Flags::SYMBOLIC));
    assert!(flags.contains(Flags::TEXT_RELOCATION));
    assert!(flags.contains(Flags::BIND_NOW));
    assert!(flags.contains(Flags::STATIC_THREAD_LOCAL_STORAGE));
}

#[test]
fn preserves_unknown_dynamic_state_flag_bits() {
    let flags = Flags::from_raw(0x8000_0000_0000_0000 | Flags::BIND_NOW);

    assert_eq!(flags.raw(), 0x8000_0000_0000_0008);
    assert!(flags.contains(Flags::BIND_NOW));
}
