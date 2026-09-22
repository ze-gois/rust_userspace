use super::super::representation::class_32 as representation;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Representation {
    pub ch_type: representation::Word,
    pub ch_size: representation::Word,
    pub ch_addralign: representation::Word,
}

impl From<Representation> for super::CompressionHeader {
    fn from(representation: Representation) -> Self {
        Self {
            r#type: super::Type::from_raw(representation.ch_type),
            uncompressed_size: representation.ch_size as u64,
            alignment: representation.ch_addralign as u64,
        }
    }
}
