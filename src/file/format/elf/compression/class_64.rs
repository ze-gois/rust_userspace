use super::super::{
    identification::Data,
    representation::{class_64 as representation, Decoder},
};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Representation {
    pub ch_type: representation::Word,
    pub ch_reserved: representation::Word,
    pub ch_size: representation::Xword,
    pub ch_addralign: representation::Xword,
}

impl Representation {
    pub fn decode(bytes: &[u8], offset: usize, data: Data) -> Option<Self> {
        let mut decoder = Decoder::new(bytes, offset, data)?;
        Some(Self {
            ch_type: decoder.word()?,
            ch_reserved: decoder.word()?,
            ch_size: decoder.xword()?,
            ch_addralign: decoder.xword()?,
        })
    }
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
