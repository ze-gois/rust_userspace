use super::super::{identification, representation::class_32 as representation};

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
