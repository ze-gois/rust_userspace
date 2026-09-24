#![no_std]
#![no_main]

use ample::r#type::Vec;

use userspace::file::format::elf::{
    ObjectFile,
    header::Type as ObjectType,
    processor_specific::x86_64::{
        entry_point,
        stack::initial::{AuxiliaryEntry, Image as InitialStack},
    },
};
use userspace::target::operating_system::{
    process_image,
    process_stack,
    syscall,
};

const PAGE_SIZE: usize = 0x1000;
const PROCESS_STACK_SIZE: usize = 1024 * 1024;
const DECIMAL_ARGUMENT_SIZE: usize = 21;

#[unsafe(no_mangle)]
pub extern "C" fn entry(
    stack_pointer: userspace::target::architecture::StackPointer,
) -> ! {
    let stack = unsafe { userspace::memory::Stack::from_pointer(stack_pointer) };

    let natural_number = stack
        .arguments
        .get(1)
        .and_then(|argument| argument.as_natural_number());

    match natural_number {
        Some(number) => userspace::info!("userspace {}\n", number),
        None => userspace::info!("userspace\n"),
    }

    let Some(number) = natural_number else {
        syscall::exit(0)
    };
    let Some(next_number) = number.checked_sub(1) else {
        syscall::exit(0)
    };

    let Some(argument_0) = stack.arguments.get(0) else {
        userspace::info!("userspace self execution requires argv[0]\n");
        syscall::exit(1)
    };
    let Some(path) = argument_0.as_c_str() else {
        userspace::info!("userspace argv[0] is not a C string\n");
        syscall::exit(1)
    };

    let Some(bytes) = userspace::file::read(path) else {
        userspace::info!("failed to read userspace from argv[0]\n");
        syscall::exit(1)
    };

    let object_file = match ObjectFile::parse(&bytes) {
        Ok(object_file) => object_file,
        Err(error) => {
            userspace::info!("failed to parse userspace ELF: {:?}\n", error);
            syscall::exit(1)
        }
    };

    if !matches!(object_file.header.r#type, ObjectType::SharedObject) {
        userspace::info!(
            "userspace self execution requires static PIE ET_DYN, got {:?}\n",
            object_file.header.r#type,
        );
        syscall::exit(1)
    }

    if let Some(interpreter) = object_file.program_interpreter() {
        userspace::info!(
            "userspace self execution requires no PT_INTERP, got {:?}\n",
            interpreter.pathname_str(),
        );
        syscall::exit(1)
    }

    let mapping = match process_image::map(&object_file, PAGE_SIZE) {
        Ok(mapping) => mapping,
        Err(error) => {
            userspace::info!("failed to map userspace process image: {:?}\n", error);
            syscall::exit(1)
        }
    };

    let Some(entry) = mapping.entry_address(&object_file) else {
        userspace::info!("userspace e_entry is not executable\n");
        syscall::exit(1)
    };

    let mut next_argument = [0u8; DECIMAL_ARGUMENT_SIZE];
    let Some(next_argument_pointer) =
        decimal_argument(next_number, &mut next_argument)
    else {
        userspace::info!("failed to represent next natural-number argument\n");
        syscall::exit(1)
    };

    let arguments = [
        path.as_ptr().cast::<u8>(),
        next_argument_pointer,
    ];

    let mut environment = Vec::with_capacity(stack.environment.len());
    for variable in stack.environment.iter() {
        environment.push(variable.pointer());
    }

    let auxiliary = [
        AuxiliaryEntry::new(6, PAGE_SIZE),
        AuxiliaryEntry::new(9, entry as usize),
        AuxiliaryEntry::new(31, path.as_ptr() as usize),
    ];

    let initial_stack =
        InitialStack::new(&arguments, environment.as_slice(), &auxiliary);

    let mut stack_mapping = match process_stack::map(PROCESS_STACK_SIZE) {
        Ok(mapping) => mapping,
        Err(error) => {
            userspace::info!("failed to map userspace process stack: {:?}\n", error);
            syscall::exit(1)
        }
    };

    let new_stack_pointer = match stack_mapping.write(&initial_stack) {
        Ok(stack_pointer) => stack_pointer,
        Err(error) => {
            userspace::info!("failed to write userspace initial stack: {:?}\n", error);
            syscall::exit(1)
        }
    };

    unsafe {
        entry_point::transfer_control(
            entry,
            new_stack_pointer,
            None,
        )
    }
}

fn decimal_argument(
    mut number: usize,
    storage: &mut [u8; DECIMAL_ARGUMENT_SIZE],
) -> Option<*const u8> {
    let terminator = storage.len().checked_sub(1)?;
    storage[terminator] = 0;

    let mut start = terminator;
    loop {
        start = start.checked_sub(1)?;
        storage[start] = b'0' + u8::try_from(number % 10).ok()?;
        number /= 10;

        if number == 0 {
            break;
        }
    }

    Some(storage[start..].as_ptr())
}
