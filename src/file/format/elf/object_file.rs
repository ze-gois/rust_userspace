//! Parsed ELF object file.

use ample::r#type::Vec;

use super::{
    compression::{self, CompressedSection, CompressionHeader},
    dynamic::section::Section as DynamicSection,
    hash::HashTable,
    header::Header,
    loadable_segment::LoadableSegment,
    note_table::NoteTable,
    identification::Class,
    program_header::{self, ProgramHeader},
    program_header_table_image::ProgramHeaderTableImage,
    program_interpreter::ProgramInterpreter,
    relocation::{self, relative, Relocation},
    relocation_table::{RelocationTable, TargetSection as RelocationTargetSection},
    section_group::{Flags as SectionGroupFlags, Member as SectionGroupMember, SectionGroup},
    section::LinkOrder,
    section_link::{Meaning as SectionLinkMeaning, SectionLink},
    section_header::{self, SectionHeader},
    segment::contents::{ImageContribution, Section as SegmentSection},
    string_table::StringTable,
    symbol::{self, Symbol},
    symbol_table::SymbolTable,
    thread_local_storage,
};

mod dynamic;
mod parser;

use self::parser::{parse_dynamic_array, parse_note_table, range, word};

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

    pub fn validate_header(
        &self,
    ) -> Result<(), super::header::ValidationError> {
        use super::header::{Type as ObjectType, ValidationError, Version};
        use super::identification;

        if self.header.identification.version != Version::Current.raw() as u8 {
            return Err(ValidationError::IdentificationVersionNotCurrent {
                version: self.header.identification.version,
            });
        }

        for (padding_index, value) in self
            .header
            .identification
            .padding
            .iter()
            .copied()
            .enumerate()
        {
            if value != 0 {
                return Err(ValidationError::IdentificationPaddingNotZero {
                    index: identification::PADDING_INDEX + padding_index,
                    value,
                });
            }
        }

        if !matches!(self.header.version, Version::Current) {
            return Err(ValidationError::ObjectVersionNotCurrent {
                version: self.header.version.raw(),
            });
        }

        if let ObjectType::Reserved(raw) = self.header.r#type {
            return Err(ValidationError::ReservedObjectType { raw });
        }

        if !self.header.machine.is_assigned() {
            return Err(ValidationError::ReservedMachine {
                raw: self.header.machine.raw(),
            });
        }

        let minimum_header_size = match self.header.identification.class {
            Class::Class32 => {
                core::mem::size_of::<super::header::class_32::Representation>() as u16
            }
            Class::Class64 => {
                core::mem::size_of::<super::header::class_64::Representation>() as u16
            }
            Class::None | Class::Reserved(_) => self.header.header_size,
        };

        if self.header.header_size < minimum_header_size {
            return Err(ValidationError::HeaderSizeTooSmall {
                size: self.header.header_size,
                minimum: minimum_header_size,
            });
        }

        if usize::from(self.header.header_size) > self.bytes.len() {
            return Err(ValidationError::HeaderSizeExceedsFile {
                size: self.header.header_size,
            });
        }

        if self.header.program_header_count == 0 {
            if self.header.program_header_offset != 0 {
                return Err(ValidationError::ProgramHeaderOffsetWithoutTable {
                    offset: self.header.program_header_offset,
                });
            }
        } else if self.header.program_header_offset == 0 {
            return Err(ValidationError::ProgramHeaderTableWithoutOffset {
                count: self.header.program_header_count,
            });
        }

        if self.section_header_count == 0 {
            if self.header.section_header_offset != 0 {
                return Err(ValidationError::SectionHeaderOffsetWithoutTable {
                    offset: self.header.section_header_offset,
                });
            }
        } else if self.header.section_header_offset == 0 {
            return Err(ValidationError::SectionHeaderTableWithoutOffset {
                count: self.section_header_count,
            });
        }

        Ok(())
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

    pub fn validate_section_headers(
        &self,
    ) -> Result<(), section_header::ValidationError> {
        use super::header::{SectionHeaderCount, SectionNameStringTableIndex};
        use section_header::{Flags, ValidationError};

        if let Some(initial) = self.section_headers.first() {
            if initial.name_index != 0 {
                return Err(ValidationError::UndefinedSectionNameNotZero);
            }
            if !matches!(initial.r#type, section_header::Type::Null) {
                return Err(ValidationError::UndefinedSectionTypeNotNull);
            }
            if initial.flags.raw() != 0 {
                return Err(ValidationError::UndefinedSectionFlagsNotZero);
            }
            if initial.address != 0 {
                return Err(ValidationError::UndefinedSectionAddressNotZero);
            }
            if initial.offset != 0 {
                return Err(ValidationError::UndefinedSectionOffsetNotZero);
            }
            if initial.information != 0 {
                return Err(ValidationError::UndefinedSectionInformationNotZero);
            }
            if initial.alignment != 0 {
                return Err(ValidationError::UndefinedSectionAlignmentNotZero);
            }
            if initial.entry_size != 0 {
                return Err(ValidationError::UndefinedSectionEntrySizeNotZero);
            }

            match self.header.section_header_count {
                SectionHeaderCount::Direct(_) => {
                    if initial.size != 0 {
                        return Err(ValidationError::DirectSectionCountWithInitialSize {
                            size: initial.size,
                        });
                    }
                }
                SectionHeaderCount::ZeroOrExtended => {
                    if self.header.section_header_offset != 0
                        && self.section_header_count
                            < section_header::Index::LOWER_RESERVED.raw() as usize
                    {
                        return Err(ValidationError::ExtendedSectionCountBelowReservedRange {
                            count: self.section_header_count,
                        });
                    }
                }
            }

            match self.header.section_name_string_table_index {
                SectionNameStringTableIndex::Undefined
                | SectionNameStringTableIndex::Direct(_) => {
                    if initial.link != 0 {
                        return Err(
                            ValidationError::DirectSectionNameStringTableWithInitialLink {
                                link: initial.link,
                            },
                        );
                    }
                }
                SectionNameStringTableIndex::Extended => {
                    let index = self
                        .section_name_string_table_index
                        .ok_or(
                            ValidationError::ExtendedSectionNameStringTableBelowReservedRange {
                                index: 0,
                            },
                        )?;
                    if index < section_header::Index::LOWER_RESERVED.raw() as usize {
                        return Err(
                            ValidationError::ExtendedSectionNameStringTableBelowReservedRange {
                                index,
                            },
                        );
                    }
                }
            }
        }

        let mut dynamic_section_index = None;
        let mut hash_section_index = None;
        let mut symbol_table_section_index = None;
        let mut dynamic_symbol_table_section_index = None;

        for (index, header) in self.section_headers.iter().enumerate() {
            if index != 0 && matches!(header.r#type, section_header::Type::Null) {
                continue;
            }

            if let section_header::Type::Reserved(raw) = header.r#type {
                return Err(ValidationError::ReservedSectionType { index, raw });
            }

            let undefined_flags = header.flags.raw() & !Flags::DEFINED_MASK;
            if undefined_flags != 0 {
                return Err(ValidationError::UndefinedFlagsSet {
                    index,
                    flags: undefined_flags,
                });
            }

            if !header.flags.contains(Flags::ALLOCATE) && header.address != 0 {
                return Err(ValidationError::NonAllocatedSectionAddressNotZero {
                    index,
                    address: header.address,
                });
            }

            match header.r#type {
                section_header::Type::Dynamic => {
                    if let Some(first) = dynamic_section_index {
                        return Err(ValidationError::MultipleDynamicSections {
                            first,
                            second: index,
                        });
                    }
                    dynamic_section_index = Some(index);
                }
                section_header::Type::Hash => {
                    if let Some(first) = hash_section_index {
                        return Err(ValidationError::MultipleHashSections {
                            first,
                            second: index,
                        });
                    }
                    hash_section_index = Some(index);
                }
                section_header::Type::SymbolTable => {
                    if let Some(first) = symbol_table_section_index {
                        return Err(ValidationError::MultipleSymbolTables {
                            first,
                            second: index,
                        });
                    }
                    symbol_table_section_index = Some(index);
                }
                section_header::Type::DynamicSymbolTable => {
                    if let Some(first) = dynamic_symbol_table_section_index {
                        return Err(ValidationError::MultipleDynamicSymbolTables {
                            first,
                            second: index,
                        });
                    }
                    dynamic_symbol_table_section_index = Some(index);
                }
                _ => {}
            }

            if header.alignment > 1 && !header.alignment.is_power_of_two() {
                return Err(ValidationError::AlignmentNotPowerOfTwo { index });
            }

            if header.alignment > 1 && header.address % header.alignment != 0 {
                return Err(ValidationError::AddressMisaligned { index });
            }

            if header.flags.contains(Flags::INFORMATION_LINK) {
                let target = usize::try_from(header.information)
                    .map_err(|_| ValidationError::InformationLinkOutOfBounds {
                        index,
                        target: usize::MAX,
                    })?;

                if target >= self.section_headers.len() {
                    return Err(ValidationError::InformationLinkOutOfBounds {
                        index,
                        target,
                    });
                }
            }

            if header.flags.contains(Flags::MERGE) || header.flags.contains(Flags::STRINGS) {
                if header.entry_size == 0 {
                    return Err(ValidationError::MergeOrStringsEntrySizeZero { index });
                }

                if header.size % header.entry_size != 0 {
                    return Err(ValidationError::MergeOrStringsSizeNotEntryMultiple { index });
                }
            }
        }

        for (index, header) in self.section_headers.iter().enumerate() {
            if matches!(
                header.r#type,
                section_header::Type::Null | section_header::Type::NoBits
            ) || header.size == 0
            {
                continue;
            }

            let end = header
                .offset
                .checked_add(header.size)
                .ok_or(ValidationError::SectionOutsideFile { index })?;
            if end > self.bytes.len() as u64 {
                return Err(ValidationError::SectionOutsideFile { index });
            }
        }

        for first in 0..self.section_headers.len() {
            let first_header = self.section_headers[first];
            if matches!(
                first_header.r#type,
                section_header::Type::Null | section_header::Type::NoBits
            ) || first_header.size == 0
            {
                continue;
            }
            let first_end = first_header.offset + first_header.size;

            for second in (first + 1)..self.section_headers.len() {
                let second_header = self.section_headers[second];
                if matches!(
                    second_header.r#type,
                    section_header::Type::Null | section_header::Type::NoBits
                ) || second_header.size == 0
                {
                    continue;
                }
                let second_end = second_header.offset + second_header.size;

                if first_header.offset < second_end && second_header.offset < first_end {
                    return Err(ValidationError::SectionsOverlap { first, second });
                }
            }
        }

        if let Some(string_table_index) = self.section_name_string_table_index {
            let string_table_header = self
                .section_headers
                .get(string_table_index)
                .ok_or(ValidationError::InvalidSectionNameStringTableType {
                    index: string_table_index,
                })?;
            if !matches!(string_table_header.r#type, section_header::Type::StringTable) {
                return Err(ValidationError::InvalidSectionNameStringTableType {
                    index: string_table_index,
                });
            }

            let strings = self
                .section_name_string_table()
                .ok_or(ValidationError::InvalidSectionNameStringTableType {
                    index: string_table_index,
                })?;
            for (index, header) in self.section_headers.iter().enumerate() {
                if index != 0 && matches!(header.r#type, section_header::Type::Null) {
                    continue;
                }

                if strings.get(header.name_index as usize).is_none() {
                    return Err(ValidationError::InvalidSectionName {
                        index,
                        name_index: header.name_index,
                    });
                }
            }
        }

        Ok(())
    }

    pub fn lowest_load_virtual_address(&self) -> Option<u64> {
        self.program_headers
            .iter()
            .filter(|header| matches!(header.r#type, program_header::Type::Load))
            .map(|header| header.virtual_address)
            .min()
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

    pub fn note_table_from_program_header(&self, index: usize) -> Option<NoteTable<'file>> {
        let program_header = *self.program_headers.get(index)?;
        if !matches!(program_header.r#type, program_header::Type::Note) {
            return None;
        }

        let segment = self.segment(index)?;
        parse_note_table(
            segment.file_image,
            self.header.identification.class,
            self.header.identification.data,
        )
    }

    pub fn sections_in_loadable_segment(
        &self,
        program_header_index: usize,
    ) -> Option<Vec<SegmentSection>> {
        let program_header = *self.program_headers.get(program_header_index)?;
        if !matches!(program_header.r#type, program_header::Type::Load) {
            return None;
        }

        let memory_end = program_header
            .virtual_address
            .checked_add(program_header.memory_size)?;
        let file_end = program_header.offset.checked_add(program_header.file_size)?;
        let mut sections = Vec::new();

        for (section_index, section_header) in self.section_headers.iter().copied().enumerate() {
            if !section_header.flags.contains(section_header::Flags::ALLOCATE) {
                continue;
            }

            let section_memory_end = section_header.address.checked_add(section_header.size)?;
            let inside_memory = section_header.address >= program_header.virtual_address
                && section_memory_end <= memory_end;

            if !inside_memory {
                continue;
            }

            let contribution = if matches!(section_header.r#type, section_header::Type::NoBits) {
                ImageContribution::MemoryOnly
            } else {
                let section_file_end = section_header.offset.checked_add(section_header.size)?;
                let inside_file = section_header.offset >= program_header.offset
                    && section_file_end <= file_end;

                if !inside_file {
                    continue;
                }

                let address_displacement =
                    section_header.address.checked_sub(program_header.virtual_address)?;
                let file_displacement = section_header.offset.checked_sub(program_header.offset)?;

                if address_displacement != file_displacement {
                    continue;
                }

                ImageContribution::FileAndMemory
            };

            sections.push(SegmentSection::new(
                section_index,
                section_header,
                contribution,
            ));
        }

        Some(sections)
    }

    pub fn loadable_segment_for_section(
        &self,
        section_index: usize,
    ) -> Option<(usize, SegmentSection)> {
        for program_header_index in 0..self.program_headers.len() {
            let Some(sections) = self.sections_in_loadable_segment(program_header_index) else {
                continue;
            };

            if let Some(section) = sections
                .into_iter()
                .find(|section| section.section_index == section_index)
            {
                return Some((program_header_index, section));
            }
        }

        None
    }

    pub fn section_link(&self, section_index: usize) -> Option<SectionLink<'file>> {
        let header = *self.section_headers.get(section_index)?;

        let meaning = match header.r#type {
            section_header::Type::Dynamic
            | section_header::Type::SymbolTable
            | section_header::Type::DynamicSymbolTable => SectionLinkMeaning::StringTable,
            section_header::Type::Hash
            | section_header::Type::Relocation
            | section_header::Type::RelocationWithAddend
            | section_header::Type::Group
            | section_header::Type::SymbolTableSectionIndex => {
                SectionLinkMeaning::SymbolTable
            }
            _ => return None,
        };

        let linked_section_index = usize::try_from(header.link).ok()?;
        let linked_section = self.section(linked_section_index)?;

        match meaning {
            SectionLinkMeaning::StringTable
                if !matches!(linked_section.header.r#type, section_header::Type::StringTable) =>
            {
                return None;
            }
            SectionLinkMeaning::SymbolTable
                if !matches!(
                    linked_section.header.r#type,
                    section_header::Type::SymbolTable
                        | section_header::Type::DynamicSymbolTable
                ) =>
            {
                return None;
            }
            _ => {}
        }

        Some(SectionLink::new(
            section_index,
            linked_section_index,
            linked_section,
            meaning,
        ))
    }

    pub fn validate_section_links(
        &self,
    ) -> Result<(), super::section_link::ValidationError> {
        use super::section_link::ValidationError;

        for (section_index, header) in self.section_headers.iter().copied().enumerate() {
            let requires_zero_information = matches!(
                header.r#type,
                section_header::Type::Dynamic
                    | section_header::Type::Hash
                    | section_header::Type::SymbolTableSectionIndex
            );

            if requires_zero_information && header.information != 0 {
                return Err(ValidationError::InformationMustBeZero {
                    section_index,
                    information: header.information,
                });
            }

            let link_meaning = match header.r#type {
                section_header::Type::Dynamic
                | section_header::Type::SymbolTable
                | section_header::Type::DynamicSymbolTable => {
                    Some(SectionLinkMeaning::StringTable)
                }
                section_header::Type::Hash
                | section_header::Type::Relocation
                | section_header::Type::RelocationWithAddend
                | section_header::Type::Group
                | section_header::Type::SymbolTableSectionIndex => {
                    Some(SectionLinkMeaning::SymbolTable)
                }
                _ => None,
            };

            if let Some(meaning) = link_meaning {
                let linked_section_index = usize::try_from(header.link).map_err(|_| {
                    ValidationError::LinkOutOfBounds {
                        section_index,
                        linked_section_index: usize::MAX,
                    }
                })?;
                let linked_header = self.section_headers.get(linked_section_index).ok_or(
                    ValidationError::LinkOutOfBounds {
                        section_index,
                        linked_section_index,
                    },
                )?;

                match meaning {
                    SectionLinkMeaning::StringTable => {
                        if !matches!(linked_header.r#type, section_header::Type::StringTable) {
                            return Err(ValidationError::LinkNotStringTable {
                                section_index,
                                linked_section_index,
                            });
                        }
                    }
                    SectionLinkMeaning::SymbolTable => {
                        if !matches!(
                            linked_header.r#type,
                            section_header::Type::SymbolTable
                                | section_header::Type::DynamicSymbolTable
                        ) {
                            return Err(ValidationError::LinkNotSymbolTable {
                                section_index,
                                linked_section_index,
                            });
                        }
                    }
                }
            }

            if matches!(
                header.r#type,
                section_header::Type::Relocation | section_header::Type::RelocationWithAddend
            ) && header.information != section_header::Index::UNDEFINED.raw() as u32
            {
                let target_section_index =
                    usize::try_from(header.information).map_err(|_| {
                        ValidationError::RelocationTargetOutOfBounds {
                            section_index,
                            target_section_index: usize::MAX,
                        }
                    })?;

                if target_section_index >= self.section_headers.len() {
                    return Err(ValidationError::RelocationTargetOutOfBounds {
                        section_index,
                        target_section_index,
                    });
                }
            }

            if matches!(header.r#type, section_header::Type::Group) {
                let symbols = self.symbol_table(header.link as usize).ok_or(
                    ValidationError::LinkNotSymbolTable {
                        section_index,
                        linked_section_index: header.link as usize,
                    },
                )?;
                let symbol_index = header.information as usize;
                if symbols.get(symbol_index).is_none() {
                    return Err(ValidationError::GroupSignatureOutOfBounds {
                        section_index,
                        symbol_index,
                    });
                }
            }

            if header.flags.contains(section_header::Flags::LINK_ORDER) {
                let target_section_index = usize::try_from(header.link).map_err(|_| {
                    ValidationError::LinkOrderTargetOutOfBounds {
                        section_index,
                        target_section_index: usize::MAX,
                    }
                })?;
                if target_section_index >= self.section_headers.len() {
                    return Err(ValidationError::LinkOrderTargetOutOfBounds {
                        section_index,
                        target_section_index,
                    });
                }
            }
        }

        Ok(())
    }

    pub fn link_order(&self, section_index: usize) -> Option<LinkOrder<'file>> {
        let metadata = self.section(section_index)?;
        if !metadata
            .header
            .flags
            .contains(section_header::Flags::LINK_ORDER)
        {
            return None;
        }

        let referenced_index = usize::try_from(metadata.header.link).ok()?;
        let referenced = self.section(referenced_index)?;

        Some(LinkOrder::new(
            section_index,
            metadata,
            referenced_index,
            referenced,
        ))
    }

    pub fn section_name_string_table(&self) -> Option<StringTable<'file>> {
        let index = self.section_name_string_table_index?;
        let section = self.section(index)?;
        if !matches!(section.header.r#type, section_header::Type::StringTable) {
            return None;
        }
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
                    let representation = symbol::class_32::Representation::decode(
                        section.contents,
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

                for index in 0..count {
                    let offset = index.checked_mul(entry_size)?;
                    let representation = symbol::class_64::Representation::decode(
                        section.contents,
                        offset,
                        self.header.identification.data,
                    )?;
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
                let extended = word(
                    extended_section_indices.contents,
                    offset,
                    self.header.identification.data,
                )?;

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

    pub fn validate_symbol_table_section_indices(
        &self,
        symbol_table_section_index: usize,
    ) -> Result<(), super::symbol_table::ValidationError> {
        use super::symbol_table::ValidationError;

        let header = self
            .section_headers
            .get(symbol_table_section_index)
            .ok_or(ValidationError::MissingSectionIndexTable)?;

        if !matches!(
            header.r#type,
            section_header::Type::SymbolTable | section_header::Type::DynamicSymbolTable
        ) {
            return Ok(());
        }

        let symbol_section = self
            .section(symbol_table_section_index)
            .ok_or(ValidationError::SectionIndexTableSizeMismatch)?;
        let entry_size = usize::try_from(header.entry_size)
            .ok()
            .filter(|entry_size| *entry_size != 0)
            .ok_or(ValidationError::SectionIndexTableSizeMismatch)?;
        if symbol_section.contents.len() % entry_size != 0 {
            return Err(ValidationError::SectionIndexTableSizeMismatch);
        }
        let symbol_count = symbol_section.contents.len() / entry_size;

        let companion = self
            .section_headers
            .iter()
            .enumerate()
            .find(|(_, candidate)| {
                matches!(
                    candidate.r#type,
                    section_header::Type::SymbolTableSectionIndex
                ) && candidate.link as usize == symbol_table_section_index
            })
            .and_then(|(index, _)| self.section(index));

        let symbols = self.symbol_table(symbol_table_section_index);
        if symbols.is_none() {
            let mut requires_extended = false;
            match self.header.identification.class {
                Class::Class32 => {
                    for index in 0..symbol_count {
                        let offset = index
                            .checked_mul(entry_size)
                            .ok_or(ValidationError::SectionIndexTableSizeMismatch)?;
                        let representation = symbol::class_32::Representation::decode(
                            symbol_section.contents,
                            offset,
                            self.header.identification.data,
                        )
                        .ok_or(ValidationError::SectionIndexTableSizeMismatch)?;
                        if representation.st_shndx == section_header::Index::EXTENDED.raw() {
                            requires_extended = true;
                            break;
                        }
                    }
                }
                Class::Class64 => {
                    for index in 0..symbol_count {
                        let offset = index
                            .checked_mul(entry_size)
                            .ok_or(ValidationError::SectionIndexTableSizeMismatch)?;
                        let representation = symbol::class_64::Representation::decode(
                            symbol_section.contents,
                            offset,
                            self.header.identification.data,
                        )
                        .ok_or(ValidationError::SectionIndexTableSizeMismatch)?;
                        if representation.st_shndx == section_header::Index::EXTENDED.raw() {
                            requires_extended = true;
                            break;
                        }
                    }
                }
                Class::None | Class::Reserved(_) => return Ok(()),
            }

            if requires_extended && companion.is_none() {
                return Err(ValidationError::MissingSectionIndexTable);
            }
        }

        let Some(companion) = companion else {
            return Ok(());
        };

        let word_size = core::mem::size_of::<u32>();
        if companion.contents.len() != symbol_count * word_size {
            return Err(ValidationError::SectionIndexTableSizeMismatch);
        }

        match self.header.identification.class {
            Class::Class32 => {
                for index in 0..symbol_count {
                    let symbol_offset = index * entry_size;
                    let representation = symbol::class_32::Representation::decode(
                        symbol_section.contents,
                        symbol_offset,
                        self.header.identification.data,
                    )
                    .ok_or(ValidationError::SectionIndexTableSizeMismatch)?;
                    let extended = word(
                        companion.contents,
                        index * word_size,
                        self.header.identification.data,
                    )
                    .ok_or(ValidationError::SectionIndexTableSizeMismatch)?;
                    if representation.st_shndx != section_header::Index::EXTENDED.raw()
                        && extended != 0
                    {
                        return Err(ValidationError::SectionIndexTableUnexpectedValue {
                            index,
                            value: extended,
                        });
                    }
                }
            }
            Class::Class64 => {
                for index in 0..symbol_count {
                    let symbol_offset = index * entry_size;
                    let representation = symbol::class_64::Representation::decode(
                        symbol_section.contents,
                        symbol_offset,
                        self.header.identification.data,
                    )
                    .ok_or(ValidationError::SectionIndexTableSizeMismatch)?;
                    let extended = word(
                        companion.contents,
                        index * word_size,
                        self.header.identification.data,
                    )
                    .ok_or(ValidationError::SectionIndexTableSizeMismatch)?;
                    if representation.st_shndx != section_header::Index::EXTENDED.raw()
                        && extended != 0
                    {
                        return Err(ValidationError::SectionIndexTableUnexpectedValue {
                            index,
                            value: extended,
                        });
                    }
                }
            }
            Class::None | Class::Reserved(_) => {}
        }

        Ok(())
    }

    pub fn validate_symbol_table(
        &self,
        symbol_table_section_index: usize,
    ) -> Result<(), super::symbol_table::ValidationError> {
        use super::{
            header::Type as ObjectType,
            symbol::{ResolvedSectionIndex, Type as SymbolType},
            symbol_table::ValidationError,
        };

        let symbols = self
            .symbol_table(symbol_table_section_index)
            .ok_or(ValidationError::InvalidSymbolTableSection {
                section_index: symbol_table_section_index,
            })?;

        symbols.validate()?;

        for (index, symbol) in symbols.symbols.iter().enumerate() {
            let resolved = symbols
                .section_index(index)
                .ok_or(ValidationError::InvalidSymbolTableSection {
                    section_index: symbol_table_section_index,
                })?;

            match resolved {
                ResolvedSectionIndex::Section(section_index) => {
                    if section_index >= self.section_headers.len() {
                        return Err(ValidationError::SectionIndexOutOfBounds {
                            index,
                            section_index,
                        });
                    }
                }
                ResolvedSectionIndex::Reserved(raw) => {
                    return Err(ValidationError::ReservedSectionIndex { index, raw });
                }
                ResolvedSectionIndex::Common => {
                    if !matches!(self.header.r#type, ObjectType::Relocatable) {
                        return Err(
                            ValidationError::CommonSectionIndexOutsideRelocatableObject {
                                index,
                            },
                        );
                    }

                    if symbol.value > 1 && !symbol.value.is_power_of_two() {
                        return Err(ValidationError::CommonAlignmentNotPowerOfTwo {
                            index,
                            alignment: symbol.value,
                        });
                    }
                }
                ResolvedSectionIndex::Undefined
                | ResolvedSectionIndex::ProcessorSpecific(_)
                | ResolvedSectionIndex::OperatingSystemSpecific(_)
                | ResolvedSectionIndex::Absolute => {}
            }

            if matches!(symbol.r#type, SymbolType::Common) {
                match self.header.r#type {
                    ObjectType::Relocatable => {
                        if !matches!(resolved, ResolvedSectionIndex::Common) {
                            return Err(ValidationError::CommonSymbolWithoutCommonSection {
                                index,
                            });
                        }
                    }
                    ObjectType::Executable | ObjectType::SharedObject => {
                        if !matches!(resolved, ResolvedSectionIndex::Section(_)) {
                            return Err(ValidationError::CommonSymbolWithoutAllocatedSection {
                                index,
                            });
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }

    pub fn symbol_section(
        &self,
        symbol_table_section_index: usize,
        symbol_index: usize,
    ) -> Option<super::section::Section<'file>> {
        let symbols = self.symbol_table(symbol_table_section_index)?;
        let section_index = match symbols.section_index(symbol_index)? {
            symbol::ResolvedSectionIndex::Section(index) => index,
            symbol::ResolvedSectionIndex::Undefined
            | symbol::ResolvedSectionIndex::ProcessorSpecific(_)
            | symbol::ResolvedSectionIndex::OperatingSystemSpecific(_)
            | symbol::ResolvedSectionIndex::Absolute
            | symbol::ResolvedSectionIndex::Common
            | symbol::ResolvedSectionIndex::Reserved(_) => return None,
        };

        self.section(section_index)
    }

    pub fn relative_relocation_table(
        &self,
        section_index: usize,
    ) -> Option<relative::Table> {
        self.validate_relative_relocation_section(section_index).ok()?;

        let section = self.section(section_index)?;
        let entry_size = usize::try_from(section.header.entry_size).ok()?;
        let count = section.contents.len() / entry_size;
        let mut entries = Vec::with_capacity(count);

        match self.header.identification.class {
            Class::Class32 => {
                for entry_index in 0..count {
                    let offset = entry_index.checked_mul(entry_size)?;
                    let representation = relative::class_32::Representation::decode(
                        section.contents,
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
                        section.contents,
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

    pub fn validate_relative_relocation_section(
        &self,
        section_index: usize,
    ) -> Result<(), relative::SectionValidationError> {
        use relative::SectionValidationError;

        let Some(header) = self.section_headers.get(section_index) else {
            return Ok(());
        };
        if !matches!(header.r#type, section_header::Type::RelativeRelocation) {
            return Ok(());
        }

        if !matches!(
            self.header.r#type,
            super::header::Type::Executable | super::header::Type::SharedObject
        ) {
            return Err(SectionValidationError::ObjectTypeNotExecutableOrSharedObject);
        }

        let expected_entry_size = match self.header.identification.class {
            Class::Class32 => 4u64,
            Class::Class64 => 8u64,
            Class::None | Class::Reserved(_) => return Ok(()),
        };

        if header.entry_size != expected_entry_size {
            return Err(SectionValidationError::EntrySizeMismatch);
        }
        if header.size % header.entry_size != 0 {
            return Err(SectionValidationError::SizeNotEntryMultiple);
        }

        Ok(())
    }

    pub fn relocation_target_section(
        &self,
        section_index: usize,
    ) -> Option<RelocationTargetSection<'file>> {
        let header = *self.section_headers.get(section_index)?;
        if !matches!(
            header.r#type,
            section_header::Type::Relocation | section_header::Type::RelocationWithAddend
        ) {
            return None;
        }

        let target_section_index = usize::try_from(header.information).ok()?;
        if target_section_index == section_header::Index::UNDEFINED.raw() as usize {
            return None;
        }

        let target_section = self.section(target_section_index)?;
        Some(RelocationTargetSection::new(
            target_section_index,
            target_section,
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
                    let representation = relocation::class_32::RelRepresentation::decode(
                        section.contents,
                        offset,
                        self.header.identification.data,
                    )?;
                    relocations.push(Relocation::from(representation));
                }
            }
            (Class::Class32, true) => {
                if entry_size < core::mem::size_of::<relocation::class_32::RelaRepresentation>() {
                    return None;
                }
                for index in 0..count {
                    let offset = index.checked_mul(entry_size)?;
                    let representation = relocation::class_32::RelaRepresentation::decode(
                        section.contents,
                        offset,
                        self.header.identification.data,
                    )?;
                    relocations.push(Relocation::from(representation));
                }
            }
            (Class::Class64, false) => {
                if entry_size < core::mem::size_of::<relocation::class_64::RelRepresentation>() {
                    return None;
                }
                for index in 0..count {
                    let offset = index.checked_mul(entry_size)?;
                    let representation = relocation::class_64::RelRepresentation::decode(
                        section.contents,
                        offset,
                        self.header.identification.data,
                    )?;
                    relocations.push(Relocation::from(representation));
                }
            }
            (Class::Class64, true) => {
                if entry_size < core::mem::size_of::<relocation::class_64::RelaRepresentation>() {
                    return None;
                }
                for index in 0..count {
                    let offset = index.checked_mul(entry_size)?;
                    let representation = relocation::class_64::RelaRepresentation::decode(
                        section.contents,
                        offset,
                        self.header.identification.data,
                    )?;
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

    pub fn dynamic_section(&self, section_index: usize) -> Option<DynamicSection<'file>> {
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
            self.header.identification.data,
            entry_size,
        )?;

        Some(DynamicSection::new(
            array,
            StringTable::new(strings_section.contents),
        ))
    }

    pub fn hash_table(&self, section_index: usize) -> Option<HashTable> {
        let header = *self.section_headers.get(section_index)?;
        if !matches!(header.r#type, section_header::Type::Hash) {
            return None;
        }

        let section = self.section(section_index)?;
        if section.contents.len() < 8 {
            return None;
        }

        let bucket_count = word(section.contents, 0, self.header.identification.data)? as usize;
        let chain_count = word(section.contents, 4, self.header.identification.data)? as usize;
        let mut offset = 8usize;

        let mut buckets = Vec::with_capacity(bucket_count);
        for _ in 0..bucket_count {
            buckets.push(word(
                section.contents,
                offset,
                self.header.identification.data,
            )?);
            offset = offset.checked_add(core::mem::size_of::<u32>())?;
        }

        let mut chains = Vec::with_capacity(chain_count);
        for _ in 0..chain_count {
            chains.push(word(
                section.contents,
                offset,
                self.header.identification.data,
            )?);
            offset = offset.checked_add(core::mem::size_of::<u32>())?;
        }

        let _symbols = self.symbol_table(header.link as usize)?;
        Some(HashTable::new(buckets, chains))
    }

    pub fn section_group_members(
        &self,
        section_index: usize,
    ) -> Option<Vec<SectionGroupMember<'file>>> {
        let group = self.section_group(section_index)?;
        let mut members = Vec::with_capacity(group.members.len());

        for member_section_index in group.members.iter().copied() {
            let section = self.section(member_section_index)?;
            members.push(SectionGroupMember::new(member_section_index, section));
        }

        Some(members)
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

        let flags = SectionGroupFlags::from_raw(word(
            section.contents,
            0,
            self.header.identification.data,
        )?);
        let count = section.contents.len() / core::mem::size_of::<u32>();
        let mut members = Vec::with_capacity(count.saturating_sub(1));

        for index in 1..count {
            let offset = index.checked_mul(core::mem::size_of::<u32>())?;
            members.push(word(
                section.contents,
                offset,
                self.header.identification.data,
            )? as usize);
        }

        let symbols = self.symbol_table(header.link as usize)?;
        Some(SectionGroup::new(
            flags,
            members,
            symbols,
            header.information as usize,
        ))
    }

    pub fn validate_section_groups(
        &self,
    ) -> Result<(), super::section_group::ValidationError> {
        use super::header::Type as ObjectType;
        use super::section_group::ValidationError;

        let mut grouped_members = Vec::new();

        for (group_index, header) in self.section_headers.iter().enumerate() {
            if !matches!(header.r#type, section_header::Type::Group) {
                continue;
            }

            if !matches!(self.header.r#type, ObjectType::Relocatable) {
                return Err(ValidationError::GroupOutsideRelocatableObject {
                    index: group_index,
                });
            }

            if header.flags.raw() != 0 {
                return Err(ValidationError::GroupSectionFlagsNotZero {
                    index: group_index,
                });
            }

            let group = self
                .section_group(group_index)
                .ok_or(ValidationError::InvalidGroup { index: group_index })?;

            for member_index in group.members.iter().copied() {
                if member_index <= group_index {
                    return Err(ValidationError::MemberNotAfterGroup {
                        group_index,
                        member_index,
                    });
                }

                let member = self
                    .section_headers
                    .get(member_index)
                    .ok_or(ValidationError::InvalidGroup { index: group_index })?;

                if !member.flags.contains(section_header::Flags::GROUP) {
                    return Err(ValidationError::MemberMissingGroupFlag {
                        group_index,
                        member_index,
                    });
                }

                if grouped_members
                    .iter()
                    .any(|(_, index)| *index == member_index)
                {
                    return Err(ValidationError::MemberInMultipleGroups {
                        member_index,
                    });
                }

                grouped_members.push((group_index, member_index));
            }
        }

        for (member_index, header) in self.section_headers.iter().enumerate() {
            if !header.flags.contains(section_header::Flags::GROUP) {
                continue;
            }

            if !matches!(self.header.r#type, ObjectType::Relocatable) {
                return Err(ValidationError::MemberOutsideRelocatableObject {
                    index: member_index,
                });
            }

            if !grouped_members
                .iter()
                .any(|(_, index)| *index == member_index)
            {
                return Err(ValidationError::GroupFlagWithoutGroup {
                    member_index,
                });
            }
        }

        for (source_index, header) in self.section_headers.iter().enumerate() {
            let source_group = grouped_members
                .iter()
                .find(|(_, member_index)| *member_index == source_index)
                .map(|(group_index, _)| *group_index);

            let sh_link_is_section_reference = header
                .flags
                .contains(section_header::Flags::LINK_ORDER)
                || matches!(
                    header.r#type,
                    section_header::Type::Dynamic
                        | section_header::Type::SymbolTable
                        | section_header::Type::DynamicSymbolTable
                        | section_header::Type::Hash
                        | section_header::Type::Relocation
                        | section_header::Type::RelocationWithAddend
                        | section_header::Type::Group
                        | section_header::Type::SymbolTableSectionIndex
                );

            if sh_link_is_section_reference {
                let target = header.link as usize;
                if let Some((target_group, member_index)) = grouped_members
                    .iter()
                    .find(|(_, member_index)| *member_index == target)
                    .copied()
                {
                    if source_group != Some(target_group) {
                        return Err(
                            ValidationError::ExternalNonSymbolReferenceToMember {
                                source_index,
                                member_index,
                            },
                        );
                    }
                }
            }

            let sh_info_is_section_reference = header
                .flags
                .contains(section_header::Flags::INFORMATION_LINK)
                || matches!(
                    header.r#type,
                    section_header::Type::Relocation
                        | section_header::Type::RelocationWithAddend
                );

            if sh_info_is_section_reference {
                let target = header.information as usize;
                if let Some((target_group, member_index)) = grouped_members
                    .iter()
                    .find(|(_, member_index)| *member_index == target)
                    .copied()
                {
                    if source_group != Some(target_group) {
                        return Err(
                            ValidationError::ExternalNonSymbolReferenceToMember {
                                source_index,
                                member_index,
                            },
                        );
                    }
                }
            }
        }

        Ok(())
    }

    pub fn note_table(&self, section_index: usize) -> Option<NoteTable<'file>> {
        let section = self.section(section_index)?;
        if !matches!(section.header.r#type, section_header::Type::Note) {
            return None;
        }

        parse_note_table(
            section.contents,
            self.header.identification.class,
            self.header.identification.data,
        )
    }

    pub fn validate_compressed_section(
        &self,
        section_index: usize,
    ) -> Result<(), compression::ValidationError> {
        use compression::ValidationError;

        let Some(header) = self.section_headers.get(section_index) else {
            return Ok(());
        };
        if !header.flags.contains(section_header::Flags::COMPRESSED) {
            return Ok(());
        }

        if matches!(header.r#type, section_header::Type::NoBits) {
            return Err(ValidationError::NoBitsCompressedSection);
        }

        if header.flags.contains(section_header::Flags::ALLOCATE)
            && matches!(
                self.header.r#type,
                super::header::Type::Executable | super::header::Type::SharedObject
            )
        {
            return Err(
                ValidationError::AllocatedCompressedSectionInExecutableOrSharedObject,
            );
        }

        let section = self
            .section(section_index)
            .ok_or(ValidationError::MissingCompressionHeader)?;

        let compression_header = match self.header.identification.class {
            Class::Class32 => {
                let representation = compression::class_32::Representation::decode(
                    section.contents,
                    0,
                    self.header.identification.data,
                )
                .ok_or(ValidationError::MissingCompressionHeader)?;
                CompressionHeader::from(representation)
            }
            Class::Class64 => {
                let representation = compression::class_64::Representation::decode(
                    section.contents,
                    0,
                    self.header.identification.data,
                )
                .ok_or(ValidationError::MissingCompressionHeader)?;
                CompressionHeader::from(representation)
            }
            Class::None | Class::Reserved(_) => {
                return Err(ValidationError::MissingCompressionHeader)
            }
        };

        if compression_header.alignment > 1
            && !compression_header.alignment.is_power_of_two()
        {
            return Err(ValidationError::UncompressedAlignmentNotPowerOfTwo);
        }

        Ok(())
    }

    pub fn compressed_section(&self, section_index: usize) -> Option<CompressedSection<'file>> {
        self.validate_compressed_section(section_index).ok()?;
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
                let representation = compression::class_32::Representation::decode(
                    section.contents,
                    0,
                    self.header.identification.data,
                )?;
                let header = CompressionHeader::from(representation);
                let data = section.contents.get(size..)?;
                Some(CompressedSection::new(header, data))
            }
            Class::Class64 => {
                let size = core::mem::size_of::<compression::class_64::Representation>();
                let representation = compression::class_64::Representation::decode(
                    section.contents,
                    0,
                    self.header.identification.data,
                )?;
                let header = CompressionHeader::from(representation);
                let data = section.contents.get(size..)?;
                Some(CompressedSection::new(header, data))
            }
            Class::None | Class::Reserved(_) => None,
        }
    }
}

