use super::super::{
    identification::Data,
    representation::{class_32 as representation, Decoder},
};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RelRepresentation {
    pub r_offset: representation::Address,
    pub r_info: representation::Word,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RelaRepresentation {
    pub r_offset: representation::Address,
    pub r_info: representation::Word,
    pub r_addend: representation::Sword,
}

impl RelRepresentation {
    pub fn decode(bytes: &[u8], offset: usize, data: Data) -> Option<Self> {
        let mut decoder = Decoder::new(bytes, offset, data)?;
        Some(Self {
            r_offset: decoder.word()?,
            r_info: decoder.word()?,
        })
    }
}

impl RelaRepresentation {
    pub fn decode(bytes: &[u8], offset: usize, data: Data) -> Option<Self> {
        let mut decoder = Decoder::new(bytes, offset, data)?;
        Some(Self {
            r_offset: decoder.word()?,
            r_info: decoder.word()?,
            r_addend: decoder.sword()?,
        })
    }
}

impl From<RelRepresentation> for super::Relocation {
    fn from(representation: RelRepresentation) -> Self {
        Self {
            offset: representation.r_offset as u64,
            symbol_index: representation.r_info >> 8,
            r#type: super::Type::from_raw(representation.r_info & 0xff),
            addend: None,
        }
    }
}

impl From<RelaRepresentation> for super::Relocation {
    fn from(representation: RelaRepresentation) -> Self {
        Self {
            offset: representation.r_offset as u64,
            symbol_index: representation.r_info >> 8,
            r#type: super::Type::from_raw(representation.r_info & 0xff),
            addend: Some(representation.r_addend as i64),
        }
    }
}
