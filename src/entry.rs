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
const EXECUTION_COUNT: usize = 3;

const EXECUTION_2: &[u8] = b"--userspace-self-execution=2\0";
const EXECUTION_3: &[u8] = b"--userspace-self-execution=3\0";

#[unsafe(no_mangle)]
pub extern "C" fn entry(
    stack_pointer: userspace::target::architecture::StackPointer,
) -> ! {
    let stack = unsafe { userspace::memory::Stack::from_pointer(stack_pointer) };
    let execution = execution_number(&stack);

    userspace::info!(
        "userspace self execution {}/{}\n",
        execution,
        EXECUTION_COUNT,
    );

    if execution >= EXECUTION_COUNT {
        syscall::exit(0)
    }

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

    let next_marker = match execution + 1 {
        2 => EXECUTION_2,
        3 => EXECUTION_3,
        _ => {
            userspace::info!("invalid userspace self-execution generation\n");
            syscall::exit(1)
        }
    };

    let arguments = [
        path.as_ptr().cast::<u8>(),
        next_marker.as_ptr(),
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

    userspace::info!(
        "transferring userspace execution {} -> {} at entry {:p}\n",
        execution,
        execution + 1,
        entry,
    );

    unsafe {
        entry_point::transfer_control(
            entry,
            new_stack_pointer,
            None,
        )
    }
}

fn execution_number(stack: &userspace::memory::Stack) -> usize {
    match stack.arguments.get(1).and_then(|argument| argument.as_str()) {
        Some("--userspace-self-execution=2") => 2,
        Some("--userspace-self-execution=3") => 3,
        _ => 1,
    }
}
