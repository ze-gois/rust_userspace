use super::super::representation::class_64 as representation;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Representation {
    pub ch_type: representation::Word,
    pub ch_reserved: representation::Word,
    pub ch_size: representation::Xword,
    pub ch_addralign: representation::Xword,
}

impl From<Representation> for super::CompressionHeader {
    fn from(representation: Representation) -> Self {
        Self {
            r#type: super::Type::from_raw(representation.ch_type),
            uncompressed_size: representation.ch_size,
            alignment: representation.ch_addralign,
        }
    }
}
