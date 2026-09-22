use super::super::{
    identification::Data,
    representation::{class_32 as representation, Decoder},
};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Representation {
    pub p_type: representation::Word,
    pub p_offset: representation::Offset,
    pub p_vaddr: representation::Address,
    pub p_paddr: representation::Address,
    pub p_filesz: representation::Word,
    pub p_memsz: representation::Word,
    pub p_flags: representation::Word,
    pub p_align: representation::Word,
}

impl Representation {
    pub fn decode(bytes: &[u8], offset: usize, data: Data) -> Option<Self> {
        let mut decoder = Decoder::new(bytes, offset, data)?;
        Some(Self {
            p_type: decoder.word()?,
            p_offset: decoder.word()?,
            p_vaddr: decoder.word()?,
            p_paddr: decoder.word()?,
            p_filesz: decoder.word()?,
            p_memsz: decoder.word()?,
            p_flags: decoder.word()?,
            p_align: decoder.word()?,
        })
    }
}
