use userspace::file::format::elf::{
    base_address::BaseAddress,
    memory_image::{MemoryImageWriter, RegionWriter},
    processor_specific::x86_64::relocation::{
        R_X86_64_NONE, R_X86_64_RELATIVE, RelativeError, RelativeWrite, Type,
        relative_write,
    },
    relocation::{Relocation, Type as GenericType},
};

fn base(value: u64) -> BaseAddress {
    BaseAddress::calculate(value, 0, 0x1000).expect("base address must calculate")
}

#[test]
fn recognizes_normative_x86_64_relocation_numbers() {
    assert_eq!(R_X86_64_NONE, 0);
    assert_eq!(R_X86_64_RELATIVE, 8);

    assert_eq!(
        Type::from_generic(GenericType::from_raw(R_X86_64_NONE)),
        Type::None,
    );
    assert_eq!(
        Type::from_generic(GenericType::from_raw(R_X86_64_RELATIVE)),
        Type::Relative,
    );

    let other = GenericType::from_raw(41);
    assert_eq!(Type::from_generic(other), Type::Other(other));
}

#[test]
fn plans_x86_64_relative_relocation_as_base_plus_explicit_addend() {
    let relocation = Relocation {
        offset: 0x2000,
        symbol_index: 0,
        r#type: GenericType::from_raw(R_X86_64_RELATIVE),
        addend: Some(0x1234),
    };

    assert_eq!(
        relative_write(relocation, base(0x500000)),
        Ok(RelativeWrite {
            link_time_virtual_address: 0x2000,
            load_time_virtual_address: 0x502000,
            value: 0x501234,
        }),
    );
}

#[test]
fn permits_negative_x86_64_relative_addend_when_result_remains_representable() {
    let relocation = Relocation {
        offset: 0x2000,
        symbol_index: 0,
        r#type: GenericType::from_raw(R_X86_64_RELATIVE),
        addend: Some(-0x1000),
    };

    assert_eq!(
        relative_write(relocation, base(0x500000)),
        Ok(RelativeWrite {
            link_time_virtual_address: 0x2000,
            load_time_virtual_address: 0x502000,
            value: 0x4ff000,
        }),
    );
}

#[test]
fn rejects_non_relative_type_for_x86_64_relative_semantics() {
    let relocation = Relocation {
        offset: 0,
        symbol_index: 0,
        r#type: GenericType::from_raw(1),
        addend: Some(0),
    };

    assert_eq!(
        relative_write(relocation, base(0x500000)),
        Err(RelativeError::WrongType { raw: 1 }),
    );
}

#[test]
fn rejects_symbol_reference_in_x86_64_relative_relocation() {
    let relocation = Relocation {
        offset: 0,
        symbol_index: 1,
        r#type: GenericType::from_raw(R_X86_64_RELATIVE),
        addend: Some(0),
    };

    assert_eq!(
        relative_write(relocation, base(0x500000)),
        Err(RelativeError::SymbolIndexNotZero { symbol_index: 1 }),
    );
}

#[test]
fn rejects_implicit_addend_for_x86_64_lp64_relative_relocation() {
    let relocation = Relocation {
        offset: 0,
        symbol_index: 0,
        r#type: GenericType::from_raw(R_X86_64_RELATIVE),
        addend: None,
    };

    assert_eq!(
        relative_write(relocation, base(0x500000)),
        Err(RelativeError::MissingExplicitAddend),
    );
}

#[test]
fn rejects_x86_64_relative_value_outside_word64() {
    let relocation = Relocation {
        offset: 0,
        symbol_index: 0,
        r#type: GenericType::from_raw(R_X86_64_RELATIVE),
        addend: Some(0x2000),
    };
    let base = base(u64::MAX & !0xfff);

    assert!(matches!(
        relative_write(relocation, base),
        Err(RelativeError::ValueOutOfRange { .. }),
    ));
}

#[test]
fn rejects_x86_64_relative_relocation_place_overflow() {
    let relocation = Relocation {
        offset: u64::MAX,
        symbol_index: 0,
        r#type: GenericType::from_raw(R_X86_64_RELATIVE),
        addend: Some(0),
    };

    assert_eq!(
        relative_write(relocation, base(0x1000)),
        Err(RelativeError::LoadTimeVirtualAddressOverflow {
            link_time_virtual_address: u64::MAX,
        }),
    );
}

#[test]
fn applies_x86_64_relative_write_at_load_time_address() {
    let write = RelativeWrite {
        link_time_virtual_address: 0x2000,
        load_time_virtual_address: 0x502000,
        value: 0x501234,
    };

    let mut bytes = [0u8; 8];
    let mut regions = ample::r#type::Vec::new();
    regions.push(RegionWriter::new(0x502000, &mut bytes));
    let mut image = MemoryImageWriter::new(regions);

    assert_eq!(write.apply(&mut image), Ok(()));
    drop(image);

    assert_eq!(bytes, 0x501234u64.to_le_bytes());
}
