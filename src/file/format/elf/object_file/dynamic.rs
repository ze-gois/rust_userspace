//! Dynamic-linking relationships resolved through an ELF object file.

use ample::r#type::Vec;

use super::{
    parser::{parse_dynamic_array, read},
    ObjectFile,
};
use super::super::{
    dynamic::{self, Tag},
    dynamic_array::DynamicArray,
    dynamic_hash_table::DynamicHashTable,
    dynamic_relocation_table::{
        Addend as DynamicRelocationAddend,
        DynamicRelocationTable,
        Purpose as DynamicRelocationPurpose,
    },
    dynamic_symbol_table::DynamicSymbolTable,
    hash::HashTable,
    identification::Class,
    program_header,
    relocation::{self, Relocation},
    section_header,
    shared_object_dependencies::SharedObjectDependencies,
    string_table::StringTable,
    symbol::{self, Symbol},
};

impl<'file> ObjectFile<'file> {
    fn system_v_hash_counts(&self, address: u64) -> Option<(u32, u32)> {
        let bytes = self.file_range_for_virtual_address(address, 8)?;
        Some((read::<u32>(bytes, 0)?, read::<u32>(bytes, 4)?))
    }

    pub fn dynamic_array_from_program_header(&self, index: usize) -> Option<DynamicArray> {
        let program_header = *self.program_headers.get(index)?;
        if !matches!(program_header.r#type, program_header::Type::Dynamic) {
            return None;
        }

        let segment = self.segment(index)?;
        let entry_size = match self.header.identification.class {
            Class::Class32 => core::mem::size_of::<dynamic::class_32::Representation>(),
            Class::Class64 => core::mem::size_of::<dynamic::class_64::Representation>(),
            Class::None | Class::Reserved(_) => return None,
        };

        parse_dynamic_array(
            segment.file_image,
            self.header.identification.class,
            entry_size,
        )
    }

    pub fn dynamic_string_table_from_program_header(
        &self,
        index: usize,
    ) -> Option<StringTable<'file>> {
        let array = self.dynamic_array_from_program_header(index)?;
        let address = array.first(Tag::StringTable)?.payload;
        let size = array.first(Tag::StringTableSize)?.payload;
        let bytes = self.file_range_for_virtual_address(address, size)?;
        Some(StringTable::new(bytes))
    }

    pub fn dynamic_symbol_table_from_program_header(
        &self,
        index: usize,
    ) -> Option<DynamicSymbolTable<'file>> {
        let array = self.dynamic_array_from_program_header(index)?;
        let strings = self.dynamic_string_table_from_program_header(index)?;

        let hash_address = array.first(Tag::Hash)?.payload;
        let (_, chain_count) = self.system_v_hash_counts(hash_address)?;

        let symbol_address = array.first(Tag::SymbolTable)?.payload;
        let entry_size = usize::try_from(array.first(Tag::SymbolEntrySize)?.payload).ok()?;
        let count = usize::try_from(chain_count).ok()?;
        let byte_size = count.checked_mul(entry_size)?;
        let symbol_bytes = self.file_range_for_virtual_address(
            symbol_address,
            u64::try_from(byte_size).ok()?,
        )?;

        let mut symbols = Vec::with_capacity(count);

        match self.header.identification.class {
            Class::Class32 => {
                if entry_size < core::mem::size_of::<symbol::class_32::Representation>() {
                    return None;
                }

                for symbol_index in 0..count {
                    let offset = symbol_index.checked_mul(entry_size)?;
                    let representation =
                        read::<symbol::class_32::Representation>(symbol_bytes, offset)?;
                    symbols.push(Symbol::from(representation));
                }
            }
            Class::Class64 => {
                if entry_size < core::mem::size_of::<symbol::class_64::Representation>() {
                    return None;
                }

                for symbol_index in 0..count {
                    let offset = symbol_index.checked_mul(entry_size)?;
                    let representation =
                        read::<symbol::class_64::Representation>(symbol_bytes, offset)?;
                    symbols.push(Symbol::from(representation));
                }
            }
            Class::None | Class::Reserved(_) => return None,
        }

        let extended_address = array
            .first(Tag::SymbolTableSectionIndex)
            .map(|entry| entry.payload);

        let extended_indices = if let Some(address) = extended_address {
            let size = count.checked_mul(core::mem::size_of::<u32>())?;
            Some(self.file_range_for_virtual_address(
                address,
                u64::try_from(size).ok()?,
            )?)
        } else {
            None
        };

        let mut section_indices = Vec::with_capacity(count);
        for (symbol_index, symbol) in symbols.iter().enumerate() {
            let extended = if let Some(bytes) = extended_indices {
                let offset = symbol_index.checked_mul(core::mem::size_of::<u32>())?;
                Some(read::<u32>(bytes, offset)?)
            } else {
                None
            };

            if symbol.section_index != section_header::Index::EXTENDED
                && extended.is_some_and(|value| value != 0)
            {
                return None;
            }

            section_indices.push(symbol::ResolvedSectionIndex::resolve(
                symbol.section_index,
                extended,
            )?);
        }

        Some(DynamicSymbolTable::new(
            symbols,
            strings,
            section_indices,
        ))
    }

    pub fn dynamic_hash_table_from_program_header(
        &self,
        index: usize,
    ) -> Option<DynamicHashTable<'file>> {
        let array = self.dynamic_array_from_program_header(index)?;
        let hash_address = array.first(Tag::Hash)?.payload;
        let (bucket_count, chain_count) = self.system_v_hash_counts(hash_address)?;

        let total_words = usize::try_from(bucket_count)
            .ok()?
            .checked_add(usize::try_from(chain_count).ok()?)?;
        let total_size = 8usize.checked_add(
            total_words.checked_mul(core::mem::size_of::<u32>())?,
        )?;
        let bytes = self.file_range_for_virtual_address(
            hash_address,
            u64::try_from(total_size).ok()?,
        )?;

        let mut offset = 8usize;
        let mut buckets = Vec::with_capacity(usize::try_from(bucket_count).ok()?);
        for _ in 0..bucket_count {
            buckets.push(read::<u32>(bytes, offset)?);
            offset = offset.checked_add(core::mem::size_of::<u32>())?;
        }

        let mut chains = Vec::with_capacity(usize::try_from(chain_count).ok()?);
        for _ in 0..chain_count {
            chains.push(read::<u32>(bytes, offset)?);
            offset = offset.checked_add(core::mem::size_of::<u32>())?;
        }

        Some(DynamicHashTable::new(
            HashTable::new(buckets, chains),
            self.dynamic_symbol_table_from_program_header(index)?,
        ))
    }

    pub fn dynamic_relocation_tables_from_program_header(
        &self,
        index: usize,
    ) -> Option<Vec<DynamicRelocationTable<'file>>> {
        let array = self.dynamic_array_from_program_header(index)?;
        let mut tables = Vec::new();

        if let (Some(address), Some(size), Some(entry_size)) = (
            array.first(Tag::Relocation).map(|entry| entry.payload),
            array.first(Tag::RelocationSize).map(|entry| entry.payload),
            array.first(Tag::RelocationEntrySize).map(|entry| entry.payload),
        ) {
            tables.push(self.dynamic_relocation_table_from_parts(
                index,
                address,
                size,
                entry_size,
                DynamicRelocationAddend::Implicit,
                DynamicRelocationPurpose::General,
            )?);
        }

        if let (Some(address), Some(size), Some(entry_size)) = (
            array
                .first(Tag::RelocationWithAddend)
                .map(|entry| entry.payload),
            array
                .first(Tag::RelocationWithAddendSize)
                .map(|entry| entry.payload),
            array
                .first(Tag::RelocationWithAddendEntrySize)
                .map(|entry| entry.payload),
        ) {
            tables.push(self.dynamic_relocation_table_from_parts(
                index,
                address,
                size,
                entry_size,
                DynamicRelocationAddend::Explicit,
                DynamicRelocationPurpose::General,
            )?);
        }

        if let (Some(address), Some(size), Some(format)) = (
            array.first(Tag::JumpRelocation).map(|entry| entry.payload),
            array
                .first(Tag::ProcedureLinkageTableRelocationSize)
                .map(|entry| entry.payload),
            array
                .first(Tag::ProcedureLinkageTableRelocation)
                .map(|entry| entry.payload),
        ) {
            let (addend, entry_size) = match Tag::from_raw(i64::try_from(format).ok()?) {
                Tag::Relocation => (
                    DynamicRelocationAddend::Implicit,
                    array.first(Tag::RelocationEntrySize)?.payload,
                ),
                Tag::RelocationWithAddend => (
                    DynamicRelocationAddend::Explicit,
                    array.first(Tag::RelocationWithAddendEntrySize)?.payload,
                ),
                _ => return None,
            };

            tables.push(self.dynamic_relocation_table_from_parts(
                index,
                address,
                size,
                entry_size,
                addend,
                DynamicRelocationPurpose::ProcedureLinkageTable,
            )?);
        }

        Some(tables)
    }

    fn dynamic_relocation_table_from_parts(
        &self,
        program_header_index: usize,
        address: u64,
        size: u64,
        entry_size: u64,
        addend: DynamicRelocationAddend,
        purpose: DynamicRelocationPurpose,
    ) -> Option<DynamicRelocationTable<'file>> {
        let entry_size = usize::try_from(entry_size).ok()?;
        let size = usize::try_from(size).ok()?;
        if entry_size == 0 || size % entry_size != 0 {
            return None;
        }

        let bytes = self.file_range_for_virtual_address(
            address,
            u64::try_from(size).ok()?,
        )?;
        let count = size / entry_size;
        let mut relocations = Vec::with_capacity(count);

        match (self.header.identification.class, addend) {
            (Class::Class32, DynamicRelocationAddend::Implicit) => {
                if entry_size < core::mem::size_of::<relocation::class_32::RelRepresentation>() {
                    return None;
                }

                for relocation_index in 0..count {
                    let offset = relocation_index.checked_mul(entry_size)?;
                    let representation =
                        read::<relocation::class_32::RelRepresentation>(bytes, offset)?;
                    relocations.push(Relocation::from(representation));
                }
            }
            (Class::Class32, DynamicRelocationAddend::Explicit) => {
                if entry_size < core::mem::size_of::<relocation::class_32::RelaRepresentation>() {
                    return None;
                }

                for relocation_index in 0..count {
                    let offset = relocation_index.checked_mul(entry_size)?;
                    let representation =
                        read::<relocation::class_32::RelaRepresentation>(bytes, offset)?;
                    relocations.push(Relocation::from(representation));
                }
            }
            (Class::Class64, DynamicRelocationAddend::Implicit) => {
                if entry_size < core::mem::size_of::<relocation::class_64::RelRepresentation>() {
                    return None;
                }

                for relocation_index in 0..count {
                    let offset = relocation_index.checked_mul(entry_size)?;
                    let representation =
                        read::<relocation::class_64::RelRepresentation>(bytes, offset)?;
                    relocations.push(Relocation::from(representation));
                }
            }
            (Class::Class64, DynamicRelocationAddend::Explicit) => {
                if entry_size < core::mem::size_of::<relocation::class_64::RelaRepresentation>() {
                    return None;
                }

                for relocation_index in 0..count {
                    let offset = relocation_index.checked_mul(entry_size)?;
                    let representation =
                        read::<relocation::class_64::RelaRepresentation>(bytes, offset)?;
                    relocations.push(Relocation::from(representation));
                }
            }
            (Class::None | Class::Reserved(_), _) => return None,
        }

        Some(DynamicRelocationTable::new(
            relocations,
            self.dynamic_symbol_table_from_program_header(program_header_index)?,
            addend,
            purpose,
        ))
    }

    pub fn shared_object_dependencies_from_program_header(
        &self,
        index: usize,
    ) -> Option<SharedObjectDependencies<'file>> {
        let array = self.dynamic_array_from_program_header(index)?;
        let strings = self.dynamic_string_table_from_program_header(index)?;

        let mut needed = Vec::new();
        for entry in array.iter() {
            if matches!(entry.tag, Tag::Needed) {
                needed.push(strings.get_str(entry.payload as usize)?);
            }
        }

        let shared_object_name = array
            .first(Tag::SharedObjectName)
            .and_then(|entry| strings.get_str(entry.payload as usize));
        let runtime_search_path = array
            .first(Tag::RuntimeSearchPath)
            .and_then(|entry| strings.get_str(entry.payload as usize));
        let run_path = array
            .first(Tag::RunPath)
            .and_then(|entry| strings.get_str(entry.payload as usize));

        Some(SharedObjectDependencies::new(
            needed,
            shared_object_name,
            runtime_search_path,
            run_path,
        ))
    }
}
