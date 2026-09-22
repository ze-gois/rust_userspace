use super::super::representation::class_64 as representation;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RelRepresentation {
    pub r_offset: representation::Address,
    pub r_info: representation::Xword,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RelaRepresentation {
    pub r_offset: representation::Address,
    pub r_info: representation::Xword,
    pub r_addend: representation::Sxword,
}

impl From<RelRepresentation> for super::Relocation {
    fn from(representation: RelRepresentation) -> Self {
        Self {
            offset: representation.r_offset,
            symbol_index: (representation.r_info >> 32) as u32,
            r#type: super::Type::from_raw(representation.r_info as u32),
            addend: None,
        }
    }
}

impl From<RelaRepresentation> for super::Relocation {
    fn from(representation: RelaRepresentation) -> Self {
        Self {
            offset: representation.r_offset,
            symbol_index: (representation.r_info >> 32) as u32,
            r#type: super::Type::from_raw(representation.r_info as u32),
            addend: Some(representation.r_addend),
        }
    }
}
