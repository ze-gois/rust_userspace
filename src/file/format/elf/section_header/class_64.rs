use super::super::{
    identification::Data,
    representation::{class_64 as representation, Decoder},
};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Representation {
    pub sh_name: representation::Word,
    pub sh_type: representation::Word,
    pub sh_flags: representation::Xword,
    pub sh_addr: representation::Address,
    pub sh_offset: representation::Offset,
    pub sh_size: representation::Xword,
    pub sh_link: representation::Word,
    pub sh_info: representation::Word,
    pub sh_addralign: representation::Xword,
    pub sh_entsize: representation::Xword,
}

impl Representation {
    pub fn decode(bytes: &[u8], offset: usize, data: Data) -> Option<Self> {
        let mut decoder = Decoder::new(bytes, offset, data)?;
        Some(Self {
            sh_name: decoder.word()?,
            sh_type: decoder.word()?,
            sh_flags: decoder.xword()?,
            sh_addr: decoder.xword()?,
            sh_offset: decoder.xword()?,
            sh_size: decoder.xword()?,
            sh_link: decoder.word()?,
            sh_info: decoder.word()?,
            sh_addralign: decoder.xword()?,
            sh_entsize: decoder.xword()?,
        })
    }
}
