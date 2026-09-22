//! Mechanical decoding of ELF object-file representations.
//!
//! This module turns serialized bytes into ELF structures. Cross-structure
//! relationships remain the responsibility of `ObjectFile`.

use ample::r#type::Vec;

use super::{ObjectFile, ParseError};
use super::super::{
    dynamic::{self, Dynamic, Tag},
    dynamic_array::DynamicArray,
    header::{self, Header},
    identification::{Class, Data, Identification},
    note::Note,
    representation::Decoder,
    note_table::NoteTable,
    program_header::{self, ProgramHeader},
    section_header::{self, SectionHeader},
};

impl<'file> ObjectFile<'file> {
    pub fn parse(bytes: &'file [u8]) -> Result<Self, ParseError> {
        let identification_bytes: [u8; super::super::identification::SIZE] = bytes
            .get(..super::super::identification::SIZE)
            .ok_or(ParseError::Truncated)?
            .try_into()
            .map_err(|_| ParseError::Truncated)?;

        let identification = Identification::from_bytes(identification_bytes)
            .ok_or(ParseError::InvalidIdentification)?;

        if !matches!(
            identification.data,
            Data::LeastSignificantByteFirst | Data::MostSignificantByteFirst
        ) {
            return Err(ParseError::UnsupportedDataEncoding);
        }

        match identification.class {
            Class::Class32 => Self::parse_class_32(bytes, identification),
            Class::Class64 => Self::parse_class_64(bytes, identification),
            Class::None | Class::Reserved(_) => Err(ParseError::UnsupportedClass),
        }
    }

    fn parse_class_32(
        bytes: &'file [u8],
        identification: Identification,
    ) -> Result<Self, ParseError> {
        let representation = header::class_32::Representation::decode(
            bytes,
            0,
            identification.data,
        )
        .ok_or(ParseError::Truncated)?;
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
                program_header::class_32::Representation::decode(
                    bytes,
                    offset,
                    identification.data,
                )
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
                section_header::class_32::Representation::decode(
                    bytes,
                    offset,
                    identification.data,
                )
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

    fn parse_class_64(
        bytes: &'file [u8],
        identification: Identification,
    ) -> Result<Self, ParseError> {
        let representation = header::class_64::Representation::decode(
            bytes,
            0,
            identification.data,
        )
        .ok_or(ParseError::Truncated)?;
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
                program_header::class_64::Representation::decode(
                    bytes,
                    offset,
                    identification.data,
                )
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
                section_header::class_64::Representation::decode(
                    bytes,
                    offset,
                    identification.data,
                )
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
}

pub(super) fn parse_dynamic_array(
    bytes: &[u8],
    class: Class,
    data: Data,
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
                    dynamic::class_32::Representation::decode(bytes, offset, data)?;
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
                    dynamic::class_64::Representation::decode(bytes, offset, data)?;
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

pub(super) fn parse_note_table<'file>(
    bytes: &'file [u8],
    class: Class,
    data: Data,
) -> Option<NoteTable<'file>> {
    let word_size = match class {
        Class::Class32 => 4usize,
        Class::Class64 => 8usize,
        Class::None | Class::Reserved(_) => return None,
    };

    let mut notes = Vec::new();
    let mut offset = 0usize;

    while offset < bytes.len() {
        let namesz = word_by_size(bytes, offset, word_size, data)?;
        offset = offset.checked_add(word_size)?;
        let descsz = word_by_size(bytes, offset, word_size, data)?;
        offset = offset.checked_add(word_size)?;
        let r#type = word_by_size(bytes, offset, word_size, data)?;
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
    Ok(section_header::class_32::Representation::decode(
        bytes,
        offset,
        header.identification.data,
    )
    .map(SectionHeader::from))
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
    Ok(section_header::class_64::Representation::decode(
        bytes,
        offset,
        header.identification.data,
    )
    .map(SectionHeader::from))
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

fn align(value: usize, alignment: usize) -> Option<usize> {
    let mask = alignment.checked_sub(1)?;
    value.checked_add(mask).map(|value| value & !mask)
}

fn word_by_size(
    bytes: &[u8],
    offset: usize,
    word_size: usize,
    data: Data,
) -> Option<u64> {
    let mut decoder = Decoder::new(bytes, offset, data)?;
    match word_size {
        4 => decoder.word().map(u64::from),
        8 => decoder.xword(),
        _ => None,
    }
}

pub(super) fn word(bytes: &[u8], offset: usize, data: Data) -> Option<u32> {
    Decoder::new(bytes, offset, data)?.word()
}

pub(super) fn range(bytes: &[u8], offset: u64, size: u64) -> Option<&[u8]> {
    let start = usize::try_from(offset).ok()?;
    let length = usize::try_from(size).ok()?;
    let end = start.checked_add(length)?;
    bytes.get(start..end)
}

