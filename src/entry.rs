#![no_std]
#![no_main]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use userspace::info;

#[derive(Debug)]
pub struct Origin;

ample::trait_implement_primitives!();

#[inline(never)]
fn jump_to_loaded_entry(entry: u64, initial_stack: userspace::target::arch::PointerType) -> ! {
    unsafe { userspace::file::format::elf::loader::jump_to_entry(entry, initial_stack) }
}

#[unsafe(no_mangle)]
pub extern "C" fn entry(stack_pointer: userspace::target::arch::PointerType) -> ! {
    let stack_pointer = userspace::target::arch::Pointer(stack_pointer);
    let stack = userspace::memory::Stack::from_pointer(stack_pointer);
    stack.arguments.print();

    let (target_path, target_path_pointer) = match stack.arguments.get(1) {
        Some(argument) if !argument.pointer.0.is_null() => {
            use userspace::traits::Str;
            (
                <&str>::from_null_terminated_pointer(argument.pointer.0 as *const u8),
                argument.pointer.0 as *const u8,
            )
        }
        _ => ("/usr/bin/pwd", b"/usr/bin/pwd\0".as_ptr()),
    };

    let prepared = match userspace::file::format::elf::loader::prepare_execution(
        target_path,
        target_path_pointer,
        stack_pointer.0,
    ) {
        Ok(prepared) => prepared,
        Err(error) => {
            info!("ELF execution preparation failed: {:?}\n", error);
            userspace::target::os::syscall::exit(126);
        }
    };

    let image = prepared.image;
    info!(
        "Loaded ELF image {:?}: bias={:#x}, entry={:#x}, range={:#x}..{:#x}, segments={}, interpreter={}\n",
        target_path,
        image.base,
        image.entry,
        image.base,
        image.end,
        image.segment_count,
        image.interpreter.is_some(),
    );

    for index in 0..image.segment_count {
        if let Some(segment) = image.segments[index] {
            info!(
                "  PT_LOAD[{}]: addr={:#x}, vaddr={:#x}, offset={:#x}, filesz={:#x}, memsz={:#x}, flags={:#x}, align={:#x}, map={:#x}..{:#x}\n",
                segment.index,
                segment.address,
                segment.virtual_address,
                segment.file_offset,
                segment.file_size,
                segment.memory_size,
                segment.flags,
                segment.alignment,
                segment.map_start,
                segment.map_end,
            );
        }
    }

    jump_to_loaded_entry(prepared.entry, prepared.stack_pointer);
}
