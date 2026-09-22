use super::super::representation::class_32 as representation;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Representation {
    pub d_tag: representation::Sword,
    pub d_un: representation::Word,
}

impl From<Representation> for super::Dynamic {
    fn from(representation: Representation) -> Self {
        Self {
            tag: super::Tag::from_raw(representation.d_tag as i64),
            payload: representation.d_un as u64,
        }
    }
}
