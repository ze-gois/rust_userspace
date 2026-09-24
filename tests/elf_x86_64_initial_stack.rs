use userspace::{
    file::format::elf::processor_specific::x86_64::stack::initial::{
        ALIGNMENT, AuxiliaryEntry, Error as InitialStackError, Image,
    },
    target::operating_system::process_stack,
};

#[test]
fn serializes_x86_64_initial_process_stack_in_psabi_order() {
    let argument_0 = b"program\0";
    let argument_1 = b"argument\0";
    let environment_0 = b"KEY=VALUE\0";

    let arguments = [
        argument_0.as_ptr(),
        argument_1.as_ptr(),
    ];
    let environment = [environment_0.as_ptr()];
    let auxiliary = [
        AuxiliaryEntry::new(6, 0x1000),
        AuxiliaryEntry::new(9, 0x401000),
        AuxiliaryEntry::NULL,
        AuxiliaryEntry::new(31, 0xdead_beef),
    ];

    let image = Image::new(&arguments, &environment, &auxiliary);

    assert_eq!(
        image.words(),
        &[
            2,
            argument_0.as_ptr() as usize,
            argument_1.as_ptr() as usize,
            0,
            environment_0.as_ptr() as usize,
            0,
            6,
            0x1000,
            9,
            0x401000,
            0,
            0,
        ],
    );
}

#[test]
fn writes_initial_process_stack_at_sixteen_byte_aligned_pointer() {
    let argument = b"program\0";
    let arguments = [argument.as_ptr()];
    let auxiliary = [AuxiliaryEntry::new(6, 0x1000)];
    let image = Image::new(&arguments, &[], &auxiliary);

    let mut memory = [0u8; 256];
    let stack_pointer = image
        .write(&mut memory)
        .expect("initial stack must fit");

    assert_eq!(stack_pointer as usize % ALIGNMENT, 0);

    let words = unsafe {
        core::slice::from_raw_parts(
            stack_pointer.cast::<usize>(),
            image.word_len(),
        )
    };
    assert_eq!(words, image.words());
}

#[test]
fn rejects_memory_too_small_for_initial_process_stack() {
    let arguments = [core::ptr::null()];
    let image = Image::new(&arguments, &[], &[]);
    let mut memory = [0u8; 8];

    assert!(matches!(
        image.write(&mut memory),
        Err(InitialStackError::InsufficientMemory { .. }),
    ));
}

#[test]
fn linux_maps_and_writes_x86_64_initial_process_stack() {
    let argument = b"program\0";
    let arguments = [argument.as_ptr()];
    let auxiliary = [
        AuxiliaryEntry::new(6, 0x1000),
        AuxiliaryEntry::new(9, 0x401000),
    ];
    let image = Image::new(&arguments, &[], &auxiliary);

    let mut mapping =
        process_stack::map(0x4000).expect("Linux stack mapping must succeed");
    let stack_pointer = mapping
        .write(&image)
        .expect("initial process stack must write");

    let start = mapping.address() as usize;
    let end = start + mapping.length();

    assert!(stack_pointer as usize >= start);
    assert!((stack_pointer as usize) < end);
    assert_eq!(stack_pointer as usize % ALIGNMENT, 0);

    let words = unsafe {
        core::slice::from_raw_parts(
            stack_pointer.cast::<usize>(),
            image.word_len(),
        )
    };
    assert_eq!(words, image.words());

    assert!(mapping.unmap());
}
