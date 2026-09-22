use super::super::{
    identification::Data,
    representation::{class_64 as representation, Decoder},
};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Representation {
    pub p_type: representation::Word,
    pub p_flags: representation::Word,
    pub p_offset: representation::Offset,
    pub p_vaddr: representation::Address,
    pub p_paddr: representation::Address,
    pub p_filesz: representation::Xword,
    pub p_memsz: representation::Xword,
    pub p_align: representation::Xword,
}

impl Representation {
    pub fn decode(bytes: &[u8], offset: usize, data: Data) -> Option<Self> {
        let mut decoder = Decoder::new(bytes, offset, data)?;
        Some(Self {
            p_type: decoder.word()?,
            p_flags: decoder.word()?,
            p_offset: decoder.xword()?,
            p_vaddr: decoder.xword()?,
            p_paddr: decoder.xword()?,
            p_filesz: decoder.xword()?,
            p_memsz: decoder.xword()?,
            p_align: decoder.xword()?,
        })
    }
}
