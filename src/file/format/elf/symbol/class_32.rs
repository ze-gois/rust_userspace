use super::super::representation::class_32 as representation;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Representation {
    pub st_name: representation::Word,
    pub st_value: representation::Address,
    pub st_size: representation::Word,
    pub st_info: u8,
    pub st_other: u8,
    pub st_shndx: representation::Half,
}

impl From<Representation> for super::Symbol {
    fn from(representation: Representation) -> Self {
        Self {
            name_index: representation.st_name,
            value: representation.st_value as u64,
            size: representation.st_size as u64,
            binding: super::Binding::from_raw(representation.st_info >> 4),
            r#type: super::Type::from_raw(representation.st_info & 0x0f),
            visibility: super::Visibility::from_raw(representation.st_other),
            section_index: super::super::section_header::Index::from_raw(representation.st_shndx),
        }
    }
}
