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
        "entry={:#x} phoff={:#x} phentsize={} phnum={} shoff={:#x} shentsize={} shnum_encoding={:?} shnum={} shstrndx_encoding={:?} shstrndx={:?}\n",
        header.entry,
        header.program_header_offset,
        header.program_header_entry_size,
        header.program_header_count,
        header.section_header_offset,
        header.section_header_entry_size,
        header.section_header_count,
        object_file.section_header_count,
        header.section_name_string_table_index,
        object_file.section_name_string_table_index,
    );

    userspace::info!(
        "entry_file_offset={:?} lowest_load_virtual_address={:?}\n",
        object_file.file_offset_for_virtual_address(header.entry),
        object_file.lowest_load_virtual_address(),
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

    userspace::info!(
        "program_header_conformance={:?}\n",
        object_file.validate_program_headers()
    );

    userspace::info!("loadable_segments\n");
    for index in 0..object_file.program_headers.len() {
        let Some(segment) = object_file.loadable_segment(index) else {
            continue;
        };

        userspace::info!(
            "  ph[{}] = {{ file_size: {:#x}, memory_size: {:#x}, zero_fill_size: {:#x}, alignment: {:#x}, readable: {}, writable: {}, executable: {} }}\n",
            index,
            segment.file_size(),
            segment.memory_size(),
            segment.zero_fill_size(),
            segment.alignment(),
            segment.program_header.flags.readable(),
            segment.program_header.flags.writable(),
            segment.program_header.flags.executable(),
        );

        if let Some(sections) = object_file.sections_in_loadable_segment(index) {
            for section in sections {
                userspace::info!(
                    "    section[{}] {:?} contribution={:?}\n",
                    section.section_index,
                    object_file.section_name(section.section_index),
                    section.contribution,
                );
            }
        }
    }

    if let Some(interpreter) = object_file.program_interpreter() {
        userspace::info!(
            "program_interpreter = {:?}\n",
            interpreter.pathname_str(),
        );
    }

    if let Some(program_header_table_image) = object_file.program_header_table_image() {
        userspace::info!(
            "program_header_table_image = {{ file_size: {:#x}, virtual_address: {:#x} }}\n",
            program_header_table_image.program_header.file_size,
            program_header_table_image.program_header.virtual_address,
        );
    }

    if let Some(template) = object_file.thread_local_storage_template() {
        userspace::info!(
            "thread_local_storage = {{ initialization_size: {:#x}, total_size: {:#x}, zero_fill_size: {:#x}, alignment: {:#x} }}\n",
            template.initialization_size(),
            template.total_size(),
            template.zero_fill_size(),
            template.alignment(),
        );
    }

    for (index, program_header) in object_file.program_headers.iter().enumerate() {
        match program_header.r#type {
            userspace::file::format::elf::program_header::Type::Dynamic => {
                if let Some(array) = object_file.dynamic_array_from_program_header(index) {
                    userspace::info!(
                        "dynamic_array ph[{}]: entries[{}]\n",
                        index,
                        array.len(),
                    );

                    if let Some(strings) =
                        object_file.dynamic_string_table_from_program_header(index)
                    {
                        for entry in array.iter() {
                            if matches!(
                                entry.tag,
                                userspace::file::format::elf::dynamic::Tag::Needed
                                    | userspace::file::format::elf::dynamic::Tag::SharedObjectName
                                    | userspace::file::format::elf::dynamic::Tag::RuntimeSearchPath
                                    | userspace::file::format::elf::dynamic::Tag::RunPath
                            ) {
                                userspace::info!(
                                    "  {:?} = {:?}\n",
                                    entry.tag,
                                    strings.get_str(entry.payload as usize),
                                );
                            }
                        }
                    }
                }
            }
            userspace::file::format::elf::program_header::Type::Note => {
                if let Some(notes) = object_file.note_table_from_program_header(index) {
                    userspace::info!(
                        "notes ph[{}]: entries[{}]\n",
                        index,
                        notes.len(),
                    );
                    for (note_index, note) in notes.iter().enumerate() {
                        userspace::info!(
                            "  note[{}] = {{ name: {:?}, type: {}, descriptor_size: {} }}\n",
                            note_index,
                            core::str::from_utf8(note.name).ok(),
                            note.r#type,
                            note.descriptor.len(),
                        );
                    }
                }
            }
            _ => {}
        }
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
            "  section[{}] {:?}: symbols[{}] local={} non_local={}\n",
            section_index,
            object_file.section_name(section_index),
            symbol_table.len(),
            symbol_table.local_symbols().len(),
            symbol_table.non_local_symbols().len(),
        );

        const SYMBOL_PREVIEW: usize = 32;
        for (symbol_index, symbol) in symbol_table.iter().take(SYMBOL_PREVIEW).enumerate() {
            userspace::info!(
                "    symbol[{}] = {{ name: {:?}, binding: {:?}, type: {:?}, visibility: {:?}, section_index_encoding: {:#x}, section_index: {:?}, value: {:#x}, size: {:#x} }}\n",
                symbol_index,
                symbol_table.name(symbol_index),
                symbol.binding,
                symbol.r#type,
                symbol.visibility,
                symbol.section_index.raw(),
                symbol_table.section_index(symbol_index),
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
