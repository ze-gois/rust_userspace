#![no_std]
#![no_main]

#[unsafe(no_mangle)]
pub extern "C" fn entry(
    stack_pointer: userspace::target::architecture::StackPointer,
) -> ! {
    let stack = unsafe { userspace::memory::Stack::from_pointer(stack_pointer) };
    stack.print();

    let Some(argument_0) = stack.arguments.get(0) else {
        userspace::info!("argv[0] is absent\n");
        userspace::target::os::syscall::exit(1)
    };

    let Some(path) = argument_0.as_c_str() else {
        userspace::info!("argv[0] is not a valid C string\n");
        userspace::target::os::syscall::exit(1)
    };

    let Some(bytes) = userspace::file::read(path) else {
        userspace::info!("failed to read argv[0]\n");
        userspace::target::os::syscall::exit(1)
    };

    let object_file = match userspace::file::format::elf::ObjectFile::parse(&bytes) {
        Ok(object_file) => object_file,
        Err(error) => {
            userspace::info!("failed to parse argv[0] as ELF: {:?}\n", error);
            userspace::target::os::syscall::exit(1)
        }
    };

    let header = object_file.header;
    userspace::info!("--- ELF Object File ---\n");
    userspace::info!(
        "class={:?} data={:?} type={:?} machine={} version={:?}\n",
        header.identification.class,
        header.identification.data,
        header.r#type,
        header.machine.raw(),
        header.version,
    );
    userspace::info!(
        "entry={:#x} phoff={:#x} phentsize={} phnum={} shoff={:#x} shentsize={} shnum={} shstrndx={}\n",
        header.entry,
        header.program_header_offset,
        header.program_header_entry_size,
        header.program_header_count,
        header.section_header_offset,
        header.section_header_entry_size,
        header.section_header_count,
        header.section_name_string_table_index.raw(),
    );

    userspace::info!("program_headers[{}]\n", object_file.program_headers.len());
    for (index, program_header) in object_file.program_headers.iter().enumerate() {
        userspace::info!(
            "  ph[{}] = {{ type: {:?}, flags: {:#x}, offset: {:#x}, virtual_address: {:#x}, physical_address: {:#x}, file_size: {:#x}, memory_size: {:#x}, alignment: {:#x} }}\n",
            index,
            program_header.r#type,
            program_header.flags.raw(),
            program_header.offset,
            program_header.virtual_address,
            program_header.physical_address,
            program_header.file_size,
            program_header.memory_size,
            program_header.alignment,
        );
    }

    userspace::info!("section_headers[{}]\n", object_file.section_headers.len());
    for (index, section_header) in object_file.section_headers.iter().enumerate() {
        userspace::info!(
            "  sh[{}] = {{ name: {:?}, type: {:?}, flags: {:#x}, address: {:#x}, offset: {:#x}, size: {:#x}, link: {}, information: {}, alignment: {:#x}, entry_size: {:#x} }}\n",
            index,
            object_file.section_name(index),
            section_header.r#type,
            section_header.flags.raw(),
            section_header.address,
            section_header.offset,
            section_header.size,
            section_header.link,
            section_header.information,
            section_header.alignment,
            section_header.entry_size,
        );
    }
    userspace::info!("symbol_tables\n");
    for (section_index, section_header) in object_file.section_headers.iter().enumerate() {
        if !matches!(
            section_header.r#type,
            userspace::file::format::elf::section_header::Type::SymbolTable
                | userspace::file::format::elf::section_header::Type::DynamicSymbolTable
        ) {
            continue;
        }

        let Some(symbol_table) = object_file.symbol_table(section_index) else {
            userspace::info!(
                "  section[{}] {:?}: <invalid symbol table>\n",
                section_index,
                object_file.section_name(section_index),
            );
            continue;
        };

        userspace::info!(
            "  section[{}] {:?}: symbols[{}]\n",
            section_index,
            object_file.section_name(section_index),
            symbol_table.len(),
        );

        const SYMBOL_PREVIEW: usize = 32;
        for (symbol_index, symbol) in symbol_table.iter().take(SYMBOL_PREVIEW).enumerate() {
            userspace::info!(
                "    symbol[{}] = {{ name: {:?}, binding: {:?}, type: {:?}, visibility: {:?}, section_index: {}, value: {:#x}, size: {:#x} }}\n",
                symbol_index,
                symbol_table.name(symbol_index),
                symbol.binding,
                symbol.r#type,
                symbol.visibility,
                symbol.section_index.raw(),
                symbol.value,
                symbol.size,
            );
        }

        if symbol_table.len() > SYMBOL_PREVIEW {
            userspace::info!(
                "    ... {} additional symbols omitted from preview\n",
                symbol_table.len() - SYMBOL_PREVIEW,
            );
        }
    }

    userspace::info!("-----------------------\n");

    userspace::target::os::syscall::exit(0)
}
