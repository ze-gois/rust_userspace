//! Dynamic-linking relationships resolved through an ELF object file.

use ample::r#type::Vec;

use super::{
    parser::{parse_dynamic_array, word},
    ObjectFile,
};
use super::super::{
    dynamic::{self, Flags, Tag},
    header,
    initialization_termination::{
        FunctionAddress,
        FunctionPointer,
        Functions as InitializationAndTerminationFunctions,
        Initialization,
        PreInitialization,
        Termination,
    },
    representation::Decoder,
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
    relocation::{self, relative, Relocation},
    section_header,
    shared_object_dependencies::SharedObjectDependencies,
    string_table::StringTable,
    symbol::{self, Symbol},
};

impl<'file> ObjectFile<'file> {
    fn system_v_hash_counts(&self, address: u64) -> Option<(u32, u32)> {
        let bytes = self.file_range_for_virtual_address(address, 8)?;
        Some((
            word(bytes, 0, self.header.identification.data)?,
            word(bytes, 4, self.header.identification.data)?,
        ))
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
            self.header.identification.data,
            entry_size,
        )
    }

    pub fn validate_dynamic_array(
        &self,
        index: usize,
    ) -> Result<(), dynamic::validation::ValidationError> {
        use dynamic::validation::ValidationError;

        let Some(program_header) = self.program_headers.get(index).copied() else {
            return Ok(());
        };
        if !matches!(program_header.r#type, program_header::Type::Dynamic) {
            return Ok(());
        }

        let array = self
            .dynamic_array_from_program_header(index)
            .ok_or(ValidationError::MalformedDynamicArray)?;

        if !matches!(array.entries.last().map(|entry| entry.tag), Some(Tag::Null)) {
            return Err(ValidationError::MissingNullTerminator);
        }

        for (entry_index, entry) in array.iter().enumerate() {
            if let Tag::Reserved(raw) = entry.tag {
                return Err(ValidationError::ReservedTag {
                    index: entry_index,
                    raw,
                });
            }
        }

        let string_table = array
            .first(Tag::StringTable)
            .ok_or(ValidationError::MissingStringTable)?;
        let string_table_size = array
            .first(Tag::StringTableSize)
            .ok_or(ValidationError::MissingStringTableSize)?;
        let symbol_table = array
            .first(Tag::SymbolTable)
            .ok_or(ValidationError::MissingSymbolTable)?;
        let symbol_entry_size = array
            .first(Tag::SymbolEntrySize)
            .ok_or(ValidationError::MissingSymbolEntrySize)?;

        let _ = (string_table, string_table_size, symbol_table);

        if array.first(Tag::Hash).is_none() && array.first(Tag::SymbolTableSize).is_none() {
            return Err(ValidationError::MissingHashOrSymbolTableSize);
        }

        let expected_symbol_entry_size = match self.header.identification.class {
            Class::Class32 => core::mem::size_of::<symbol::class_32::Representation>() as u64,
            Class::Class64 => core::mem::size_of::<symbol::class_64::Representation>() as u64,
            Class::None | Class::Reserved(_) => return Ok(()),
        };

        if symbol_entry_size.payload != expected_symbol_entry_size {
            return Err(ValidationError::SymbolEntrySizeMismatch {
                expected: expected_symbol_entry_size,
                actual: symbol_entry_size.payload,
            });
        }

        if let Some(symbol_table_size) = array.first(Tag::SymbolTableSize) {
            if symbol_table_size.payload % symbol_entry_size.payload != 0 {
                return Err(ValidationError::SymbolTableSizeNotEntryMultiple);
            }
        }

        let expected_rel_entry_size = match self.header.identification.class {
            Class::Class32 => {
                core::mem::size_of::<relocation::class_32::RelRepresentation>() as u64
            }
            Class::Class64 => {
                core::mem::size_of::<relocation::class_64::RelRepresentation>() as u64
            }
            Class::None | Class::Reserved(_) => return Ok(()),
        };
        let expected_rela_entry_size = match self.header.identification.class {
            Class::Class32 => {
                core::mem::size_of::<relocation::class_32::RelaRepresentation>() as u64
            }
            Class::Class64 => {
                core::mem::size_of::<relocation::class_64::RelaRepresentation>() as u64
            }
            Class::None | Class::Reserved(_) => return Ok(()),
        };

        let rel = array.first(Tag::Relocation);
        let rela = array.first(Tag::RelocationWithAddend);

        if matches!(self.header.r#type, header::Type::Executable) && rel.is_none() && rela.is_none() {
            return Err(ValidationError::ExecutableMissingRelocationTable);
        }

        if rel.is_some() {
            let size = array
                .first(Tag::RelocationSize)
                .ok_or(ValidationError::RelocationMissingSize)?;
            let entry_size = array
                .first(Tag::RelocationEntrySize)
                .ok_or(ValidationError::RelocationMissingEntrySize)?;

            if entry_size.payload != expected_rel_entry_size {
                return Err(ValidationError::RelocationEntrySizeMismatch {
                    expected: expected_rel_entry_size,
                    actual: entry_size.payload,
                });
            }
            if size.payload % entry_size.payload != 0 {
                return Err(ValidationError::RelocationSizeNotEntryMultiple);
            }
        }

        if rela.is_some() {
            let size = array
                .first(Tag::RelocationWithAddendSize)
                .ok_or(ValidationError::RelocationWithAddendMissingSize)?;
            let entry_size = array
                .first(Tag::RelocationWithAddendEntrySize)
                .ok_or(ValidationError::RelocationWithAddendMissingEntrySize)?;

            if entry_size.payload != expected_rela_entry_size {
                return Err(ValidationError::RelocationWithAddendEntrySizeMismatch {
                    expected: expected_rela_entry_size,
                    actual: entry_size.payload,
                });
            }
            if size.payload % entry_size.payload != 0 {
                return Err(ValidationError::RelocationWithAddendSizeNotEntryMultiple);
            }
        }

        if array.first(Tag::JumpRelocation).is_some() {
            let size = array
                .first(Tag::ProcedureLinkageTableRelocationSize)
                .ok_or(ValidationError::JumpRelocationMissingSize)?;
            let format = array
                .first(Tag::ProcedureLinkageTableRelocation)
                .ok_or(ValidationError::JumpRelocationMissingFormat)?;

            let entry_size = match Tag::from_raw(
                i64::try_from(format.payload).unwrap_or(i64::MIN),
            ) {
                Tag::Relocation => expected_rel_entry_size,
                Tag::RelocationWithAddend => expected_rela_entry_size,
                _ => {
                    return Err(ValidationError::JumpRelocationInvalidFormat {
                        raw: format.payload,
                    })
                }
            };

            if size.payload % entry_size != 0 {
                return Err(ValidationError::JumpRelocationSizeNotEntryMultiple);
            }
        }

        if let Some(flags) = array.first(Tag::Flags) {
            let reserved = flags.payload & !Flags::DEFINED_MASK;
            if reserved != 0 {
                return Err(ValidationError::ReservedFlags { bits: reserved });
            }
        }

        self.validate_dynamic_relative_relocation(index)?;
        self.validate_dynamic_initialization_and_termination(index)?;

        Ok(())
    }

    pub fn dynamic_flags_from_program_header(&self, index: usize) -> Option<Flags> {
        let array = self.dynamic_array_from_program_header(index)?;
        Some(Flags::from_raw(
            array.first(Tag::Flags).map_or(0, |entry| entry.payload),
        ))
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

        let symbol_address = array.first(Tag::SymbolTable)?.payload;
        let entry_size = usize::try_from(array.first(Tag::SymbolEntrySize)?.payload).ok()?;
        if entry_size == 0 {
            return None;
        }

        let byte_size = if let Some(size) = array.first(Tag::SymbolTableSize) {
            usize::try_from(size.payload).ok()?
        } else {
            let hash_address = array.first(Tag::Hash)?.payload;
            let (_, chain_count) = self.system_v_hash_counts(hash_address)?;
            usize::try_from(chain_count).ok()?.checked_mul(entry_size)?
        };

        if byte_size % entry_size != 0 {
            return None;
        }
        let count = byte_size / entry_size;
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
                    let representation = symbol::class_32::Representation::decode(
                        symbol_bytes,
                        offset,
                        self.header.identification.data,
                    )?;
                    symbols.push(Symbol::from(representation));
                }
            }
            Class::Class64 => {
                if entry_size < core::mem::size_of::<symbol::class_64::Representation>() {
                    return None;
                }

                for symbol_index in 0..count {
                    let offset = symbol_index.checked_mul(entry_size)?;
                    let representation = symbol::class_64::Representation::decode(
                        symbol_bytes,
                        offset,
                        self.header.identification.data,
                    )?;
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
                Some(word(bytes, offset, self.header.identification.data)?)
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

    pub fn validate_dynamic_linking_tables(
        &self,
        index: usize,
    ) -> Result<(), dynamic::validation::ValidationError> {
        use dynamic::validation::ValidationError;

        let array = self
            .dynamic_array_from_program_header(index)
            .ok_or(ValidationError::MalformedDynamicArray)?;

        let strings = self
            .dynamic_string_table_from_program_header(index)
            .ok_or(ValidationError::DynamicStringTableUnavailable)?;
        strings
            .validate()
            .map_err(ValidationError::DynamicStringTableInvalid)?;

        for (entry_index, entry) in array.iter().enumerate() {
            let string_is_meaningful = match entry.tag {
                Tag::Needed | Tag::RunPath => true,
                Tag::SharedObjectName => matches!(self.header.r#type, header::Type::SharedObject),
                Tag::RuntimeSearchPath => matches!(self.header.r#type, header::Type::Executable),
                _ => false,
            };

            if string_is_meaningful && strings.get(entry.payload as usize).is_none() {
                return Err(ValidationError::InvalidStringOffset {
                    index: entry_index,
                    tag: entry.tag,
                    offset: entry.payload,
                });
            }
        }

        let symbols = self
            .dynamic_symbol_table_from_program_header(index)
            .ok_or(ValidationError::DynamicSymbolTableUnavailable)?;
        symbols
            .validate()
            .map_err(ValidationError::DynamicSymbolTableInvalid)?;

        if array.first(Tag::Hash).is_some() {
            let hash = self
                .dynamic_hash_table_from_program_header(index)
                .ok_or(ValidationError::DynamicHashTableUnavailable)?;
            hash.table
                .validate(symbols.len())
                .map_err(ValidationError::DynamicHashTableInvalid)?;
        }

        Ok(())
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
            buckets.push(word(bytes, offset, self.header.identification.data)?);
            offset = offset.checked_add(core::mem::size_of::<u32>())?;
        }

        let mut chains = Vec::with_capacity(usize::try_from(chain_count).ok()?);
        for _ in 0..chain_count {
            chains.push(word(bytes, offset, self.header.identification.data)?);
            offset = offset.checked_add(core::mem::size_of::<u32>())?;
        }

        Some(DynamicHashTable::new(
            HashTable::new(buckets, chains),
            self.dynamic_symbol_table_from_program_header(index)?,
        ))
    }

    pub fn relative_relocation_table_from_program_header(
        &self,
        index: usize,
    ) -> Option<relative::Table> {
        self.validate_dynamic_relative_relocation(index).ok()?;

        let array = self.dynamic_array_from_program_header(index)?;
        let address = array.first(Tag::RelativeRelocation)?.payload;
        let size = usize::try_from(array.first(Tag::RelativeRelocationSize)?.payload).ok()?;
        let entry_size = usize::try_from(
            array.first(Tag::RelativeRelocationEntrySize)?.payload,
        ).ok()?;
        let bytes = self.file_range_for_virtual_address(address, u64::try_from(size).ok()?)?;
        let count = size / entry_size;
        let mut entries = Vec::with_capacity(count);

        match self.header.identification.class {
            Class::Class32 => {
                for entry_index in 0..count {
                    let offset = entry_index.checked_mul(entry_size)?;
                    let representation = relative::class_32::Representation::decode(
                        bytes,
                        offset,
                        self.header.identification.data,
                    )?;
                    entries.push(relative::Entry::from(representation));
                }
            }
            Class::Class64 => {
                for entry_index in 0..count {
                    let offset = entry_index.checked_mul(entry_size)?;
                    let representation = relative::class_64::Representation::decode(
                        bytes,
                        offset,
                        self.header.identification.data,
                    )?;
                    entries.push(relative::Entry::from(representation));
                }
            }
            Class::None | Class::Reserved(_) => return None,
        }

        Some(relative::Table::new(entries))
    }

    pub fn validate_dynamic_relative_relocation(
        &self,
        index: usize,
    ) -> Result<(), dynamic::validation::ValidationError> {
        use dynamic::validation::ValidationError;

        let Some(array) = self.dynamic_array_from_program_header(index) else {
            return Ok(());
        };
        let Some(_) = array.first(Tag::RelativeRelocation) else {
            return Ok(());
        };

        let size = array
            .first(Tag::RelativeRelocationSize)
            .ok_or(ValidationError::RelativeRelocationMissingSize)?
            .payload;
        let entry_size = array
            .first(Tag::RelativeRelocationEntrySize)
            .ok_or(ValidationError::RelativeRelocationMissingEntrySize)?
            .payload;

        let expected_entry_size = match self.header.identification.class {
            Class::Class32 => 4u64,
            Class::Class64 => 8u64,
            Class::None | Class::Reserved(_) => return Ok(()),
        };

        if entry_size != expected_entry_size {
            return Err(ValidationError::RelativeRelocationEntrySizeMismatch);
        }
        if size % entry_size != 0 {
            return Err(ValidationError::RelativeRelocationSizeNotEntryMultiple);
        }

        if size != 0 {
            let address = array
                .first(Tag::RelativeRelocation)
                .expect("DT_RELR presence checked above")
                .payload;
            let bytes = self
                .file_range_for_virtual_address(address, size)
                .ok_or(ValidationError::RelativeRelocationTableUnavailable)?;

            let first = match self.header.identification.class {
                Class::Class32 => {
                    let representation = relative::class_32::Representation::decode(
                        bytes,
                        0,
                        self.header.identification.data,
                    )
                    .ok_or(ValidationError::RelativeRelocationTableUnavailable)?;
                    relative::Entry::from(representation)
                }
                Class::Class64 => {
                    let representation = relative::class_64::Representation::decode(
                        bytes,
                        0,
                        self.header.identification.data,
                    )
                    .ok_or(ValidationError::RelativeRelocationTableUnavailable)?;
                    relative::Entry::from(representation)
                }
                Class::None | Class::Reserved(_) => return Ok(()),
            };

            if !matches!(first, relative::Entry::Address(_)) {
                return Err(ValidationError::RelativeRelocationFirstEntryMustBeAddress);
            }
        }

        Ok(())
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
            let (addend, entry_size) = match (
                self.header.identification.class,
                Tag::from_raw(i64::try_from(format).ok()?),
            ) {
                (Class::Class32, Tag::Relocation) => (
                    DynamicRelocationAddend::Implicit,
                    core::mem::size_of::<relocation::class_32::RelRepresentation>() as u64,
                ),
                (Class::Class32, Tag::RelocationWithAddend) => (
                    DynamicRelocationAddend::Explicit,
                    core::mem::size_of::<relocation::class_32::RelaRepresentation>() as u64,
                ),
                (Class::Class64, Tag::Relocation) => (
                    DynamicRelocationAddend::Implicit,
                    core::mem::size_of::<relocation::class_64::RelRepresentation>() as u64,
                ),
                (Class::Class64, Tag::RelocationWithAddend) => (
                    DynamicRelocationAddend::Explicit,
                    core::mem::size_of::<relocation::class_64::RelaRepresentation>() as u64,
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

    pub fn validate_dynamic_relocation_tables(
        &self,
        index: usize,
    ) -> Result<(), dynamic::validation::ValidationError> {
        use dynamic::validation::ValidationError;

        self.validate_dynamic_array(index)?;
        self.validate_dynamic_linking_tables(index)?;

        let tables = self
            .dynamic_relocation_tables_from_program_header(index)
            .ok_or(ValidationError::DynamicRelocationTablesUnavailable)?;

        for (table_index, table) in tables.iter().enumerate() {
            for (entry_index, relocation) in table.relocations.iter().enumerate() {
                if relocation.symbol_index as usize >= table.symbols.len() {
                    return Err(
                        ValidationError::DynamicRelocationSymbolIndexOutOfBounds {
                            table_index,
                            entry_index,
                            symbol_index: relocation.symbol_index,
                        },
                    );
                }
            }
        }

        Ok(())
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
                    let representation = relocation::class_32::RelRepresentation::decode(
                        bytes,
                        offset,
                        self.header.identification.data,
                    )?;
                    relocations.push(Relocation::from(representation));
                }
            }
            (Class::Class32, DynamicRelocationAddend::Explicit) => {
                if entry_size < core::mem::size_of::<relocation::class_32::RelaRepresentation>() {
                    return None;
                }

                for relocation_index in 0..count {
                    let offset = relocation_index.checked_mul(entry_size)?;
                    let representation = relocation::class_32::RelaRepresentation::decode(
                        bytes,
                        offset,
                        self.header.identification.data,
                    )?;
                    relocations.push(Relocation::from(representation));
                }
            }
            (Class::Class64, DynamicRelocationAddend::Implicit) => {
                if entry_size < core::mem::size_of::<relocation::class_64::RelRepresentation>() {
                    return None;
                }

                for relocation_index in 0..count {
                    let offset = relocation_index.checked_mul(entry_size)?;
                    let representation = relocation::class_64::RelRepresentation::decode(
                        bytes,
                        offset,
                        self.header.identification.data,
                    )?;
                    relocations.push(Relocation::from(representation));
                }
            }
            (Class::Class64, DynamicRelocationAddend::Explicit) => {
                if entry_size < core::mem::size_of::<relocation::class_64::RelaRepresentation>() {
                    return None;
                }

                for relocation_index in 0..count {
                    let offset = relocation_index.checked_mul(entry_size)?;
                    let representation = relocation::class_64::RelaRepresentation::decode(
                        bytes,
                        offset,
                        self.header.identification.data,
                    )?;
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

    pub fn initialization_and_termination_functions_from_program_header(
        &self,
        index: usize,
    ) -> Option<InitializationAndTerminationFunctions> {
        let array = self.dynamic_array_from_program_header(index)?;

        let initialization = Initialization::new(
            array
                .first(Tag::Initialization)
                .map(|entry| FunctionAddress::new(entry.payload)),
            self.dynamic_function_pointer_array(
                array.first(Tag::InitializationArray).map(|entry| entry.payload),
                array
                    .first(Tag::InitializationArraySize)
                    .map(|entry| entry.payload),
            )?,
        );

        let pre_initialization = match (
            array
                .first(Tag::PreInitializationArray)
                .map(|entry| entry.payload),
            array
                .first(Tag::PreInitializationArraySize)
                .map(|entry| entry.payload),
        ) {
            (None, _) => None,
            (Some(address), size) => Some(PreInitialization::new(
                self.dynamic_function_pointer_array(Some(address), size)?,
            )),
        };

        let termination = Termination::new(
            self.dynamic_function_pointer_array(
                array.first(Tag::TerminationArray).map(|entry| entry.payload),
                array
                    .first(Tag::TerminationArraySize)
                    .map(|entry| entry.payload),
            )?,
            array
                .first(Tag::Termination)
                .map(|entry| FunctionAddress::new(entry.payload)),
        );

        Some(InitializationAndTerminationFunctions::new(
            pre_initialization,
            initialization,
            termination,
        ))
    }

    pub fn validate_dynamic_initialization_and_termination(
        &self,
        index: usize,
    ) -> Result<(), dynamic::validation::ValidationError> {
        use dynamic::validation::ValidationError;

        let Some(array) = self.dynamic_array_from_program_header(index) else {
            return Ok(());
        };

        let pointer_size = match self.header.identification.class {
            Class::Class32 => 4u64,
            Class::Class64 => 8u64,
            Class::None | Class::Reserved(_) => return Ok(()),
        };

        let initialization_array = array.first(Tag::InitializationArray);
        let initialization_size = array.first(Tag::InitializationArraySize);
        match (initialization_array, initialization_size) {
            (Some(_), None) => return Err(ValidationError::InitializationArrayMissingSize),
            (Some(_), Some(size)) if size.payload % pointer_size != 0 => {
                return Err(ValidationError::InitializationArraySizeNotPointerMultiple)
            }
            _ => {}
        }

        let termination_array = array.first(Tag::TerminationArray);
        let termination_size = array.first(Tag::TerminationArraySize);
        match (termination_array, termination_size) {
            (Some(_), None) => return Err(ValidationError::TerminationArrayMissingSize),
            (Some(_), Some(size)) if size.payload % pointer_size != 0 => {
                return Err(ValidationError::TerminationArraySizeNotPointerMultiple)
            }
            _ => {}
        }

        let pre_initialization_array = array.first(Tag::PreInitializationArray);
        let pre_initialization_size = array.first(Tag::PreInitializationArraySize);
        match (pre_initialization_array, pre_initialization_size) {
            (Some(_), None) => {
                return Err(ValidationError::PreInitializationArrayMissingSize)
            }
            (Some(_), Some(size)) if size.payload % pointer_size != 0 => {
                return Err(ValidationError::PreInitializationArraySizeNotPointerMultiple)
            }
            _ => {}
        }

        if matches!(self.header.r#type, header::Type::SharedObject)
            && pre_initialization_array.is_some()
        {
            return Err(ValidationError::PreInitializationInSharedObject);
        }

        Ok(())
    }

    fn dynamic_function_pointer_array(
        &self,
        address: Option<u64>,
        size: Option<u64>,
    ) -> Option<Vec<FunctionPointer>> {
        match (address, size) {
            (None, _) => Some(Vec::new()),
            (Some(address), Some(size)) => {
                let pointer_size = match self.header.identification.class {
                    Class::Class32 => 4usize,
                    Class::Class64 => 8usize,
                    Class::None | Class::Reserved(_) => return None,
                };
                let size = usize::try_from(size).ok()?;
                if size % pointer_size != 0 {
                    return None;
                }

                let bytes = self.file_range_for_virtual_address(
                    address,
                    u64::try_from(size).ok()?,
                )?;
                let mut pointers = Vec::with_capacity(size / pointer_size);

                for pointer_index in 0..(size / pointer_size) {
                    let offset = pointer_index.checked_mul(pointer_size)?;
                    let mut decoder = Decoder::new(
                        bytes,
                        offset,
                        self.header.identification.data,
                    )?;
                    let value = match self.header.identification.class {
                        Class::Class32 => u64::from(decoder.word()?),
                        Class::Class64 => decoder.xword()?,
                        Class::None | Class::Reserved(_) => return None,
                    };
                    pointers.push(FunctionPointer::new(value));
                }

                Some(pointers)
            }
            _ => None,
        }
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
