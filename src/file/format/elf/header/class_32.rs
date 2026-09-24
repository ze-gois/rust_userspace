use super::super::{
    identification,
    representation::{class_32 as representation, Decoder},
};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Representation {
    pub e_ident: [u8; identification::SIZE],
    pub e_type: representation::Half,
    pub e_machine: representation::Half,
    pub e_version: representation::Word,
    pub e_entry: representation::Address,
    pub e_phoff: representation::Offset,
    pub e_shoff: representation::Offset,
    pub e_flags: representation::Word,
    pub e_ehsize: representation::Half,
    pub e_phentsize: representation::Half,
    pub e_phnum: representation::Half,
    pub e_shentsize: representation::Half,
    pub e_shnum: representation::Half,
    pub e_shstrndx: representation::Half,
}

impl Representation {
    pub fn decode(
        bytes: &[u8],
        offset: usize,
        data: identification::Data,
    ) -> Option<Self> {
        let mut decoder = Decoder::new(bytes, offset, data)?;
        Some(Self {
            e_ident: decoder.bytes()?,
            e_type: decoder.half()?,
            e_machine: decoder.half()?,
            e_version: decoder.word()?,
            e_entry: decoder.word()?,
            e_phoff: decoder.word()?,
            e_shoff: decoder.word()?,
            e_flags: decoder.word()?,
            e_ehsize: decoder.half()?,
            e_phentsize: decoder.half()?,
            e_phnum: decoder.half()?,
            e_shentsize: decoder.half()?,
            e_shnum: decoder.half()?,
            e_shstrndx: decoder.half()?,
        })
    }
}
