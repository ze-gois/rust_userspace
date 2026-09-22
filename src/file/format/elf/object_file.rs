//! Parsed ELF object file.

use ample::r#type::Vec;

use super::{
    compression::{self, CompressedSection, CompressionHeader},
    dynamic::{self, Dynamic, Tag},
    dynamic_array::DynamicArray,
    dynamic_table::DynamicTable,
    hash::HashTable,
    header::{self, Header},
    loadable_segment::LoadableSegment,
    note::Note,
    note_table::NoteTable,
    identification::{Class, Data, Identification},
    program_header::{self, ProgramHeader},
    program_header_table_image::ProgramHeaderTableImage,
    program_interpreter::ProgramInterpreter,
    relocation::{self, Relocation},
    relocation_table::RelocationTable,
    section_group::{Flags as SectionGroupFlags, SectionGroup},
    section_header::{self, SectionHeader},
    string_table::StringTable,
    symbol::{self, Symbol},
    symbol_table::SymbolTable,
    thread_local_storage,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseError {
    Truncated,
    InvalidIdentification,
    UnsupportedDataEncoding,
    UnsupportedClass,
    ProgramHeaderEntryTooSmall,
    SectionHeaderEntryTooSmall,
    InvalidHeader,
    InvalidSectionHeaderTable,
    InvalidSectionNameStringTable,
}

#[derive(Debug)]
pub struct ObjectFile<'file> {
    bytes: &'file [u8],
    pub header: Header,
    pub program_headers: Vec<ProgramHeader>,
    pub section_headers: Vec<SectionHeader>,
    pub section_header_count: usize,
    pub section_name_string_table_index: Option<usize>,
}

impl<'file> ObjectFile<'file> {
    pub fn parse(bytes: &'file [u8]) -> Result<Self, ParseError> {
        let identification_bytes: [u8; super::identification::SIZE] = bytes
            .get(..super::identification::SIZE)
            .ok_or(ParseError::Truncated)?
            .try_into()
            .map_err(|_| ParseError::Truncated)?;

        let identification = Identification::from_bytes(identification_bytes)
            .ok_or(ParseError::InvalidIdentification)?;

        if !native_data_encoding(identification.data) {
            return Err(ParseError::UnsupportedDataEncoding);
        }

        match identification.class {
            Class::Class32 => Self::parse_class_32(bytes),
            Class::Class64 => Self::parse_class_64(bytes),
            Class::None | Class::Reserved(_) => Err(ParseError::UnsupportedClass),
        }
    }

    fn parse_class_32(bytes: &'file [u8]) -> Result<Self, ParseError> {
        let representation: header::class_32::Representation =
            read(bytes, 0).ok_or(ParseError::Truncated)?;
        let header = Header::try_from(representation).map_err(|_| ParseError::InvalidHeader)?;

        let initial_section_header = initial_section_header_32(bytes, &header)?;
        let section_header_count =
            resolve_section_header_count(&header, initial_section_header)?;
        let section_name_string_table_index =
            resolve_section_name_string_table_index(&header, initial_section_header)?;

        let program_headers = parse_table(
            bytes,
            header.program_header_offset,
            header.program_header_entry_size,
            header.program_header_count as usize,
            core::mem::size_of::<program_header::class_32::Representation>(),
            |bytes, offset| {
                read::<program_header::class_32::Representation>(bytes, offset)
                    .map(ProgramHeader::from)
            },
            ParseError::ProgramHeaderEntryTooSmall,
        )?;

        let section_headers = parse_table(
            bytes,
            header.section_header_offset,
            header.section_header_entry_size,
            section_header_count,
            core::mem::size_of::<section_header::class_32::Representation>(),
            |bytes, offset| {
                read::<section_header::class_32::Representation>(bytes, offset)
                    .map(SectionHeader::from)
            },
            ParseError::SectionHeaderEntryTooSmall,
        )?;

        if let Some(index) = section_name_string_table_index {
            if index >= section_headers.len() {
                return Err(ParseError::InvalidSectionNameStringTable);
            }
        }

        Ok(Self {
            bytes,
            header,
            program_headers,
            section_headers,
            section_header_count,
            section_name_string_table_index,
        })
    }

    fn parse_class_64(bytes: &'file [u8]) -> Result<Self, ParseError> {
        let representation: header::class_64::Representation =
            read(bytes, 0).ok_or(ParseError::Truncated)?;
        let header = Header::try_from(representation).map_err(|_| ParseError::InvalidHeader)?;

        let initial_section_header = initial_section_header_64(bytes, &header)?;
        let section_header_count =
            resolve_section_header_count(&header, initial_section_header)?;
        let section_name_string_table_index =
            resolve_section_name_string_table_index(&header, initial_section_header)?;

        let program_headers = parse_table(
            bytes,
            header.program_header_offset,
            header.program_header_entry_size,
            header.program_header_count as usize,
            core::mem::size_of::<program_header::class_64::Representation>(),
            |bytes, offset| {
                read::<program_header::class_64::Representation>(bytes, offset)
                    .map(ProgramHeader::from)
            },
            ParseError::ProgramHeaderEntryTooSmall,
        )?;

        let section_headers = parse_table(
            bytes,
            header.section_header_offset,
            header.section_header_entry_size,
            section_header_count,
            core::mem::size_of::<section_header::class_64::Representation>(),
            |bytes, offset| {
                read::<section_header::class_64::Representation>(bytes, offset)
                    .map(SectionHeader::from)
            },
            ParseError::SectionHeaderEntryTooSmall,
        )?;

        if let Some(index) = section_name_string_table_index {
            if index >= section_headers.len() {
                return Err(ParseError::InvalidSectionNameStringTable);
            }
        }

        Ok(Self {
            bytes,
            header,
            program_headers,
            section_headers,
            section_header_count,
            section_name_string_table_index,
        })
    }

    pub const fn bytes(&self) -> &'file [u8] {
        self.bytes
    }

    pub fn section(&self, index: usize) -> Option<super::section::Section<'file>> {
        let header = *self.section_headers.get(index)?;
        let contents = if matches!(header.r#type, section_header::Type::NoBits) {
            &[][..]
        } else {
            range(self.bytes, header.offset, header.size)?
        };

        Some(super::section::Section::new(header, contents))
    }

    pub fn segment(&self, index: usize) -> Option<super::segment::Segment<'file>> {
        let program_header = *self.program_headers.get(index)?;
        let file_image = range(
            self.bytes,
            program_header.offset,
            program_header.file_size,
        )?;

        Some(super::segment::Segment::new(program_header, file_image))
    }

    pub fn loadable_segment(&self, index: usize) -> Option<LoadableSegment<'file>> {
        let program_header = *self.program_headers.get(index)?;
        if !matches!(program_header.r#type, program_header::Type::Load) {
            return None;
        }

        if program_header.file_size > program_header.memory_size {
            return None;
        }

        let file_image = range(self.bytes, program_header.offset, program_header.file_size)?;
        Some(LoadableSegment::new(program_header, file_image))
    }

    pub fn program_interpreter(&self) -> Option<ProgramInterpreter<'file>> {
        let (index, program_header) = self
            .program_headers
            .iter()
            .enumerate()
            .find(|(_, header)| matches!(header.r#type, program_header::Type::Interpreter))?;
        let segment = self.segment(index)?;
        let pathname = core::ffi::CStr::from_bytes_with_nul(segment.file_image).ok()?;
        Some(ProgramInterpreter::new(*program_header, pathname))
    }

    pub fn program_header_table_image(&self) -> Option<ProgramHeaderTableImage<'file>> {
        let (index, program_header) = self
            .program_headers
            .iter()
            .enumerate()
            .find(|(_, header)| matches!(header.r#type, program_header::Type::ProgramHeader))?;
        let segment = self.segment(index)?;
        Some(ProgramHeaderTableImage::new(
            *program_header,
            segment.file_image,
        ))
    }

    pub fn thread_local_storage_template(
        &self,
    ) -> Option<thread_local_storage::Template<'file>> {
        let (index, program_header) = self
            .program_headers
            .iter()
            .enumerate()
            .find(|(_, header)| matches!(header.r#type, program_header::Type::ThreadLocalStorage))?;

        if program_header.file_size > program_header.memory_size {
            return None;
        }

        let segment = self.segment(index)?;
        Some(thread_local_storage::Template::new(
            *program_header,
            segment.file_image,
        ))
    }

    pub fn validate_program_headers(
        &self,
    ) -> Result<(), program_header::ValidationError> {
        use program_header::{Type, ValidationError};

        let mut first_load_seen = false;
        let mut interpreter_seen = false;
        let mut program_header_table_image_seen = false;
        let mut previous_load: Option<(usize, u64)> = None;

        for (index, header) in self.program_headers.iter().enumerate() {
            match header.r#type {
                Type::Load => {
                    first_load_seen = true;

                    if header.file_size > header.memory_size {
                        return Err(ValidationError::LoadFileImageLargerThanMemoryImage {
                            index,
                        });
                    }

                    if header.alignment > 1 {
                        if !header.alignment.is_power_of_two() {
                            return Err(ValidationError::LoadAlignmentNotPowerOfTwo { index });
                        }

                        if header.virtual_address % header.alignment
                            != header.offset % header.alignment
                        {
                            return Err(ValidationError::LoadAddressOffsetIncongruent {
                                index,
                            });
                        }
                    }

                    if let Some((previous_index, previous_virtual_address)) = previous_load {
                        if header.virtual_address < previous_virtual_address {
                            return Err(
                                ValidationError::LoadSegmentsNotOrderedByVirtualAddress {
                                    previous: previous_index,
                                    current: index,
                                },
                            );
                        }
                    }

                    previous_load = Some((index, header.virtual_address));
                }
                Type::Interpreter => {
                    if interpreter_seen {
                        return Err(ValidationError::MultipleInterpreters);
                    }
                    if first_load_seen {
                        return Err(ValidationError::InterpreterAfterLoad { index });
                    }
                    interpreter_seen = true;
                }
                Type::ProgramHeader => {
                    if program_header_table_image_seen {
                        return Err(ValidationError::MultipleProgramHeaderTableImages);
                    }
                    if first_load_seen {
                        return Err(ValidationError::ProgramHeaderTableImageAfterLoad { index });
                    }
                    program_header_table_image_seen = true;
                }
                Type::SharedLibrary => {
                    return Err(ValidationError::SharedLibrarySegment { index });
                }
                _ => {}
            }
        }

        Ok(())
    }

    pub fn file_offset_for_virtual_address(&self, address: u64) -> Option<u64> {
        for header in &self.program_headers {
            if !matches!(header.r#type, program_header::Type::Load) {
                continue;
            }

            let end = header.virtual_address.checked_add(header.file_size)?;
            if address < header.virtual_address || address >= end {
                continue;
            }

            let displacement = address.checked_sub(header.virtual_address)?;
            return header.offset.checked_add(displacement);
        }

        None
    }

    pub fn file_range_for_virtual_address(
        &self,
        address: u64,
        size: u64,
    ) -> Option<&'file [u8]> {
        let offset = self.file_offset_for_virtual_address(address)?;
        range(self.bytes, offset, size)
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

    pub fn note_table_from_program_header(&self, index: usize) -> Option<NoteTable<'file>> {
        let program_header = *self.program_headers.get(index)?;
        if !matches!(program_header.r#type, program_header::Type::Note) {
            return None;
        }

        let segment = self.segment(index)?;
        parse_note_table(segment.file_image, self.header.identification.class)
    }

    pub fn section_name_string_table(&self) -> Option<StringTable<'file>> {
        let index = self.section_name_string_table_index?;
        let section = self.section(index)?;
        Some(StringTable::new(section.contents))
    }

    pub fn section_name(&self, index: usize) -> Option<&'file str> {
        let header = self.section_headers.get(index)?;
        self.section_name_string_table()?
            .get_str(header.name_index as usize)
    }

    pub fn symbol_table(&self, section_index: usize) -> Option<SymbolTable<'file>> {
        let header = *self.section_headers.get(section_index)?;

        if !matches!(
            header.r#type,
            section_header::Type::SymbolTable | section_header::Type::DynamicSymbolTable
        ) {
            return None;
        }

        let section = self.section(section_index)?;
        let strings_section = self.section(header.link as usize)?;

        if !matches!(strings_section.header.r#type, section_header::Type::StringTable) {
            return None;
        }

        let strings = StringTable::new(strings_section.contents);
        let entry_size = usize::try_from(header.entry_size).ok()?;

        if entry_size == 0 || section.contents.len() % entry_size != 0 {
            return None;
        }

        let count = section.contents.len() / entry_size;
        let mut symbols = Vec::with_capacity(count);

        match self.header.identification.class {
            Class::Class32 => {
                if entry_size < core::mem::size_of::<symbol::class_32::Representation>() {
                    return None;
                }

                for index in 0..count {
                    let offset = index.checked_mul(entry_size)?;
                    let representation =
                        read::<symbol::class_32::Representation>(section.contents, offset)?;
                    symbols.push(Symbol::from(representation));
                }
            }
            Class::Class64 => {
                if entry_size < core::mem::size_of::<symbol::class_64::Representation>() {
                    return None;
                }

                for index in 0..count {
                    let offset = index.checked_mul(entry_size)?;
                    let representation =
                        read::<symbol::class_64::Representation>(section.contents, offset)?;
                    symbols.push(Symbol::from(representation));
                }
            }
            Class::None | Class::Reserved(_) => return None,
        }

        let extended_section_indices = self
            .section_headers
            .iter()
            .enumerate()
            .find(|(_, candidate)| {
                matches!(
                    candidate.r#type,
                    section_header::Type::SymbolTableSectionIndex
                ) && candidate.link as usize == section_index
            })
            .and_then(|(index, _)| self.section(index));

        let mut section_indices = Vec::with_capacity(count);

        if let Some(extended_section_indices) = extended_section_indices {
            let word_size = core::mem::size_of::<u32>();
            if extended_section_indices.contents.len() != count.checked_mul(word_size)? {
                return None;
            }

            for (index, symbol) in symbols.iter().enumerate() {
                let offset = index.checked_mul(word_size)?;
                let extended = read::<u32>(extended_section_indices.contents, offset)?;

                if symbol.section_index != section_header::Index::EXTENDED && extended != 0 {
                    return None;
                }

                section_indices.push(symbol::ResolvedSectionIndex::resolve(
                    symbol.section_index,
                    Some(extended),
                )?);
            }
        } else {
            for symbol in &symbols {
                section_indices.push(symbol::ResolvedSectionIndex::resolve(
                    symbol.section_index,
                    None,
                )?);
            }
        }

        Some(SymbolTable::new(
            symbols,
            strings,
            section_indices,
            header.information as usize,
        ))
    }

    pub fn relocation_table(&self, section_index: usize) -> Option<RelocationTable<'file>> {
        let header = *self.section_headers.get(section_index)?;
        let with_addend = match header.r#type {
            section_header::Type::Relocation => false,
            section_header::Type::RelocationWithAddend => true,
            _ => return None,
        };

        let section = self.section(section_index)?;
        let symbols = self.symbol_table(header.link as usize)?;
        let entry_size = usize::try_from(header.entry_size).ok()?;

        if entry_size == 0 || section.contents.len() % entry_size != 0 {
            return None;
        }

        let count = section.contents.len() / entry_size;
        let mut relocations = Vec::with_capacity(count);

        match (self.header.identification.class, with_addend) {
            (Class::Class32, false) => {
                if entry_size < core::mem::size_of::<relocation::class_32::RelRepresentation>() {
                    return None;
                }
                for index in 0..count {
                    let offset = index.checked_mul(entry_size)?;
                    let representation =
                        read::<relocation::class_32::RelRepresentation>(section.contents, offset)?;
                    relocations.push(Relocation::from(representation));
                }
            }
            (Class::Class32, true) => {
                if entry_size < core::mem::size_of::<relocation::class_32::RelaRepresentation>() {
                    return None;
                }
                for index in 0..count {
                    let offset = index.checked_mul(entry_size)?;
                    let representation =
                        read::<relocation::class_32::RelaRepresentation>(section.contents, offset)?;
                    relocations.push(Relocation::from(representation));
                }
            }
            (Class::Class64, false) => {
                if entry_size < core::mem::size_of::<relocation::class_64::RelRepresentation>() {
                    return None;
                }
                for index in 0..count {
                    let offset = index.checked_mul(entry_size)?;
                    let representation =
                        read::<relocation::class_64::RelRepresentation>(section.contents, offset)?;
                    relocations.push(Relocation::from(representation));
                }
            }
            (Class::Class64, true) => {
                if entry_size < core::mem::size_of::<relocation::class_64::RelaRepresentation>() {
                    return None;
                }
                for index in 0..count {
                    let offset = index.checked_mul(entry_size)?;
                    let representation =
                        read::<relocation::class_64::RelaRepresentation>(section.contents, offset)?;
                    relocations.push(Relocation::from(representation));
                }
            }
            (Class::None | Class::Reserved(_), _) => return None,
        }

        Some(RelocationTable::new(
            relocations,
            symbols,
            header.information as usize,
        ))
    }

    pub fn dynamic_table(&self, section_index: usize) -> Option<DynamicTable<'file>> {
        let header = *self.section_headers.get(section_index)?;
        if !matches!(header.r#type, section_header::Type::Dynamic) {
            return None;
        }

        let section = self.section(section_index)?;
        let strings_section = self.section(header.link as usize)?;
        if !matches!(strings_section.header.r#type, section_header::Type::StringTable) {
            return None;
        }

        let entry_size = usize::try_from(header.entry_size).ok()?;
        if entry_size == 0 || section.contents.len() % entry_size != 0 {
            return None;
        }

        let array = parse_dynamic_array(
            section.contents,
            self.header.identification.class,
            entry_size,
        )?;

        Some(DynamicTable::new(
            array,
            StringTable::new(strings_section.contents),
        ))
    }

    pub fn hash_table(&self, section_index: usize) -> Option<HashTable<'file>> {
        let header = *self.section_headers.get(section_index)?;
        if !matches!(header.r#type, section_header::Type::Hash) {
            return None;
        }

        let section = self.section(section_index)?;
        if section.contents.len() < 8 {
            return None;
        }

        let bucket_count = read::<u32>(section.contents, 0)? as usize;
        let chain_count = read::<u32>(section.contents, 4)? as usize;
        let mut offset = 8usize;

        let mut buckets = Vec::with_capacity(bucket_count);
        for _ in 0..bucket_count {
            buckets.push(read::<u32>(section.contents, offset)?);
            offset = offset.checked_add(core::mem::size_of::<u32>())?;
        }

        let mut chains = Vec::with_capacity(chain_count);
        for _ in 0..chain_count {
            chains.push(read::<u32>(section.contents, offset)?);
            offset = offset.checked_add(core::mem::size_of::<u32>())?;
        }

        let symbols = self.symbol_table(header.link as usize)?;
        Some(HashTable::new(buckets, chains, symbols))
    }

    pub fn section_group(&self, section_index: usize) -> Option<SectionGroup<'file>> {
        let header = *self.section_headers.get(section_index)?;
        if !matches!(header.r#type, section_header::Type::Group) {
            return None;
        }

        let section = self.section(section_index)?;
        if section.contents.len() < core::mem::size_of::<u32>()
            || section.contents.len() % core::mem::size_of::<u32>() != 0
        {
            return None;
        }

        let flags = SectionGroupFlags::from_raw(read::<u32>(section.contents, 0)?);
        let count = section.contents.len() / core::mem::size_of::<u32>();
        let mut members = Vec::with_capacity(count.saturating_sub(1));

        for index in 1..count {
            let offset = index.checked_mul(core::mem::size_of::<u32>())?;
            members.push(read::<u32>(section.contents, offset)? as usize);
        }

        let symbols = self.symbol_table(header.link as usize)?;
        Some(SectionGroup::new(
            flags,
            members,
            symbols,
            header.information as usize,
        ))
    }

    pub fn note_table(&self, section_index: usize) -> Option<NoteTable<'file>> {
        let section = self.section(section_index)?;
        if !matches!(section.header.r#type, section_header::Type::Note) {
            return None;
        }

        parse_note_table(section.contents, self.header.identification.class)
    }

    pub fn compressed_section(&self, section_index: usize) -> Option<CompressedSection<'file>> {
        let section = self.section(section_index)?;
        if !section
            .header
            .flags
            .contains(section_header::Flags::COMPRESSED)
        {
            return None;
        }

        match self.header.identification.class {
            Class::Class32 => {
                let size = core::mem::size_of::<compression::class_32::Representation>();
                let representation =
                    read::<compression::class_32::Representation>(section.contents, 0)?;
                let header = CompressionHeader::from(representation);
                let data = section.contents.get(size..)?;
                Some(CompressedSection::new(header, data))
            }
            Class::Class64 => {
                let size = core::mem::size_of::<compression::class_64::Representation>();
                let representation =
                    read::<compression::class_64::Representation>(section.contents, 0)?;
                let header = CompressionHeader::from(representation);
                let data = section.contents.get(size..)?;
                Some(CompressedSection::new(header, data))
            }
            Class::None | Class::Reserved(_) => None,
        }
    }
}

fn parse_dynamic_array(
    bytes: &[u8],
    class: Class,
    entry_size: usize,
) -> Option<DynamicArray> {
    if entry_size == 0 || bytes.len() % entry_size != 0 {
        return None;
    }

    let count = bytes.len() / entry_size;
    let mut entries = Vec::with_capacity(count);

    match class {
        Class::Class32 => {
            if entry_size < core::mem::size_of::<dynamic::class_32::Representation>() {
                return None;
            }

            for index in 0..count {
                let offset = index.checked_mul(entry_size)?;
                let representation =
                    read::<dynamic::class_32::Representation>(bytes, offset)?;
                let entry = Dynamic::from(representation);
                let end = matches!(entry.tag, Tag::Null);
                entries.push(entry);
                if end {
                    break;
                }
            }
        }
        Class::Class64 => {
            if entry_size < core::mem::size_of::<dynamic::class_64::Representation>() {
                return None;
            }

            for index in 0..count {
                let offset = index.checked_mul(entry_size)?;
                let representation =
                    read::<dynamic::class_64::Representation>(bytes, offset)?;
                let entry = Dynamic::from(representation);
                let end = matches!(entry.tag, Tag::Null);
                entries.push(entry);
                if end {
                    break;
                }
            }
        }
        Class::None | Class::Reserved(_) => return None,
    }

    Some(DynamicArray::new(entries))
}

fn parse_note_table<'file>(bytes: &'file [u8], class: Class) -> Option<NoteTable<'file>> {
    let word_size = match class {
        Class::Class32 => 4usize,
        Class::Class64 => 8usize,
        Class::None | Class::Reserved(_) => return None,
    };

    let mut notes = Vec::new();
    let mut offset = 0usize;

    while offset < bytes.len() {
        let namesz = read_word(bytes, offset, word_size)?;
        offset = offset.checked_add(word_size)?;
        let descsz = read_word(bytes, offset, word_size)?;
        offset = offset.checked_add(word_size)?;
        let r#type = read_word(bytes, offset, word_size)?;
        offset = offset.checked_add(word_size)?;

        let name_length = usize::try_from(namesz).ok()?;
        let name_end = offset.checked_add(name_length)?;
        let name = bytes.get(offset..name_end)?;
        offset = align(name_end, word_size)?;

        let descriptor_length = usize::try_from(descsz).ok()?;
        let descriptor_end = offset.checked_add(descriptor_length)?;
        let descriptor = bytes.get(offset..descriptor_end)?;
        offset = align(descriptor_end, word_size)?;

        notes.push(Note::new(name, r#type, descriptor));
    }

    Some(NoteTable::new(notes))
}

fn initial_section_header_32(
    bytes: &[u8],
    header: &Header,
) -> Result<Option<SectionHeader>, ParseError> {
    if header.section_header_offset == 0 {
        return Ok(None);
    }

    if (header.section_header_entry_size as usize)
        < core::mem::size_of::<section_header::class_32::Representation>()
    {
        return Err(ParseError::SectionHeaderEntryTooSmall);
    }

    let offset =
        usize::try_from(header.section_header_offset).map_err(|_| ParseError::Truncated)?;
    Ok(read::<section_header::class_32::Representation>(bytes, offset).map(SectionHeader::from))
}

fn initial_section_header_64(
    bytes: &[u8],
    header: &Header,
) -> Result<Option<SectionHeader>, ParseError> {
    if header.section_header_offset == 0 {
        return Ok(None);
    }

    if (header.section_header_entry_size as usize)
        < core::mem::size_of::<section_header::class_64::Representation>()
    {
        return Err(ParseError::SectionHeaderEntryTooSmall);
    }

    let offset =
        usize::try_from(header.section_header_offset).map_err(|_| ParseError::Truncated)?;
    Ok(read::<section_header::class_64::Representation>(bytes, offset).map(SectionHeader::from))
}

fn resolve_section_header_count(
    header: &Header,
    initial: Option<SectionHeader>,
) -> Result<usize, ParseError> {
    match header.section_header_count {
        header::SectionHeaderCount::Direct(count) => {
            if header.section_header_offset == 0 {
                return Err(ParseError::InvalidSectionHeaderTable);
            }
            Ok(count as usize)
        }
        header::SectionHeaderCount::ZeroOrExtended if header.section_header_offset == 0 => Ok(0),
        header::SectionHeaderCount::ZeroOrExtended => {
            let initial = initial.ok_or(ParseError::Truncated)?;
            let count = usize::try_from(initial.size).map_err(|_| ParseError::Truncated)?;
            if count == 0 {
                return Err(ParseError::InvalidSectionHeaderTable);
            }
            Ok(count)
        }
    }
}

fn resolve_section_name_string_table_index(
    header: &Header,
    initial: Option<SectionHeader>,
) -> Result<Option<usize>, ParseError> {
    match header.section_name_string_table_index {
        header::SectionNameStringTableIndex::Undefined => Ok(None),
        header::SectionNameStringTableIndex::Direct(index) => Ok(Some(index.raw() as usize)),
        header::SectionNameStringTableIndex::Extended => {
            let initial = initial.ok_or(ParseError::InvalidSectionNameStringTable)?;
            Ok(Some(initial.link as usize))
        }
    }
}

fn align(value: usize, alignment: usize) -> Option<usize> {
    let mask = alignment.checked_sub(1)?;
    value.checked_add(mask).map(|value| value & !mask)
}

fn read_word(bytes: &[u8], offset: usize, word_size: usize) -> Option<u64> {
    match word_size {
        4 => read::<u32>(bytes, offset).map(u64::from),
        8 => read::<u64>(bytes, offset),
        _ => None,
    }
}

fn native_data_encoding(data: Data) -> bool {
    #[cfg(target_endian = "little")]
    {
        matches!(data, Data::LeastSignificantByteFirst)
    }

    #[cfg(target_endian = "big")]
    {
        matches!(data, Data::MostSignificantByteFirst)
    }
}

fn parse_table<T, F>(
    bytes: &[u8],
    offset: u64,
    entry_size: u16,
    count: usize,
    minimum_entry_size: usize,
    mut parse: F,
    size_error: ParseError,
) -> Result<Vec<T>, ParseError>
where
    F: FnMut(&[u8], usize) -> Option<T>,
{
    if count == 0 {
        return Ok(Vec::new());
    }

    if (entry_size as usize) < minimum_entry_size {
        return Err(size_error);
    }

    let offset = usize::try_from(offset).map_err(|_| ParseError::Truncated)?;
    let entry_size = entry_size as usize;
    let mut entries = Vec::with_capacity(count);

    for index in 0..count {
        let entry_offset = offset
            .checked_add(index.checked_mul(entry_size).ok_or(ParseError::Truncated)?)
            .ok_or(ParseError::Truncated)?;
        entries.push(parse(bytes, entry_offset).ok_or(ParseError::Truncated)?);
    }

    Ok(entries)
}

fn range(bytes: &[u8], offset: u64, size: u64) -> Option<&[u8]> {
    let start = usize::try_from(offset).ok()?;
    let length = usize::try_from(size).ok()?;
    let end = start.checked_add(length)?;
    bytes.get(start..end)
}

fn read<T: Copy>(bytes: &[u8], offset: usize) -> Option<T> {
    let size = core::mem::size_of::<T>();
    let end = offset.checked_add(size)?;
    let source = bytes.get(offset..end)?;

    Some(unsafe { source.as_ptr().cast::<T>().read_unaligned() })
}
