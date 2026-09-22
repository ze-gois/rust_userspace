//! Parsed ELF object file.

use ample::r#type::Vec;

use super::{
    header::{self, Header},
    identification::{Class, Data, Identification},
    program_header::{self, ProgramHeader},
    section_header::{self, SectionHeader},
    string_table::StringTable,
    symbol::{self, Symbol},
    symbol_table::SymbolTable,
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
    InvalidSectionNameStringTable,
}

#[derive(Debug)]
pub struct ObjectFile<'file> {
    bytes: &'file [u8],
    pub header: Header,
    pub program_headers: Vec<ProgramHeader>,
    pub section_headers: Vec<SectionHeader>,
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

        let program_headers = parse_table(
            bytes,
            header.program_header_offset,
            header.program_header_entry_size,
            header.program_header_count,
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
            header.section_header_count,
            core::mem::size_of::<section_header::class_32::Representation>(),
            |bytes, offset| {
                read::<section_header::class_32::Representation>(bytes, offset)
                    .map(SectionHeader::from)
            },
            ParseError::SectionHeaderEntryTooSmall,
        )?;

        Ok(Self {
            bytes,
            header,
            program_headers,
            section_headers,
        })
    }

    fn parse_class_64(bytes: &'file [u8]) -> Result<Self, ParseError> {
        let representation: header::class_64::Representation =
            read(bytes, 0).ok_or(ParseError::Truncated)?;
        let header = Header::try_from(representation).map_err(|_| ParseError::InvalidHeader)?;

        let program_headers = parse_table(
            bytes,
            header.program_header_offset,
            header.program_header_entry_size,
            header.program_header_count,
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
            header.section_header_count,
            core::mem::size_of::<section_header::class_64::Representation>(),
            |bytes, offset| {
                read::<section_header::class_64::Representation>(bytes, offset)
                    .map(SectionHeader::from)
            },
            ParseError::SectionHeaderEntryTooSmall,
        )?;

        Ok(Self {
            bytes,
            header,
            program_headers,
            section_headers,
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

    pub fn section_name_string_table(&self) -> Option<StringTable<'file>> {
        let index = self.header.section_name_string_table_index.raw() as usize;
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

        Some(SymbolTable::new(symbols, strings))
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
    count: u16,
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
    let mut entries = Vec::with_capacity(count as usize);

    for index in 0..count as usize {
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
