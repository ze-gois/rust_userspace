//! Parsed ELF object file.

use ample::r#type::Vec;

use super::{
    compression::{self, CompressedSection, CompressionHeader},
    dynamic::{self, Dynamic, Tag},
    dynamic_table::DynamicTable,
    hash::HashTable,
    header::{self, Header},
    identification::{Class, Data, Identification},
    program_header::{self, ProgramHeader},
    relocation::{self, Relocation},
    relocation_table::RelocationTable,
    section_group::{Flags as SectionGroupFlags, SectionGroup},
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

        Some(SymbolTable::new(symbols, strings, header.information as usize))
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

        let count = section.contents.len() / entry_size;
        let mut entries = Vec::with_capacity(count);

        match self.header.identification.class {
            Class::Class32 => {
                if entry_size < core::mem::size_of::<dynamic::class_32::Representation>() {
                    return None;
                }
                for index in 0..count {
                    let offset = index.checked_mul(entry_size)?;
                    let representation =
                        read::<dynamic::class_32::Representation>(section.contents, offset)?;
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
                        read::<dynamic::class_64::Representation>(section.contents, offset)?;
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

        Some(DynamicTable::new(
            entries,
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
