use std::{env, fs, process};

use userspace::file::format::elf::{
    program_header, section_header, ObjectFile,
};

fn main() {
    let path = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("usage: cargo run --example elf_inspection -- <elf-file>");
        process::exit(2);
    });

    let bytes = fs::read(&path).unwrap_or_else(|error| {
        eprintln!("{path}: {error}");
        process::exit(2);
    });

    let object = ObjectFile::parse(&bytes).unwrap_or_else(|error| {
        eprintln!("{path}: ELF parse error: {error:?}");
        process::exit(1);
    });

    println!("ELF inspection: {path}");
    println!();

    println!("== ELF Header ==");
    println!("{:#?}", object.header);
    println!();

    println!("== Program Headers ==");
    for (index, header) in object.program_headers.iter().enumerate() {
        println!("[{index}] {header:#?}");

        if let Some(segment) = object.segment(index) {
            println!("    file image: {} bytes", segment.file_image.len());
        }

        if let Some(loadable) = object.loadable_segment(index) {
            println!(
                "    loadable: file={} memory={} zero_fill={} alignment={}",
                loadable.file_size(),
                loadable.memory_size(),
                loadable.zero_fill_size(),
                loadable.alignment(),
            );
        }

        if matches!(header.r#type, program_header::Type::Dynamic) {
            if let Some(array) = object.dynamic_array_from_program_header(index) {
                let strings = object.dynamic_string_table_from_program_header(index);
                println!("    dynamic array:");
                for (entry_index, entry) in array.iter().enumerate() {
                    let string = strings
                        .as_ref()
                        .and_then(|table| match entry.tag {
                            userspace::file::format::elf::dynamic::Tag::Needed
                            | userspace::file::format::elf::dynamic::Tag::SharedObjectName
                            | userspace::file::format::elf::dynamic::Tag::RuntimeSearchPath
                            | userspace::file::format::elf::dynamic::Tag::RunPath => {
                                table.get_str(entry.payload as usize)
                            }
                            _ => None,
                        });
                    if let Some(string) = string {
                        println!("      [{entry_index}] {entry:?} -> {string:?}");
                    } else {
                        println!("      [{entry_index}] {entry:?}");
                    }
                }
            }

            if let Some(flags) = object.dynamic_flags_from_program_header(index) {
                println!("    dynamic flags: {:#x}", flags.raw());
            }

            if let Some(strings) = object.dynamic_string_table_from_program_header(index) {
                println!("    dynamic string table: {} bytes", strings.bytes().len());
            }

            if let Some(symbols) = object.dynamic_symbol_table_from_program_header(index) {
                println!("    dynamic symbols ({}):", symbols.len());
                for (symbol_index, symbol) in symbols.iter().enumerate() {
                    println!(
                        "      [{symbol_index}] name={:?} section={:?} symbol={symbol:?}",
                        symbols.name(symbol_index),
                        symbols.section_index(symbol_index),
                    );
                }
            }

            if let Some(hash) = object.dynamic_hash_table_from_program_header(index) {
                println!(
                    "    System V hash table: buckets={} chains={} symbols={}",
                    hash.table.buckets.len(),
                    hash.table.chains.len(),
                    hash.symbols.len(),
                );
            }

            if let Some(tables) = object.dynamic_relocation_tables_from_program_header(index) {
                println!("    dynamic relocation tables ({}):", tables.len());
                for (table_index, table) in tables.iter().enumerate() {
                    println!(
                        "      [{table_index}] addend={:?} purpose={:?} entries={}",
                        table.addend,
                        table.purpose,
                        table.len(),
                    );
                    for (relocation_index, relocation) in table.iter().enumerate() {
                        println!(
                            "        [{relocation_index}] symbol={:?} relocation={relocation:?}",
                            table.symbol_name(relocation),
                        );
                    }
                }
            }

            if let Some(table) = object.relative_relocation_table_from_program_header(index) {
                println!("    dynamic relative relocations ({}):", table.len());
                for (entry_index, entry) in table.iter().enumerate() {
                    println!("      [{entry_index}] {entry:?}");
                }
            }

            if let Some(functions) =
                object.initialization_and_termination_functions_from_program_header(index)
            {
                println!("    initialization / termination: {functions:#?}");
            }

            if let Some(dependencies) =
                object.shared_object_dependencies_from_program_header(index)
            {
                println!("    shared-object dependencies: {dependencies:#?}");
            }
        }
    }
    println!();

    if let Some(interpreter) = object.program_interpreter() {
        println!("== Program Interpreter ==");
        println!("pathname: {:?}", interpreter.pathname_str());
        println!("{:#?}", interpreter.program_header);
        println!();
    }

    if let Some(image) = object.program_header_table_image() {
        println!("== Program Header Table Image ==");
        println!("{} bytes", image.bytes.len());
        println!("{:#?}", image.program_header);
        println!();
    }

    if let Some(template) = object.thread_local_storage_template() {
        println!("== Thread-Local Storage Template ==");
        println!(
            "initialization={} total={} zero_fill={} alignment={}",
            template.initialization_size(),
            template.total_size(),
            template.zero_fill_size(),
            template.alignment(),
        );
        println!("{:#?}", template.program_header);
        println!();
    }

    println!("== Section Headers ==");
    for (index, header) in object.section_headers.iter().enumerate() {
        println!(
            "[{index}] name={:?} {header:#?}",
            object.section_name(index),
        );

        if let Some(section) = object.section(index) {
            println!("    contents: {} bytes", section.len());
        }

        if let Some((program_header_index, membership)) =
            object.loadable_segment_for_section(index)
        {
            println!(
                "    loadable segment: program_header={program_header_index} contribution={:?}",
                membership.contribution,
            );
        }

        if let Some(link_order) = object.link_order(index) {
            println!(
                "    link order: metadata={} referenced={}",
                link_order.metadata_index,
                link_order.referenced_index,
            );
        }

        if header.flags.contains(section_header::Flags::COMPRESSED) {
            println!(
                "    compressed validation: {:?}",
                object.validate_compressed_section(index),
            );
            if let Some(compressed) = object.compressed_section(index) {
                println!(
                    "    compression: type={:?} uncompressed_size={} alignment={} data={} bytes",
                    compressed.header.r#type,
                    compressed.header.uncompressed_size,
                    compressed.header.alignment,
                    compressed.data.len(),
                );
            }
        }

        if let Some(symbols) = object.symbol_table(index) {
            println!("    symbols ({}):", symbols.len());
            for (symbol_index, symbol) in symbols.iter().enumerate() {
                println!(
                    "      [{symbol_index}] name={:?} section={:?} symbol={symbol:?}",
                    symbols.name(symbol_index),
                    symbols.section_index(symbol_index),
                );
            }
        }

        if let Some(relocations) = object.relocation_table(index) {
            println!(
                "    relocations ({}) -> section {}:",
                relocations.len(),
                relocations.target_section_index,
            );
            for (relocation_index, relocation) in relocations.iter().enumerate() {
                println!(
                    "      [{relocation_index}] symbol={:?} relocation={relocation:?}",
                    relocations.symbol_name(relocation),
                );
            }
        }

        if matches!(header.r#type, section_header::Type::RelativeRelocation) {
            println!(
                "    relative relocation validation: {:?}",
                object.validate_relative_relocation_section(index),
            );
            if let Some(table) = object.relative_relocation_table(index) {
                println!("    relative relocations ({}):", table.len());
                for (entry_index, entry) in table.iter().enumerate() {
                    println!("      [{entry_index}] {entry:?}");
                }
            }
        }

        if let Some(dynamic) = object.dynamic_section(index) {
            println!("    dynamic section:");
            for (entry_index, entry) in dynamic.array.iter().enumerate() {
                println!("      [{entry_index}] {entry:?}");
            }
        }

        if let Some(hash) = object.hash_table(index) {
            println!(
                "    System V hash table: buckets={} chains={}",
                hash.buckets.len(),
                hash.chains.len(),
            );
        }

        if let Some(group) = object.section_group(index) {
            println!(
                "    section group: flags={:?} signature={:?} members={:?}",
                group.flags,
                group.signature_name(),
                group.members,
            );
        }

        if let Some(notes) = object.note_table(index) {
            println!("    notes ({}):", notes.len());
            for (note_index, note) in notes.iter().enumerate() {
                println!("      [{note_index}] {note:?}");
            }
        }
    }
    println!();

    println!("== Program Notes ==");
    for index in 0..object.program_headers.len() {
        if let Some(notes) = object.note_table_from_program_header(index) {
            println!("program header [{index}] notes ({}):", notes.len());
            for (note_index, note) in notes.iter().enumerate() {
                println!("  [{note_index}] {note:?}");
            }
        }
    }

    println!();
    println!("== Validation ==");
    println!("program headers: {:?}", object.validate_program_headers());
    for index in 0..object.program_headers.len() {
        if matches!(
            object.program_headers[index].r#type,
            program_header::Type::Dynamic
        ) {
            println!(
                "dynamic[{index}] initialization / termination: {:?}",
                object.validate_dynamic_initialization_and_termination(index),
            );
            println!(
                "dynamic[{index}] relative relocation: {:?}",
                object.validate_dynamic_relative_relocation(index),
            );
        }
    }
}
