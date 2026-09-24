use userspace::file::format::elf::{
    dynamic::validation::ValidationError,
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

fn sxword(bytes: &mut Vec<u8>, value: i64, order: ByteOrder) {
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

fn dynamic_entry(bytes: &mut Vec<u8>, tag: i64, payload: u64, order: ByteOrder) {
    sxword(bytes, tag, order);
    xword(bytes, payload, order);
}

fn fixture(order: ByteOrder, shared_object: bool, pre_initialization: bool) -> Vec<u8> {
    const BASE: u64 = 0x400000;
    const HEADER_SIZE: u64 = 64;
    const PROGRAM_HEADER_SIZE: u64 = 56;
    const PROGRAM_HEADER_COUNT: u64 = 2;
    const DYNAMIC_OFFSET: u64 = HEADER_SIZE + PROGRAM_HEADER_SIZE * PROGRAM_HEADER_COUNT;

    let dynamic_entry_count = if pre_initialization { 9 } else { 7 };
    let dynamic_size = dynamic_entry_count * 16;
    let initialization_array_offset = DYNAMIC_OFFSET + dynamic_size;
    let termination_array_offset = initialization_array_offset + 16;
    let pre_initialization_array_offset = termination_array_offset + 8;
    let total_size = if pre_initialization {
        pre_initialization_array_offset + 8
    } else {
        termination_array_offset + 8
    };

    let data = match order {
        ByteOrder::Little => 1u8,
        ByteOrder::Big => 2u8,
    };

    let mut bytes = Vec::with_capacity(total_size as usize);
    bytes.extend_from_slice(&[
        0x7f, b'E', b'L', b'F',
        2, data, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]);

    half(&mut bytes, if shared_object { 3 } else { 2 }, order);
    half(&mut bytes, 0x3e, order);
    word(&mut bytes, 1, order);
    xword(&mut bytes, BASE, order);
    xword(&mut bytes, HEADER_SIZE, order);
    xword(&mut bytes, 0, order);
    word(&mut bytes, 0, order);
    half(&mut bytes, HEADER_SIZE as u16, order);
    half(&mut bytes, PROGRAM_HEADER_SIZE as u16, order);
    half(&mut bytes, PROGRAM_HEADER_COUNT as u16, order);
    half(&mut bytes, 64, order);
    half(&mut bytes, 0, order);
    half(&mut bytes, 0, order);

    // PT_LOAD
    word(&mut bytes, 1, order);
    word(&mut bytes, 4, order);
    xword(&mut bytes, 0, order);
    xword(&mut bytes, BASE, order);
    xword(&mut bytes, BASE, order);
    xword(&mut bytes, total_size, order);
    xword(&mut bytes, total_size, order);
    xword(&mut bytes, 0x1000, order);

    // PT_DYNAMIC
    word(&mut bytes, 2, order);
    word(&mut bytes, 4, order);
    xword(&mut bytes, DYNAMIC_OFFSET, order);
    xword(&mut bytes, BASE + DYNAMIC_OFFSET, order);
    xword(&mut bytes, BASE + DYNAMIC_OFFSET, order);
    xword(&mut bytes, dynamic_size, order);
    xword(&mut bytes, dynamic_size, order);
    xword(&mut bytes, 8, order);

    // DT_INIT
    dynamic_entry(&mut bytes, 12, 0x401111, order);
    // DT_INIT_ARRAY / DT_INIT_ARRAYSZ
    dynamic_entry(
        &mut bytes,
        25,
        BASE + initialization_array_offset,
        order,
    );
    dynamic_entry(&mut bytes, 27, 16, order);
    // DT_FINI_ARRAY / DT_FINI_ARRAYSZ
    dynamic_entry(
        &mut bytes,
        26,
        BASE + termination_array_offset,
        order,
    );
    dynamic_entry(&mut bytes, 28, 8, order);
    // DT_FINI
    dynamic_entry(&mut bytes, 13, 0x402222, order);

    if pre_initialization {
        dynamic_entry(
            &mut bytes,
            32,
            BASE + pre_initialization_array_offset,
            order,
        );
        dynamic_entry(&mut bytes, 33, 8, order);
    }

    // DT_NULL
    dynamic_entry(&mut bytes, 0, 0, order);

    xword(&mut bytes, 0x410001, order);
    xword(&mut bytes, 0x410002, order);
    xword(&mut bytes, 0x420001, order);
    if pre_initialization {
        xword(&mut bytes, 0x430001, order);
    }

    assert_eq!(bytes.len(), total_size as usize);
    bytes
}

fn assert_functions(bytes: &[u8]) {
    let object = ObjectFile::parse(bytes).expect("dynamic fixture must parse");
    object
        .validate_dynamic_initialization_and_termination(1)
        .expect("dynamic initialization must conform");

    let functions = object
        .initialization_and_termination_functions_from_program_header(1)
        .expect("functions must resolve");

    assert_eq!(
        functions.initialization.function.map(|function| function.value()),
        Some(0x401111)
    );
    assert_eq!(functions.initialization.functions.len(), 2);
    assert_eq!(functions.initialization.functions[0].value(), 0x410001);
    assert_eq!(functions.initialization.functions[1].value(), 0x410002);

    assert_eq!(functions.termination.functions.len(), 1);
    assert_eq!(functions.termination.functions[0].value(), 0x420001);
    assert_eq!(
        functions.termination.function.map(|function| function.value()),
        Some(0x402222)
    );
}

#[test]
fn resolves_initialization_and_termination_little_endian() {
    assert_functions(&fixture(ByteOrder::Little, false, false));
}

#[test]
fn resolves_initialization_and_termination_big_endian() {
    assert_functions(&fixture(ByteOrder::Big, false, false));
}

#[test]
fn resolves_pre_initialization_for_executable() {
    let bytes = fixture(ByteOrder::Little, false, true);
    let object = ObjectFile::parse(&bytes).expect("executable fixture must parse");
    object
        .validate_dynamic_initialization_and_termination(1)
        .expect("pre-initialization is valid for executable");

    let functions = object
        .initialization_and_termination_functions_from_program_header(1)
        .expect("functions must resolve");
    let pre = functions.pre_initialization.expect("pre-init array expected");
    assert_eq!(pre.functions.len(), 1);
    assert_eq!(pre.functions[0].value(), 0x430001);
}

#[test]
fn rejects_pre_initialization_for_shared_object() {
    let bytes = fixture(ByteOrder::Little, true, true);
    let object = ObjectFile::parse(&bytes).expect("shared-object fixture must parse");

    assert_eq!(
        object.validate_dynamic_initialization_and_termination(1),
        Err(ValidationError::PreInitializationInSharedObject),
    );
}
