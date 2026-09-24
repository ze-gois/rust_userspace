use super::super::{
    identification::Data,
    representation::{class_32 as representation, Decoder},
};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Representation {
    pub d_tag: representation::Sword,
    pub d_un: representation::Word,
}

impl Representation {
    pub fn decode(bytes: &[u8], offset: usize, data: Data) -> Option<Self> {
        let mut decoder = Decoder::new(bytes, offset, data)?;
        Some(Self {
            d_tag: decoder.sword()?,
            d_un: decoder.word()?,
        })
    }
}

impl From<Representation> for super::Dynamic {
    fn from(representation: Representation) -> Self {
        Self {
            tag: super::Tag::from_raw(representation.d_tag as i64),
            payload: representation.d_un as u64,
        }
    }
}
