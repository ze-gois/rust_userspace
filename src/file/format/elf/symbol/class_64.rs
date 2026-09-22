use super::super::{
    identification::Data,
    representation::{class_64 as representation, Decoder},
};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Representation {
    pub st_name: representation::Word,
    pub st_info: u8,
    pub st_other: u8,
    pub st_shndx: representation::Half,
    pub st_value: representation::Address,
    pub st_size: representation::Xword,
}

impl Representation {
    pub fn decode(bytes: &[u8], offset: usize, data: Data) -> Option<Self> {
        let mut decoder = Decoder::new(bytes, offset, data)?;
        Some(Self {
            st_name: decoder.word()?,
            st_info: decoder.byte()?,
            st_other: decoder.byte()?,
            st_shndx: decoder.half()?,
            st_value: decoder.xword()?,
            st_size: decoder.xword()?,
        })
    }
}

impl From<Representation> for super::Symbol {
    fn from(representation: Representation) -> Self {
        Self {
            name_index: representation.st_name,
            value: representation.st_value,
            size: representation.st_size,
            binding: super::Binding::from_raw(representation.st_info >> 4),
            r#type: super::Type::from_raw(representation.st_info & 0x0f),
            visibility: super::Visibility::from_raw(representation.st_other),
            section_index: super::super::section_header::Index::from_raw(representation.st_shndx),
        }
    }
}
