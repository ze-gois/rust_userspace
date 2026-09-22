use super::super::{
    identification::Data,
    representation::{class_32 as representation, Decoder},
};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Representation {
    pub sh_name: representation::Word,
    pub sh_type: representation::Word,
    pub sh_flags: representation::Word,
    pub sh_addr: representation::Address,
    pub sh_offset: representation::Offset,
    pub sh_size: representation::Word,
    pub sh_link: representation::Word,
    pub sh_info: representation::Word,
    pub sh_addralign: representation::Word,
    pub sh_entsize: representation::Word,
}

impl Representation {
    pub fn decode(bytes: &[u8], offset: usize, data: Data) -> Option<Self> {
        let mut decoder = Decoder::new(bytes, offset, data)?;
        Some(Self {
            sh_name: decoder.word()?,
            sh_type: decoder.word()?,
            sh_flags: decoder.word()?,
            sh_addr: decoder.word()?,
            sh_offset: decoder.word()?,
            sh_size: decoder.word()?,
            sh_link: decoder.word()?,
            sh_info: decoder.word()?,
            sh_addralign: decoder.word()?,
            sh_entsize: decoder.word()?,
        })
    }
}
