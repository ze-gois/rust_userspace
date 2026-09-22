use super::super::representation::class_64 as representation;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Representation {
    pub d_tag: representation::Sxword,
    pub d_un: representation::Xword,
}

impl From<Representation> for super::Dynamic {
    fn from(representation: Representation) -> Self {
        Self {
            tag: super::Tag::from_raw(representation.d_tag),
            payload: representation.d_un,
        }
    }
}
