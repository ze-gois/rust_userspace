use super::super::{
    identification::Data,
    representation::{class_64 as representation, Decoder},
};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Representation {
    pub d_tag: representation::Sxword,
    pub d_un: representation::Xword,
}

impl Representation {
    pub fn decode(bytes: &[u8], offset: usize, data: Data) -> Option<Self> {
        let mut decoder = Decoder::new(bytes, offset, data)?;
        Some(Self {
            d_tag: decoder.sxword()?,
            d_un: decoder.xword()?,
        })
    }
}

impl From<Representation> for super::Dynamic {
    fn from(representation: Representation) -> Self {
        Self {
            tag: super::Tag::from_raw(representation.d_tag),
            payload: representation.d_un,
        }
    }
}
